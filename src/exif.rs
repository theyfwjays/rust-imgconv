// imgconv - EXIF 방향 보정 및 EXIF 보존 모듈

use std::path::Path;

use image::DynamicImage;
use img_parts::jpeg::Jpeg;
use img_parts::ImageEXIF;

use crate::error::ConvertError;
use crate::format::ImageFormat;

/// EXIF Orientation 태그 값
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExifOrientation {
    Normal,         // 1
    FlipHorizontal, // 2
    Rotate180,      // 3
    FlipVertical,   // 4
    Transpose,      // 5
    Rotate90,       // 6
    Transverse,     // 7
    Rotate270,      // 8
}

impl ExifOrientation {
    /// EXIF Orientation 태그 값(1-8)으로부터 열거형 변환
    pub fn from_value(value: u32) -> Option<Self> {
        match value {
            1 => Some(Self::Normal),
            2 => Some(Self::FlipHorizontal),
            3 => Some(Self::Rotate180),
            4 => Some(Self::FlipVertical),
            5 => Some(Self::Transpose),
            6 => Some(Self::Rotate90),
            7 => Some(Self::Transverse),
            8 => Some(Self::Rotate270),
            _ => None,
        }
    }

    /// 방향에 따라 이미지에 회전/뒤집기 변환을 적용
    pub fn apply(self, img: &DynamicImage) -> DynamicImage {
        match self {
            Self::Normal => img.clone(),
            Self::FlipHorizontal => img.fliph(),
            Self::Rotate180 => img.rotate180(),
            Self::FlipVertical => img.flipv(),
            Self::Transpose => img.rotate90().fliph(),
            Self::Rotate90 => img.rotate90(),
            Self::Transverse => img.rotate90().flipv(),
            Self::Rotate270 => img.rotate270(),
        }
    }
}

/// EXIF Orientation 태그를 읽어 이미지 방향을 자동 보정한다.
///
/// kamadak-exif 크레이트로 입력 파일의 EXIF Orientation 태그를 읽고,
/// 해당 방향에 맞게 이미지를 회전/뒤집기한다.
/// EXIF 데이터가 없거나 Orientation 태그가 없으면 원본 이미지를 그대로 반환한다.
pub fn auto_orient(img: &DynamicImage, input_path: &Path) -> Result<DynamicImage, ConvertError> {
    let orientation = read_orientation(input_path);

    match orientation {
        Some(orient) => Ok(orient.apply(img)),
        None => Ok(img.clone()),
    }
}

/// 파일에서 EXIF Orientation 태그 값을 읽는다.
/// EXIF 데이터가 없거나 Orientation 태그가 없으면 None을 반환한다.
fn read_orientation(path: &Path) -> Option<ExifOrientation> {
    let file = std::fs::File::open(path).ok()?;
    let mut bufreader = std::io::BufReader::new(&file);
    let exif_reader = exif::Reader::new();
    let exif_data = exif_reader.read_from_container(&mut bufreader).ok()?;

    let field = exif_data.get_field(exif::Tag::Orientation, exif::In::PRIMARY)?;
    let value = field.value.get_uint(0)?;

    ExifOrientation::from_value(value)
}

/// 파일 확장자로부터 포맷을 감지한다.
fn detect_format(path: &Path) -> Option<ImageFormat> {
    let ext = path.extension()?.to_str()?;
    ImageFormat::from_extension(ext).ok()
}

/// 원본 파일의 EXIF 메타데이터를 변환 결과 파일에 복사한다.
///
/// kamadak-exif + img-parts를 사용하여 원본 JPEG의 EXIF 데이터를
/// 변환된 JPEG 파일에 기록한다.
/// - JPEG→JPEG: EXIF 데이터 복사
/// - 크로스 포맷: 경고 메시지 출력
/// - EXIF 쓰기 실패: 경고 후 EXIF 없이 저장 (에러 반환하지 않음)
pub fn preserve_exif(source_path: &Path, dest_path: &Path) -> Result<(), ConvertError> {
    let src_format = detect_format(source_path);
    let dst_format = detect_format(dest_path);

    // 크로스 포맷 변환 시 경고
    match (&src_format, &dst_format) {
        (Some(ImageFormat::Jpeg), Some(ImageFormat::Jpeg)) => {
            // JPEG→JPEG: EXIF 복사 진행
        }
        _ => {
            eprintln!(
                "경고: 크로스 포맷 변환에서는 EXIF 보존이 제한적입니다 ({} → {})",
                source_path.display(),
                dest_path.display()
            );
            return Ok(());
        }
    }

    // 원본 JPEG에서 EXIF 데이터 읽기 (img-parts 사용)
    let src_bytes = std::fs::read(source_path)
        .map_err(|e| ConvertError::ExifError(format!("원본 파일 읽기 실패: {}", e)))?;

    let src_jpeg = match Jpeg::from_bytes(src_bytes.into()) {
        Ok(jpeg) => jpeg,
        Err(e) => {
            eprintln!("경고: 원본 EXIF 읽기 실패: {}. EXIF 없이 저장합니다.", e);
            return Ok(());
        }
    };

    let exif_data = match src_jpeg.exif() {
        Some(data) => data,
        None => {
            // 원본에 EXIF 데이터가 없으면 아무것도 하지 않음
            return Ok(());
        }
    };

    // 변환 결과 JPEG에 EXIF 데이터 기록
    let dst_bytes = std::fs::read(dest_path)
        .map_err(|e| ConvertError::ExifError(format!("출력 파일 읽기 실패: {}", e)))?;

    let mut dst_jpeg = match Jpeg::from_bytes(dst_bytes.into()) {
        Ok(jpeg) => jpeg,
        Err(e) => {
            eprintln!("경고: 출력 JPEG 파싱 실패: {}. EXIF 없이 저장합니다.", e);
            return Ok(());
        }
    };

    dst_jpeg.set_exif(Some(exif_data));

    // 파일에 다시 쓰기
    let output_file = std::fs::File::create(dest_path)
        .map_err(|e| ConvertError::ExifError(format!("출력 파일 쓰기 실패: {}", e)))?;

    if let Err(e) = dst_jpeg.encoder().write_to(output_file) {
        eprintln!("경고: EXIF 쓰기 실패: {}. EXIF 없이 저장합니다.", e);
    }

    Ok(())
}

#[cfg(test)]
mod proptests {
    use super::*;
    use image::{DynamicImage, Rgba, RgbaImage};
    use proptest::prelude::*;
    use tempfile::NamedTempFile;

    /// Create a test image with deterministic pixel values based on coordinates
    fn test_image_with_pixels(w: u32, h: u32) -> DynamicImage {
        let mut img = RgbaImage::new(w, h);
        for y in 0..h {
            for x in 0..w {
                let r = ((x * 7 + y * 13) % 256) as u8;
                let g = ((x * 11 + y * 3) % 256) as u8;
                let b = ((x * 5 + y * 17) % 256) as u8;
                img.put_pixel(x, y, Rgba([r, g, b, 255]));
            }
        }
        DynamicImage::ImageRgba8(img)
    }

    /// Create a minimal EXIF segment (APP1 marker) with an Orientation tag.
    /// This produces a valid EXIF/TIFF structure that kamadak-exif and img-parts can read.
    fn build_minimal_exif_bytes() -> Vec<u8> {
        // TIFF header (little-endian) + single IFD with Orientation tag
        let mut buf: Vec<u8> = Vec::new();

        // Exif header: "Exif\0\0"
        buf.extend_from_slice(b"Exif\0\0");

        // TIFF header: byte order (little-endian), magic 42, offset to IFD0
        buf.extend_from_slice(&[0x49, 0x49]); // "II" = little-endian
        buf.extend_from_slice(&42u16.to_le_bytes()); // magic
        buf.extend_from_slice(&8u32.to_le_bytes()); // offset to IFD0 (relative to TIFF header start)

        // IFD0 at offset 8 from TIFF header start (= byte index 6+8=14 in buf)
        // Number of entries: 1
        buf.extend_from_slice(&1u16.to_le_bytes());

        // IFD entry: Orientation tag (0x0112), type SHORT (3), count 1, value 1 (Normal)
        buf.extend_from_slice(&0x0112u16.to_le_bytes()); // tag
        buf.extend_from_slice(&3u16.to_le_bytes()); // type = SHORT
        buf.extend_from_slice(&1u32.to_le_bytes()); // count
        buf.extend_from_slice(&1u16.to_le_bytes()); // value = 1 (Normal)
        buf.extend_from_slice(&0u16.to_le_bytes()); // padding (value field is 4 bytes)

        // Next IFD offset: 0 (no more IFDs)
        buf.extend_from_slice(&0u32.to_le_bytes());

        buf
    }

    /// Create a JPEG file with embedded EXIF data using img-parts.
    fn create_jpeg_with_exif(img: &DynamicImage, path: &Path) {
        // First, save as a plain JPEG
        img.to_rgb8()
            .save_with_format(path, image::ImageFormat::Jpeg)
            .unwrap();

        // Read back and inject EXIF
        let jpeg_bytes = std::fs::read(path).unwrap();
        let mut jpeg = Jpeg::from_bytes(jpeg_bytes.into()).unwrap();

        let exif_bytes = build_minimal_exif_bytes();
        jpeg.set_exif(Some(exif_bytes.into()));

        let out = std::fs::File::create(path).unwrap();
        jpeg.encoder().write_to(out).unwrap();
    }

    // Property 17: EXIF 없는 이미지의 auto-orient 무변경
    // EXIF Orientation 태그 없는 이미지에 auto-orient 적용 시 원본과 동일
    // **Validates: Requirements 29.4**
    proptest! {
        #[test]
        fn prop17_auto_orient_no_exif_unchanged(
            w in 1u32..100,
            h in 1u32..100,
        ) {
            let img = test_image_with_pixels(w, h);

            // Save as PNG to a temp file — PNG has no EXIF data
            let tmp = NamedTempFile::with_suffix(".png").unwrap();
            img.save(tmp.path()).unwrap();

            // auto_orient should return the image unchanged
            let result = auto_orient(&img, tmp.path()).unwrap();

            prop_assert_eq!(result.width(), img.width(), "Width should be unchanged");
            prop_assert_eq!(result.height(), img.height(), "Height should be unchanged");
            prop_assert_eq!(result.as_bytes(), img.as_bytes(), "Pixel data should be identical");
        }
    }

    // Property 21: EXIF 보존 라운드트립
    // EXIF 있는 JPEG의 JPEG→JPEG 변환 시 출력에 EXIF 존재
    // **Validates: Requirements 31.1**
    proptest! {
        #[test]
        fn prop21_preserve_exif_roundtrip(
            w in 1u32..100,
            h in 1u32..100,
        ) {
            let img = test_image_with_pixels(w, h);

            // Create source JPEG with EXIF data
            let src_tmp = NamedTempFile::with_suffix(".jpg").unwrap();
            create_jpeg_with_exif(&img, src_tmp.path());

            // Verify source has EXIF
            let src_bytes = std::fs::read(src_tmp.path()).unwrap();
            let src_jpeg = Jpeg::from_bytes(src_bytes.into()).unwrap();
            prop_assert!(src_jpeg.exif().is_some(), "Source JPEG should have EXIF data");

            // Create destination JPEG (plain, no EXIF — simulates a fresh encode)
            let dst_tmp = NamedTempFile::with_suffix(".jpg").unwrap();
            img.to_rgb8()
                .save_with_format(dst_tmp.path(), image::ImageFormat::Jpeg)
                .unwrap();

            // Verify destination initially has no EXIF
            let dst_bytes_before = std::fs::read(dst_tmp.path()).unwrap();
            let dst_jpeg_before = Jpeg::from_bytes(dst_bytes_before.into()).unwrap();
            prop_assert!(dst_jpeg_before.exif().is_none(), "Destination JPEG should initially have no EXIF");

            // Preserve EXIF from source to destination
            preserve_exif(src_tmp.path(), dst_tmp.path()).unwrap();

            // Verify destination now has EXIF
            let dst_bytes_after = std::fs::read(dst_tmp.path()).unwrap();
            let dst_jpeg_after = Jpeg::from_bytes(dst_bytes_after.into()).unwrap();
            prop_assert!(dst_jpeg_after.exif().is_some(), "Destination JPEG should have EXIF after preserve_exif");
        }
    }
}

