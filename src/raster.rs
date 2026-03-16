// imgconv - 래스터 포맷 인코딩/디코딩

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use image::codecs::jpeg::JpegEncoder;
use image::DynamicImage;

use crate::error::ConvertError;
use crate::format::ImageFormat;

/// `crate::format::ImageFormat`을 `image::ImageFormat`으로 매핑한다.
///
/// 12개 래스터 포맷만 매핑하며, WebP/SVG/AVIF는 별도 모듈에서 처리한다.
pub fn to_image_format(format: ImageFormat) -> Result<image::ImageFormat, ConvertError> {
    match format {
        ImageFormat::Jpeg => Ok(image::ImageFormat::Jpeg),
        ImageFormat::Png => Ok(image::ImageFormat::Png),
        ImageFormat::Gif => Ok(image::ImageFormat::Gif),
        ImageFormat::Bmp => Ok(image::ImageFormat::Bmp),
        ImageFormat::Tiff => Ok(image::ImageFormat::Tiff),
        ImageFormat::Tga => Ok(image::ImageFormat::Tga),
        ImageFormat::Ico => Ok(image::ImageFormat::Ico),
        ImageFormat::Qoi => Ok(image::ImageFormat::Qoi),
        ImageFormat::Pnm => Ok(image::ImageFormat::Pnm),
        ImageFormat::OpenExr => Ok(image::ImageFormat::OpenExr),
        ImageFormat::Hdr => Ok(image::ImageFormat::Hdr),
        ImageFormat::Farbfeld => Ok(image::ImageFormat::Farbfeld),
        _ => Err(ConvertError::EncodingError(format!(
            "{format} 포맷은 래스터 코덱에서 지원하지 않습니다"
        ))),
    }
}

/// 래스터 이미지를 디코딩한다.
///
/// `image` 크레이트의 `open`을 사용하여 파일을 읽고 `DynamicImage`로 반환한다.
pub fn decode_raster(path: &Path) -> Result<DynamicImage, ConvertError> {
    image::open(path).map_err(|e| ConvertError::DecodingError(e.to_string()))
}

/// 래스터 이미지를 인코딩하여 파일로 저장한다.
///
/// JPEG 포맷의 경우 `quality` 파라미터를 적용한다.
/// 다른 포맷은 `image` 크레이트의 기본 인코더를 사용한다.
pub fn encode_raster(
    img: &DynamicImage,
    format: ImageFormat,
    path: &Path,
    quality: Option<u8>,
) -> Result<(), ConvertError> {
    let img_format = to_image_format(format)?;

    if format == ImageFormat::Jpeg {
        let q = quality.unwrap_or(85);
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        let encoder = JpegEncoder::new_with_quality(writer, q);
        img.write_with_encoder(encoder)
            .map_err(|e| ConvertError::EncodingError(e.to_string()))?;
    } else if format == ImageFormat::Farbfeld {
        // Farbfeld는 Rgba16만 지원하므로 변환 필요
        let img16 = DynamicImage::ImageRgba16(img.to_rgba16());
        img16
            .save_with_format(path, img_format)
            .map_err(|e| ConvertError::EncodingError(e.to_string()))?;
    } else {
        img.save_with_format(path, img_format)
            .map_err(|e| ConvertError::EncodingError(e.to_string()))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, RgbaImage};
    use std::path::PathBuf;

    fn create_test_image(w: u32, h: u32) -> DynamicImage {
        let mut img = RgbaImage::new(w, h);
        // Fill with a non-trivial pattern so encoding is meaningful
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
        std::env::temp_dir().join(format!("imgconv_test_{name}"))
    }

    /// to_image_format이 12개 래스터 포맷을 올바르게 매핑하는지 확인
    /// Validates: Requirements 1.2
    #[test]
    fn to_image_format_maps_all_raster_formats() {
        let mappings: &[(ImageFormat, image::ImageFormat)] = &[
            (ImageFormat::Jpeg, image::ImageFormat::Jpeg),
            (ImageFormat::Png, image::ImageFormat::Png),
            (ImageFormat::Gif, image::ImageFormat::Gif),
            (ImageFormat::Bmp, image::ImageFormat::Bmp),
            (ImageFormat::Tiff, image::ImageFormat::Tiff),
            (ImageFormat::Tga, image::ImageFormat::Tga),
            (ImageFormat::Ico, image::ImageFormat::Ico),
            (ImageFormat::Qoi, image::ImageFormat::Qoi),
            (ImageFormat::Pnm, image::ImageFormat::Pnm),
            (ImageFormat::OpenExr, image::ImageFormat::OpenExr),
            (ImageFormat::Hdr, image::ImageFormat::Hdr),
            (ImageFormat::Farbfeld, image::ImageFormat::Farbfeld),
        ];

        for (our_fmt, expected_img_fmt) in mappings {
            let result = to_image_format(*our_fmt).unwrap();
            assert_eq!(result, *expected_img_fmt, "mismatch for {our_fmt:?}");
        }
    }

    /// WebP, SVG 등 비래스터 포맷은 에러를 반환하는지 확인
    #[test]
    fn to_image_format_rejects_non_raster() {
        assert!(to_image_format(ImageFormat::WebP).is_err());
        assert!(to_image_format(ImageFormat::Svg).is_err());
    }

    /// PNG 인코딩/디코딩 왕복 테스트
    /// Validates: Requirements 1.1, 1.2
    #[test]
    fn encode_decode_round_trip_png() {
        let img = create_test_image(16, 16);
        let path = temp_path("round_trip.png");

        encode_raster(&img, ImageFormat::Png, &path, None).unwrap();
        let decoded = decode_raster(&path).unwrap();

        assert_eq!(decoded.width(), 16);
        assert_eq!(decoded.height(), 16);

        std::fs::remove_file(&path).ok();
    }

    /// JPEG 인코딩/디코딩 왕복 테스트 (품질 파라미터 적용)
    /// Validates: Requirements 1.1, 9.1
    #[test]
    fn encode_decode_round_trip_jpeg_with_quality() {
        let img = create_test_image(16, 16);
        let path = temp_path("round_trip_q50.jpg");

        encode_raster(&img, ImageFormat::Jpeg, &path, Some(50)).unwrap();
        let decoded = decode_raster(&path).unwrap();

        assert_eq!(decoded.width(), 16);
        assert_eq!(decoded.height(), 16);

        std::fs::remove_file(&path).ok();
    }

    /// JPEG 기본 품질(85) 인코딩 테스트
    /// Validates: Requirements 9.2
    #[test]
    fn encode_jpeg_default_quality() {
        let img = create_test_image(16, 16);
        let path = temp_path("round_trip_default_q.jpg");

        // quality=None → 기본값 85 적용
        encode_raster(&img, ImageFormat::Jpeg, &path, None).unwrap();
        let decoded = decode_raster(&path).unwrap();

        assert_eq!(decoded.width(), 16);
        assert_eq!(decoded.height(), 16);

        std::fs::remove_file(&path).ok();
    }

    /// BMP 인코딩/디코딩 왕복 테스트
    /// Validates: Requirements 1.1, 1.2
    #[test]
    fn encode_decode_round_trip_bmp() {
        let img = create_test_image(8, 8);
        let path = temp_path("round_trip.bmp");

        encode_raster(&img, ImageFormat::Bmp, &path, None).unwrap();
        let decoded = decode_raster(&path).unwrap();

        assert_eq!(decoded.width(), 8);
        assert_eq!(decoded.height(), 8);

        std::fs::remove_file(&path).ok();
    }

    /// Farbfeld 인코딩/디코딩 왕복 테스트 (Rgb8 입력 → Rgba16 자동 변환)
    /// Validates: Requirements 1.1, 1.2
    #[test]
    fn encode_decode_round_trip_farbfeld() {
        let img = create_test_image(8, 8);
        let path = temp_path("round_trip.ff");

        encode_raster(&img, ImageFormat::Farbfeld, &path, None).unwrap();
        let decoded = decode_raster(&path).unwrap();

        assert_eq!(decoded.width(), 8);
        assert_eq!(decoded.height(), 8);

        std::fs::remove_file(&path).ok();
    }

    /// Farbfeld 인코딩: Rgb8 이미지(JPEG 디코딩 결과와 동일)도 정상 처리되는지 확인
    #[test]
    fn encode_farbfeld_from_rgb8_image() {
        // JPEG 디코딩 결과와 동일한 Rgb8 이미지 생성
        let rgb_img = image::RgbImage::new(16, 16);
        let img = DynamicImage::ImageRgb8(rgb_img);
        let path = temp_path("farbfeld_rgb8.ff");

        encode_raster(&img, ImageFormat::Farbfeld, &path, None).unwrap();
        let decoded = decode_raster(&path).unwrap();

        assert_eq!(decoded.width(), 16);
        assert_eq!(decoded.height(), 16);

        std::fs::remove_file(&path).ok();
    }

    /// 존재하지 않는 파일 디코딩 시 에러 반환
    #[test]
    fn decode_nonexistent_file_returns_error() {
        let result = decode_raster(Path::new("/tmp/imgconv_nonexistent_file.png"));
        assert!(result.is_err());
    }
}
