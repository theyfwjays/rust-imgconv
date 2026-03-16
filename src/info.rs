// imgconv - 이미지 정보 출력 모듈

use std::path::{Path, PathBuf};

use image::GenericImageView;

use crate::error::ConvertError;
use crate::format::ImageFormat;

/// EXIF 요약 정보
#[derive(Debug)]
pub struct ExifSummary {
    pub camera_model: Option<String>,
    pub date_taken: Option<String>,
    pub iso: Option<u32>,
    pub shutter_speed: Option<String>,
    pub aperture: Option<String>,
}

/// 이미지 메타데이터 정보
#[derive(Debug)]
pub struct ImageInfo {
    pub width: u32,
    pub height: u32,
    pub format: ImageFormat,
    pub color_type: String,
    pub bit_depth: u8,
    pub file_size: u64,
    pub exif_summary: Option<ExifSummary>,
}

/// color_type과 bit_depth를 image::ColorType으로부터 추출
fn color_type_info(ct: image::ColorType) -> (String, u8) {
    let name = match ct {
        image::ColorType::L8 => "L8 (Grayscale 8-bit)",
        image::ColorType::La8 => "La8 (Grayscale+Alpha 8-bit)",
        image::ColorType::Rgb8 => "Rgb8 (RGB 8-bit)",
        image::ColorType::Rgba8 => "Rgba8 (RGBA 8-bit)",
        image::ColorType::L16 => "L16 (Grayscale 16-bit)",
        image::ColorType::La16 => "La16 (Grayscale+Alpha 16-bit)",
        image::ColorType::Rgb16 => "Rgb16 (RGB 16-bit)",
        image::ColorType::Rgba16 => "Rgba16 (RGBA 16-bit)",
        image::ColorType::Rgb32F => "Rgb32F (RGB 32-bit float)",
        image::ColorType::Rgba32F => "Rgba32F (RGBA 32-bit float)",
        _ => "Unknown",
    };
    let bits = ct.bits_per_pixel() / ct.channel_count() as u16;
    (name.to_string(), bits as u8)
}

/// 파일 경로에서 확장자를 추출하여 ImageFormat을 감지
fn detect_format(path: &Path) -> Result<ImageFormat, ConvertError> {
    let ext = path.extension().and_then(|e| e.to_str()).ok_or_else(|| {
        ConvertError::UnsupportedInputFormat {
            extension: String::new(),
            supported: ImageFormat::supported_extensions().join(", "),
        }
    })?;
    ImageFormat::from_extension(ext)
}

/// 이미지 파일의 메타데이터를 읽어 반환
pub fn get_image_info(path: &Path) -> Result<ImageInfo, ConvertError> {
    let format = detect_format(path)?;
    let file_size = std::fs::metadata(path)?.len();

    let reader = image::ImageReader::open(path)
        .map_err(|e| ConvertError::DecodingError(e.to_string()))?
        .with_guessed_format()
        .map_err(|e| ConvertError::DecodingError(e.to_string()))?;

    let img = reader
        .decode()
        .map_err(|e| ConvertError::DecodingError(e.to_string()))?;

    let (width, height) = img.dimensions();
    let ct = img.color();
    let (color_type, bit_depth) = color_type_info(ct);

    // EXIF reading will be added in task 24.1 (kamadak-exif dependency)
    let exif_summary = None;

    Ok(ImageInfo {
        width,
        height,
        format,
        color_type,
        bit_depth,
        file_size,
        exif_summary,
    })
}

/// 디렉토리 내 모든 이미지 파일의 정보를 반환
pub fn get_directory_info(dir: &Path) -> Result<Vec<(PathBuf, ImageInfo)>, ConvertError> {
    if !dir.is_dir() {
        return Err(ConvertError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("디렉토리가 아닙니다: {}", dir.display()),
        )));
    }

    let supported = ImageFormat::supported_extensions();
    let mut results = Vec::new();

    let entries = std::fs::read_dir(dir)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let ext = match path.extension().and_then(|e| e.to_str()) {
            Some(e) => e.to_ascii_lowercase(),
            None => continue,
        };

        if !supported.iter().any(|&s| s == ext) {
            continue;
        }

        match get_image_info(&path) {
            Ok(info) => results.push((path, info)),
            Err(_) => continue, // skip files that fail to decode
        }
    }

    Ok(results)
}

impl std::fmt::Display for ImageInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "  Format:     {}", self.format)?;
        writeln!(f, "  Dimensions: {}x{}", self.width, self.height)?;
        writeln!(f, "  Color Type: {}", self.color_type)?;
        writeln!(f, "  Bit Depth:  {}", self.bit_depth)?;
        writeln!(f, "  File Size:  {} bytes", self.file_size)?;
        if let Some(ref exif) = self.exif_summary {
            writeln!(f, "  EXIF:")?;
            if let Some(ref model) = exif.camera_model {
                writeln!(f, "    Camera:        {model}")?;
            }
            if let Some(ref date) = exif.date_taken {
                writeln!(f, "    Date Taken:    {date}")?;
            }
            if let Some(iso) = exif.iso {
                writeln!(f, "    ISO:           {iso}")?;
            }
            if let Some(ref ss) = exif.shutter_speed {
                writeln!(f, "    Shutter Speed: {ss}")?;
            }
            if let Some(ref ap) = exif.aperture {
                writeln!(f, "    Aperture:      {ap}")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, RgbaImage};

    fn create_test_image(w: u32, h: u32) -> DynamicImage {
        let img = RgbaImage::new(w, h);
        DynamicImage::ImageRgba8(img)
    }

    /// get_image_info가 유효한 이미지에서 올바른 메타데이터를 반환하는지 확인
    /// Validates: Requirements 32.1
    #[test]
    fn get_image_info_returns_valid_metadata() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.png");
        let img = create_test_image(64, 48);
        img.save(&path).unwrap();

        let info = get_image_info(&path).unwrap();
        assert_eq!(info.width, 64);
        assert_eq!(info.height, 48);
        assert_eq!(info.format, ImageFormat::Png);
        assert!(info.file_size > 0);
        assert!(info.exif_summary.is_none());
    }

    /// get_image_info가 JPEG 이미지에서도 올바르게 동작하는지 확인
    /// Validates: Requirements 32.1
    #[test]
    fn get_image_info_jpeg() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.jpg");
        let img = create_test_image(32, 32);
        img.save(&path).unwrap();

        let info = get_image_info(&path).unwrap();
        assert_eq!(info.width, 32);
        assert_eq!(info.height, 32);
        assert_eq!(info.format, ImageFormat::Jpeg);
        assert!(info.file_size > 0);
    }

    /// get_directory_info가 디렉토리 내 이미지 파일 정보를 수집하는지 확인
    /// Validates: Requirements 32.4
    #[test]
    fn get_directory_info_collects_images() {
        let dir = tempfile::tempdir().unwrap();
        let img = create_test_image(16, 16);

        img.save(dir.path().join("a.png")).unwrap();
        img.save(dir.path().join("b.jpg")).unwrap();
        // non-image file should be skipped
        std::fs::write(dir.path().join("readme.txt"), "hello").unwrap();

        let results = get_directory_info(dir.path()).unwrap();
        assert_eq!(results.len(), 2);
    }

    /// get_directory_info가 이미지 파일이 없는 디렉토리에서 빈 벡터를 반환하는지 확인
    /// Validates: Requirements 32.4
    #[test]
    fn get_directory_info_empty_dir() {
        let dir = tempfile::tempdir().unwrap();
        let results = get_directory_info(dir.path()).unwrap();
        assert!(results.is_empty());
    }

    /// 존재하지 않는 파일에 대해 에러를 반환하는지 확인
    #[test]
    fn get_image_info_nonexistent_file() {
        let result = get_image_info(Path::new("/tmp/imgconv_nonexistent_test.png"));
        assert!(result.is_err());
    }

    /// color_type_info가 올바른 값을 반환하는지 확인
    #[test]
    fn color_type_info_correctness() {
        let (name, bits) = color_type_info(image::ColorType::Rgba8);
        assert!(name.contains("RGBA"));
        assert_eq!(bits, 8);

        let (name, bits) = color_type_info(image::ColorType::Rgb16);
        assert!(name.contains("RGB"));
        assert_eq!(bits, 16);

        let (name, bits) = color_type_info(image::ColorType::L8);
        assert!(name.contains("Grayscale"));
        assert_eq!(bits, 8);
    }

    /// Display trait 구현이 정상 동작하는지 확인
    #[test]
    fn image_info_display() {
        let info = ImageInfo {
            width: 100,
            height: 200,
            format: ImageFormat::Png,
            color_type: "Rgba8 (RGBA 8-bit)".to_string(),
            bit_depth: 8,
            file_size: 1024,
            exif_summary: None,
        };
        let output = format!("{info}");
        assert!(output.contains("100x200"));
        assert!(output.contains("png"));
        assert!(output.contains("1024"));
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use image::{DynamicImage, RgbaImage};
    use proptest::prelude::*;

    // Property 22: 이미지 정보 출력 시 유효한 메타데이터 — 유효한 이미지에서 0보다 큰 너비/높이 반환
    // **Validates: 요구사항 32.1**
    proptest! {
        #[test]
        fn prop22_image_info_valid_metadata(
            w in 1u32..=200,
            h in 1u32..=200,
        ) {
            let dir = tempfile::tempdir().unwrap();
            let path = dir.path().join("test.png");
            let img = DynamicImage::ImageRgba8(RgbaImage::new(w, h));
            img.save(&path).unwrap();

            let info = get_image_info(&path).unwrap();
            prop_assert!(info.width > 0, "width should be > 0, got {}", info.width);
            prop_assert!(info.height > 0, "height should be > 0, got {}", info.height);
            prop_assert!(info.file_size > 0, "file_size should be > 0, got {}", info.file_size);
            prop_assert_eq!(info.width, w, "width mismatch: expected {}, got {}", w, info.width);
            prop_assert_eq!(info.height, h, "height mismatch: expected {}, got {}", h, info.height);
        }
    }
}
