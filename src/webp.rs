// imgconv - WebP 인코딩/디코딩

use std::fs;
use std::path::Path;

use image::{DynamicImage, RgbaImage};

use crate::error::ConvertError;

/// WebP 인코딩 모드
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebPMode {
    /// Lossy 인코딩 (기본값)
    Lossy,
    /// Lossless 인코딩
    Lossless,
}

impl Default for WebPMode {
    fn default() -> Self {
        Self::Lossy
    }
}

/// WebP 파일을 디코딩하여 `DynamicImage`로 반환한다.
///
/// zenwebp의 `decode_rgba`를 사용하여 RGBA 픽셀 데이터로 디코딩한 후
/// `DynamicImage`로 변환한다.
pub fn decode_webp(path: &Path) -> Result<DynamicImage, ConvertError> {
    let data = fs::read(path)?;
    let (pixels, width, height) =
        zenwebp::decode_rgba(&data).map_err(|e| ConvertError::DecodingError(e.to_string()))?;

    let rgba_image = RgbaImage::from_raw(width, height, pixels).ok_or_else(|| {
        ConvertError::DecodingError("WebP 디코딩된 픽셀 데이터로 이미지를 생성할 수 없습니다".into())
    })?;

    Ok(DynamicImage::ImageRgba8(rgba_image))
}

/// `DynamicImage`를 WebP 포맷으로 인코딩하여 파일로 저장한다.
///
/// - `mode`: Lossy 또는 Lossless 인코딩 모드
/// - `quality`: Lossy 모드에서의 품질 (1-100, 기본값 75). Lossless 모드에서는 무시됨.
pub fn encode_webp(
    img: &DynamicImage,
    path: &Path,
    mode: WebPMode,
    quality: Option<u8>,
) -> Result<(), ConvertError> {
    let rgba = img.to_rgba8();
    let (width, height) = (rgba.width(), rgba.height());
    let raw = rgba.as_raw();

    let encoder = zenwebp::Encoder::new_rgba(raw, width, height);

    let webp_data = match mode {
        WebPMode::Lossy => {
            let q = quality.unwrap_or(75) as f32;
            encoder
                .quality(q)
                .encode()
                .map_err(|e| ConvertError::EncodingError(e.to_string()))?
        }
        WebPMode::Lossless => encoder
            .lossless(true)
            .encode()
            .map_err(|e| ConvertError::EncodingError(e.to_string()))?,
    };

    fs::write(path, &*webp_data)?;

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
        std::env::temp_dir().join(format!("imgconv_webp_test_{name}"))
    }

    /// Lossy 인코딩 왕복 테스트: 이미지 생성 → lossy WebP 인코딩 → 디코딩 → 치수 확인
    /// Validates: Requirements 2.1, 2.3, 2.5
    #[test]
    fn lossy_encode_decode_round_trip() {
        let img = create_test_image(32, 24);
        let path = temp_path("lossy_round_trip.webp");

        encode_webp(&img, &path, WebPMode::Lossy, Some(80)).unwrap();
        let decoded = decode_webp(&path).unwrap();

        assert_eq!(decoded.width(), 32);
        assert_eq!(decoded.height(), 24);

        std::fs::remove_file(&path).ok();
    }

    /// Lossless 인코딩 왕복 테스트: 이미지 생성 → lossless WebP 인코딩 → 디코딩 → 치수 정확히 일치 확인
    /// Validates: Requirements 2.1, 2.4
    #[test]
    fn lossless_encode_decode_round_trip() {
        let img = create_test_image(16, 16);
        let path = temp_path("lossless_round_trip.webp");

        encode_webp(&img, &path, WebPMode::Lossless, None).unwrap();
        let decoded = decode_webp(&path).unwrap();

        assert_eq!(decoded.width(), 16);
        assert_eq!(decoded.height(), 16);

        std::fs::remove_file(&path).ok();
    }

    /// Lossy 모드에서 quality=None일 때 기본값 75가 적용되어 정상 인코딩되는지 확인
    /// Validates: Requirements 2.5
    #[test]
    fn lossy_default_quality_applied() {
        let img = create_test_image(16, 16);
        let path = temp_path("lossy_default_q.webp");

        // quality=None → 기본값 75 적용
        encode_webp(&img, &path, WebPMode::Lossy, None).unwrap();
        let decoded = decode_webp(&path).unwrap();

        assert_eq!(decoded.width(), 16);
        assert_eq!(decoded.height(), 16);
        // 파일이 생성되었으므로 기본 품질로 인코딩 성공
        let file_size = std::fs::metadata(&path).unwrap().len();
        assert!(file_size > 0);

        std::fs::remove_file(&path).ok();
    }

    /// Lossy 모드에서 커스텀 품질 파라미터가 적용되는지 확인 (낮은 품질 → 작은 파일)
    /// Validates: Requirements 2.3, 2.5
    #[test]
    fn lossy_custom_quality_affects_output() {
        let img = create_test_image(64, 64);
        let path_low = temp_path("lossy_q10.webp");
        let path_high = temp_path("lossy_q95.webp");

        encode_webp(&img, &path_low, WebPMode::Lossy, Some(10)).unwrap();
        encode_webp(&img, &path_high, WebPMode::Lossy, Some(95)).unwrap();

        let size_low = std::fs::metadata(&path_low).unwrap().len();
        let size_high = std::fs::metadata(&path_high).unwrap().len();

        // 낮은 품질의 파일이 높은 품질보다 작아야 한다
        assert!(
            size_low < size_high,
            "low quality ({size_low}) should be smaller than high quality ({size_high})"
        );

        std::fs::remove_file(&path_low).ok();
        std::fs::remove_file(&path_high).ok();
    }

    /// WebP 디코딩 후 재인코딩 왕복 테스트
    /// Validates: Requirements 2.1, 2.2
    #[test]
    fn decode_then_reencode_round_trip() {
        let img = create_test_image(24, 24);
        let path1 = temp_path("reencode_step1.webp");
        let path2 = temp_path("reencode_step2.webp");

        // 1차: 원본 → WebP
        encode_webp(&img, &path1, WebPMode::Lossless, None).unwrap();
        // 2차: WebP 디코딩 → 재인코딩
        let decoded = decode_webp(&path1).unwrap();
        encode_webp(&decoded, &path2, WebPMode::Lossy, Some(80)).unwrap();

        let final_decoded = decode_webp(&path2).unwrap();
        assert_eq!(final_decoded.width(), 24);
        assert_eq!(final_decoded.height(), 24);

        std::fs::remove_file(&path1).ok();
        std::fs::remove_file(&path2).ok();
    }
}
