// imgconv - AVIF 인코딩/디코딩 (feature-gated)
//
// 이 모듈은 lib.rs에서 `#[cfg(feature = "avif")]`로 조건부 컴파일됩니다.
// 따라서 모듈 내부 아이템에는 별도의 cfg 속성이 필요하지 않습니다.

use std::fs;
use std::io::Cursor;
use std::path::Path;

use image::{DynamicImage, RgbaImage};

use crate::error::ConvertError;

// rav1d C-compatible API functions
use rav1d::src::lib::{
    dav1d_close, dav1d_data_create, dav1d_default_settings, dav1d_get_picture, dav1d_open,
    dav1d_picture_unref, dav1d_send_data,
};

// rav1d types
use rav1d::include::dav1d::data::Dav1dData;
use rav1d::include::dav1d::dav1d::{Dav1dContext, Dav1dSettings};
use rav1d::include::dav1d::headers::{
    DAV1D_PIXEL_LAYOUT_I400, DAV1D_PIXEL_LAYOUT_I420, DAV1D_PIXEL_LAYOUT_I422,
    DAV1D_PIXEL_LAYOUT_I444,
};
use rav1d::include::dav1d::picture::Dav1dPicture;

/// AVIF 파일을 디코딩하여 `DynamicImage`로 반환한다.
///
/// avif-parse로 AVIF 컨테이너를 파싱하고, rav1d로 AV1 비트스트림을 디코딩한다.
pub fn decode_avif(path: &Path) -> Result<DynamicImage, ConvertError> {
    let data = fs::read(path)?;
    let mut cursor = Cursor::new(&data);

    // avif-parse로 AVIF 컨테이너 파싱
    let avif = avif_parse::read_avif(&mut cursor)
        .map_err(|e| ConvertError::DecodingError(format!("AVIF 파싱 실패: {e}")))?;

    // rav1d로 AV1 비트스트림 디코딩 (color)
    let color_rgba = decode_av1_to_rgba(&avif.primary_item)?;

    let width = color_rgba.1;
    let height = color_rgba.2;
    let mut pixels = color_rgba.0;

    // 알파 채널 처리
    if let Some(ref alpha_data) = avif.alpha_item {
        let alpha_gray = decode_av1_to_gray(alpha_data)?;
        for (i, alpha) in alpha_gray.0.iter().enumerate() {
            let idx = i * 4 + 3;
            if idx < pixels.len() {
                pixels[idx] = *alpha;
            }
        }
    }

    let rgba_image = RgbaImage::from_raw(width, height, pixels).ok_or_else(|| {
        ConvertError::DecodingError(
            "AVIF 디코딩된 픽셀 데이터로 이미지를 생성할 수 없습니다".into(),
        )
    })?;

    Ok(DynamicImage::ImageRgba8(rgba_image))
}

/// rav1d를 사용하여 AV1 데이터를 RGBA 픽셀로 디코딩한다.
/// 반환: (rgba_pixels, width, height)
fn decode_av1_to_rgba(av1_data: &[u8]) -> Result<(Vec<u8>, u32, u32), ConvertError> {
    unsafe {
        // 디코더 초기화
        let mut settings: Dav1dSettings = std::mem::zeroed();
        dav1d_default_settings(std::ptr::NonNull::new(&mut settings).unwrap());
        settings.n_threads = 0; // auto-detect

        let mut ctx: Option<Dav1dContext> = None;
        let result =
            dav1d_open(std::ptr::NonNull::new(&mut ctx), std::ptr::NonNull::new(&mut settings));
        if result.0 != 0 {
            return Err(ConvertError::DecodingError(
                "rav1d 디코더 초기화 실패".into(),
            ));
        }
        let ctx_val = ctx.ok_or_else(|| {
            ConvertError::DecodingError("rav1d 컨텍스트 생성 실패".into())
        })?;

        // 데이터 전송
        let mut dav1d_data = Dav1dData::default();
        let buf_ptr: *mut u8 =
            dav1d_data_create(std::ptr::NonNull::new(&mut dav1d_data), av1_data.len());
        if buf_ptr.is_null() {
            dav1d_close(std::ptr::NonNull::new(&mut ctx));
            return Err(ConvertError::DecodingError(
                "rav1d 데이터 버퍼 할당 실패".into(),
            ));
        }
        std::ptr::copy_nonoverlapping(av1_data.as_ptr(), buf_ptr, av1_data.len());

        let _send_result = dav1d_send_data(
            Some(ctx_val),
            std::ptr::NonNull::new(&mut dav1d_data),
        );

        // 디코딩된 프레임 가져오기
        let mut pic: Dav1dPicture = std::mem::zeroed();
        let get_result =
            dav1d_get_picture(Some(ctx_val), std::ptr::NonNull::new(&mut pic));
        if get_result.0 != 0 {
            // 재시도
            let get_result2 =
                dav1d_get_picture(Some(ctx_val), std::ptr::NonNull::new(&mut pic));
            if get_result2.0 != 0 {
                dav1d_close(std::ptr::NonNull::new(&mut ctx));
                return Err(ConvertError::DecodingError(format!(
                    "rav1d 프레임 디코딩 실패: error code {}",
                    get_result.0
                )));
            }
        }

        let w = pic.p.w as u32;
        let h = pic.p.h as u32;
        let bpc = pic.p.bpc;
        let layout = pic.p.layout;

        // YUV → RGBA 변환
        let rgba = yuv_picture_to_rgba(&pic, w, h, bpc, layout)?;

        dav1d_picture_unref(std::ptr::NonNull::new(&mut pic));
        dav1d_close(std::ptr::NonNull::new(&mut ctx));

        Ok((rgba, w, h))
    }
}

/// rav1d를 사용하여 AV1 데이터를 그레이스케일 픽셀로 디코딩한다 (알파 채널용).
/// 반환: (gray_pixels, width, height)
fn decode_av1_to_gray(av1_data: &[u8]) -> Result<(Vec<u8>, u32, u32), ConvertError> {
    unsafe {
        let mut settings: Dav1dSettings = std::mem::zeroed();
        dav1d_default_settings(std::ptr::NonNull::new(&mut settings).unwrap());
        settings.n_threads = 0;

        let mut ctx: Option<Dav1dContext> = None;
        let result =
            dav1d_open(std::ptr::NonNull::new(&mut ctx), std::ptr::NonNull::new(&mut settings));
        if result.0 != 0 {
            return Err(ConvertError::DecodingError(
                "rav1d 알파 디코더 초기화 실패".into(),
            ));
        }
        let ctx_val = ctx.ok_or_else(|| {
            ConvertError::DecodingError("rav1d 알파 컨텍스트 생성 실패".into())
        })?;

        let mut dav1d_data = Dav1dData::default();
        let buf_ptr: *mut u8 =
            dav1d_data_create(std::ptr::NonNull::new(&mut dav1d_data), av1_data.len());
        if buf_ptr.is_null() {
            dav1d_close(std::ptr::NonNull::new(&mut ctx));
            return Err(ConvertError::DecodingError(
                "rav1d 알파 데이터 버퍼 할당 실패".into(),
            ));
        }
        std::ptr::copy_nonoverlapping(av1_data.as_ptr(), buf_ptr, av1_data.len());

        let _ = dav1d_send_data(
            Some(ctx_val),
            std::ptr::NonNull::new(&mut dav1d_data),
        );

        let mut pic: Dav1dPicture = std::mem::zeroed();
        let get_result =
            dav1d_get_picture(Some(ctx_val), std::ptr::NonNull::new(&mut pic));
        if get_result.0 != 0 {
            let get_result2 =
                dav1d_get_picture(Some(ctx_val), std::ptr::NonNull::new(&mut pic));
            if get_result2.0 != 0 {
                dav1d_close(std::ptr::NonNull::new(&mut ctx));
                return Err(ConvertError::DecodingError(
                    "rav1d 알파 프레임 디코딩 실패".into(),
                ));
            }
        }

        let w = pic.p.w as u32;
        let h = pic.p.h as u32;
        let bpc = pic.p.bpc;

        let gray = extract_y_plane(&pic, w, h, bpc)?;

        dav1d_picture_unref(std::ptr::NonNull::new(&mut pic));
        dav1d_close(std::ptr::NonNull::new(&mut ctx));

        Ok((gray, w, h))
    }
}


/// Dav1dPicture에서 YUV 데이터를 RGBA로 변환한다.
unsafe fn yuv_picture_to_rgba(
    pic: &Dav1dPicture,
    width: u32,
    height: u32,
    bpc: std::ffi::c_int,
    layout: rav1d::include::dav1d::headers::Dav1dPixelLayout,
) -> Result<Vec<u8>, ConvertError> {
    let y_ptr = pic.data[0]
        .ok_or_else(|| ConvertError::DecodingError("Y 평면 데이터 없음".into()))?;
    let u_ptr = pic.data[1];
    let v_ptr = pic.data[2];

    let y_stride = pic.stride[0];
    let uv_stride = pic.stride[1];

    let w = width as usize;
    let h = height as usize;
    let mut rgba = vec![0u8; w * h * 4];

    // chroma subsampling 결정
    let (ss_hor, ss_ver) = match layout {
        DAV1D_PIXEL_LAYOUT_I420 => (1usize, 1usize), // 4:2:0
        DAV1D_PIXEL_LAYOUT_I422 => (1, 0),            // 4:2:2
        DAV1D_PIXEL_LAYOUT_I444 => (0, 0),            // 4:4:4
        DAV1D_PIXEL_LAYOUT_I400 | _ => (0, 0),        // monochrome
    };

    if bpc <= 8 {
        let y_data = y_ptr.as_ptr() as *const u8;

        if layout != DAV1D_PIXEL_LAYOUT_I400 {
            if let (Some(u_p), Some(v_p)) = (u_ptr, v_ptr) {
                let u_data = u_p.as_ptr() as *const u8;
                let v_data = v_p.as_ptr() as *const u8;

                for row in 0..h {
                    for col in 0..w {
                        let y_off = row as isize * y_stride + col as isize;
                        let uv_row = row >> ss_ver;
                        let uv_col = col >> ss_hor;
                        let uv_off = uv_row as isize * uv_stride + uv_col as isize;

                        let y = *y_data.offset(y_off) as f32;
                        let u = *u_data.offset(uv_off) as f32 - 128.0;
                        let v = *v_data.offset(uv_off) as f32 - 128.0;

                        // BT.709 YUV → RGB 변환
                        let r = (y + 1.5748 * v).clamp(0.0, 255.0) as u8;
                        let g = (y - 0.1873 * u - 0.4681 * v).clamp(0.0, 255.0) as u8;
                        let b = (y + 1.8556 * u).clamp(0.0, 255.0) as u8;

                        let idx = (row * w + col) * 4;
                        rgba[idx] = r;
                        rgba[idx + 1] = g;
                        rgba[idx + 2] = b;
                        rgba[idx + 3] = 255;
                    }
                }
            } else {
                // fallback: Y만 사용
                fill_monochrome_8bit(y_data, y_stride, w, h, &mut rgba);
            }
        } else {
            // 모노크롬
            fill_monochrome_8bit(y_data, y_stride, w, h, &mut rgba);
        }
    } else {
        // 10-bit or 12-bit
        let shift = bpc - 8;
        let y_data = y_ptr.as_ptr() as *const u16;
        let y_stride_px = y_stride / 2;

        if layout != DAV1D_PIXEL_LAYOUT_I400 {
            if let (Some(u_p), Some(v_p)) = (u_ptr, v_ptr) {
                let u_data = u_p.as_ptr() as *const u16;
                let v_data = v_p.as_ptr() as *const u16;
                let uv_stride_px = uv_stride / 2;

                for row in 0..h {
                    for col in 0..w {
                        let y_off = row as isize * y_stride_px + col as isize;
                        let uv_row = row >> ss_ver;
                        let uv_col = col >> ss_hor;
                        let uv_off = uv_row as isize * uv_stride_px + uv_col as isize;

                        let y = (*y_data.offset(y_off) >> shift) as f32;
                        let u = (*u_data.offset(uv_off) >> shift) as f32 - 128.0;
                        let v = (*v_data.offset(uv_off) >> shift) as f32 - 128.0;

                        let r = (y + 1.5748 * v).clamp(0.0, 255.0) as u8;
                        let g = (y - 0.1873 * u - 0.4681 * v).clamp(0.0, 255.0) as u8;
                        let b = (y + 1.8556 * u).clamp(0.0, 255.0) as u8;

                        let idx = (row * w + col) * 4;
                        rgba[idx] = r;
                        rgba[idx + 1] = g;
                        rgba[idx + 2] = b;
                        rgba[idx + 3] = 255;
                    }
                }
            } else {
                fill_monochrome_16bit(y_ptr.as_ptr() as *const u16, y_stride / 2, shift, w, h, &mut rgba);
            }
        } else {
            fill_monochrome_16bit(y_ptr.as_ptr() as *const u16, y_stride / 2, shift, w, h, &mut rgba);
        }
    }

    Ok(rgba)
}

unsafe fn fill_monochrome_8bit(
    y_data: *const u8,
    y_stride: isize,
    w: usize,
    h: usize,
    rgba: &mut [u8],
) {
    for row in 0..h {
        for col in 0..w {
            let y_off = row as isize * y_stride + col as isize;
            let y = *y_data.offset(y_off);
            let idx = (row * w + col) * 4;
            rgba[idx] = y;
            rgba[idx + 1] = y;
            rgba[idx + 2] = y;
            rgba[idx + 3] = 255;
        }
    }
}

unsafe fn fill_monochrome_16bit(
    y_data: *const u16,
    y_stride_px: isize,
    shift: std::ffi::c_int,
    w: usize,
    h: usize,
    rgba: &mut [u8],
) {
    for row in 0..h {
        for col in 0..w {
            let y_off = row as isize * y_stride_px + col as isize;
            let y = (*y_data.offset(y_off) >> shift) as u8;
            let idx = (row * w + col) * 4;
            rgba[idx] = y;
            rgba[idx + 1] = y;
            rgba[idx + 2] = y;
            rgba[idx + 3] = 255;
        }
    }
}

/// Dav1dPicture에서 Y 평면만 추출한다 (알파 채널용).
unsafe fn extract_y_plane(
    pic: &Dav1dPicture,
    width: u32,
    height: u32,
    bpc: std::ffi::c_int,
) -> Result<Vec<u8>, ConvertError> {
    let y_ptr = pic.data[0]
        .ok_or_else(|| ConvertError::DecodingError("알파 Y 평면 데이터 없음".into()))?;

    let y_stride = pic.stride[0];
    let w = width as usize;
    let h = height as usize;
    let mut gray = vec![0u8; w * h];

    if bpc <= 8 {
        let y_data = y_ptr.as_ptr() as *const u8;
        for row in 0..h {
            for col in 0..w {
                let y_off = row as isize * y_stride + col as isize;
                gray[row * w + col] = *y_data.offset(y_off);
            }
        }
    } else {
        let shift = bpc - 8;
        let y_data = y_ptr.as_ptr() as *const u16;
        let y_stride_px = y_stride / 2;
        for row in 0..h {
            for col in 0..w {
                let y_off = row as isize * y_stride_px + col as isize;
                gray[row * w + col] = (*y_data.offset(y_off) >> shift) as u8;
            }
        }
    }

    Ok(gray)
}

/// `DynamicImage`를 AVIF 포맷으로 인코딩하여 파일로 저장한다.
///
/// ravif 크레이트를 사용하여 AVIF 인코딩을 수행한다.
/// - `quality`: 품질 (1-100, 기본값 70)
pub fn encode_avif(
    img: &DynamicImage,
    path: &Path,
    quality: Option<u8>,
) -> Result<(), ConvertError> {
    let rgba = img.to_rgba8();
    let width = rgba.width() as usize;
    let height = rgba.height() as usize;
    let raw = rgba.into_raw();

    // u8 데이터를 RGBA 픽셀로 변환
    // ravif는 &[RGBA8] 슬라이스를 기대하므로 안전하게 변환
    assert!(raw.len() == width * height * 4);
    let pixels: &[ravif::RGBA8] =
        unsafe { std::slice::from_raw_parts(raw.as_ptr() as *const ravif::RGBA8, width * height) };

    let q = quality.unwrap_or(70) as f32;

    let res = ravif::Encoder::new()
        .with_quality(q)
        .with_speed(6)
        .encode_rgba(ravif::Img::new(pixels, width, height))
        .map_err(|e| ConvertError::EncodingError(format!("AVIF 인코딩 실패: {e}")))?;

    fs::write(path, res.avif_file)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, RgbaImage};
    use std::path::PathBuf;

    fn create_test_image(w: u32, h: u32) -> DynamicImage {
        let mut img = RgbaImage::new(w, h);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            *pixel = image::Rgba([
                (x % 256) as u8,
                (y % 256) as u8,
                ((x + y) % 256) as u8,
                255,
            ]);
        }
        DynamicImage::ImageRgba8(img)
    }

    fn temp_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("imgconv_avif_test_{name}"))
    }

    /// AVIF 인코딩/디코딩 왕복 테스트: 이미지 생성 → AVIF 인코딩 → 디코딩 → 치수 확인
    /// Validates: Requirements 4.1, 4.2
    #[test]
    fn avif_encode_decode_round_trip() {
        let img = create_test_image(32, 24);
        let path = temp_path("round_trip.avif");

        encode_avif(&img, &path, Some(80)).unwrap();
        let decoded = decode_avif(&path).unwrap();

        assert_eq!(decoded.width(), 32);
        assert_eq!(decoded.height(), 24);

        std::fs::remove_file(&path).ok();
    }

    /// AVIF 커스텀 품질 파라미터 인코딩 테스트
    /// Validates: Requirements 4.1
    #[test]
    fn avif_encode_with_custom_quality() {
        let img = create_test_image(16, 16);
        let path = temp_path("custom_quality.avif");

        encode_avif(&img, &path, Some(50)).unwrap();
        let decoded = decode_avif(&path).unwrap();

        assert_eq!(decoded.width(), 16);
        assert_eq!(decoded.height(), 16);

        let file_size = std::fs::metadata(&path).unwrap().len();
        assert!(file_size > 0);

        std::fs::remove_file(&path).ok();
    }

    /// AVIF 기본 품질(70) 인코딩 테스트: quality=None일 때 기본값 70 적용
    /// Validates: Requirements 4.1
    #[test]
    fn avif_encode_with_default_quality() {
        let img = create_test_image(16, 16);
        let path = temp_path("default_quality.avif");

        // quality=None → 기본값 70 적용
        encode_avif(&img, &path, None).unwrap();
        let decoded = decode_avif(&path).unwrap();

        assert_eq!(decoded.width(), 16);
        assert_eq!(decoded.height(), 16);

        let file_size = std::fs::metadata(&path).unwrap().len();
        assert!(file_size > 0);

        std::fs::remove_file(&path).ok();
    }
}
