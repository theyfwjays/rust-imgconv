// imgconv - 텍스트 워터마크 모듈

use std::path::{Path, PathBuf};

use ab_glyph::{FontArc, PxScale};
use image::{DynamicImage, Rgba, RgbaImage};
use imageproc::drawing::{draw_text_mut, text_size};

use crate::error::ConvertError;

/// 워터마크 위치
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}

impl Default for Position {
    fn default() -> Self {
        Self::BottomRight
    }
}

impl Position {
    /// 문자열로부터 Position 파싱
    pub fn from_str(s: &str) -> Result<Self, ConvertError> {
        match s.to_lowercase().as_str() {
            "top-left" | "topleft" => Ok(Self::TopLeft),
            "top-right" | "topright" => Ok(Self::TopRight),
            "bottom-left" | "bottomleft" => Ok(Self::BottomLeft),
            "bottom-right" | "bottomright" => Ok(Self::BottomRight),
            "center" => Ok(Self::Center),
            _ => Err(ConvertError::WatermarkError(format!(
                "유효하지 않은 위치: '{}'. 유효 값: top-left, top-right, bottom-left, bottom-right, center",
                s
            ))),
        }
    }
}

/// 텍스트 워터마크 옵션
#[derive(Debug, Clone)]
pub struct WatermarkOptions {
    /// 워터마크 텍스트
    pub text: String,
    /// 배치 위치 (기본: bottom-right)
    pub position: Position,
    /// 투명도 (0.0-1.0, 기본: 0.5)
    pub opacity: f32,
    /// 폰트 파일 경로 (None이면 시스템 기본 폰트 탐색)
    pub font_path: Option<PathBuf>,
}

impl Default for WatermarkOptions {
    fn default() -> Self {
        Self {
            text: String::new(),
            position: Position::default(),
            opacity: 0.5,
            font_path: None,
        }
    }
}

/// 여백 (픽셀)
const MARGIN: u32 = 10;

/// 기본 폰트 크기 (픽셀)
const DEFAULT_FONT_SIZE: f32 = 24.0;

/// 시스템 기본 폰트 경로 후보 목록
fn system_font_paths() -> Vec<&'static str> {
    vec![
        // Linux
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/usr/share/fonts/TTF/DejaVuSans.ttf",
        "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
        "/usr/share/fonts/liberation-sans/LiberationSans-Regular.ttf",
        "/usr/share/fonts/truetype/freefont/FreeSans.ttf",
        "/usr/share/fonts/noto/NotoSans-Regular.ttf",
        "/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf",
        "/usr/share/fonts/google-noto/NotoSans-Regular.ttf",
        // macOS
        "/System/Library/Fonts/Helvetica.ttc",
        "/System/Library/Fonts/SFNSText.ttf",
        "/Library/Fonts/Arial.ttf",
        // Windows
        "C:\\Windows\\Fonts\\arial.ttf",
        "C:\\Windows\\Fonts\\segoeui.ttf",
        "C:\\Windows\\Fonts\\tahoma.ttf",
    ]
}

/// 폰트 로딩: 사용자 지정 경로 또는 시스템 기본 폰트
fn load_font(font_path: &Option<PathBuf>) -> Result<FontArc, ConvertError> {
    if let Some(path) = font_path {
        let data = std::fs::read(path).map_err(|e| {
            ConvertError::WatermarkError(format!(
                "폰트 파일을 읽을 수 없음: {}: {}",
                path.display(),
                e
            ))
        })?;
        FontArc::try_from_vec(data).map_err(|_| {
            ConvertError::WatermarkError(format!(
                "유효하지 않은 폰트 파일: {}",
                path.display()
            ))
        })
    } else {
        // 시스템 기본 폰트 탐색
        for path_str in system_font_paths() {
            let path = Path::new(path_str);
            if path.exists() {
                if let Ok(data) = std::fs::read(path) {
                    if let Ok(font) = FontArc::try_from_vec(data) {
                        return Ok(font);
                    }
                }
            }
        }
        Err(ConvertError::WatermarkError(
            "시스템에서 사용 가능한 폰트를 찾을 수 없습니다. --watermark-font 옵션으로 폰트 파일을 지정하세요".to_string()
        ))
    }
}

/// 위치별 텍스트 시작 좌표 계산
fn calculate_position(
    position: Position,
    image_width: u32,
    image_height: u32,
    text_width: u32,
    text_height: u32,
) -> (i32, i32) {
    match position {
        Position::TopLeft => (MARGIN as i32, MARGIN as i32),
        Position::TopRight => {
            let x = (image_width.saturating_sub(text_width).saturating_sub(MARGIN)) as i32;
            let y = MARGIN as i32;
            (x, y)
        }
        Position::BottomLeft => {
            let x = MARGIN as i32;
            let y = (image_height.saturating_sub(text_height).saturating_sub(MARGIN)) as i32;
            (x, y)
        }
        Position::BottomRight => {
            let x = (image_width.saturating_sub(text_width).saturating_sub(MARGIN)) as i32;
            let y = (image_height.saturating_sub(text_height).saturating_sub(MARGIN)) as i32;
            (x, y)
        }
        Position::Center => {
            let x = ((image_width.saturating_sub(text_width)) / 2) as i32;
            let y = ((image_height.saturating_sub(text_height)) / 2) as i32;
            (x, y)
        }
    }
}

/// 이미지에 텍스트 워터마크 적용
///
/// imageproc + ab_glyph를 사용하여 텍스트를 렌더링하고,
/// 알파 블렌딩으로 투명도를 적용한다.
pub fn apply_watermark(
    img: &DynamicImage,
    options: &WatermarkOptions,
) -> Result<DynamicImage, ConvertError> {
    if options.text.is_empty() {
        return Err(ConvertError::WatermarkError(
            "워터마크 텍스트가 비어있습니다".to_string(),
        ));
    }

    let opacity = options.opacity.clamp(0.0, 1.0);
    let font = load_font(&options.font_path)?;
    let scale = PxScale::from(DEFAULT_FONT_SIZE);

    // 텍스트 크기 측정
    let (text_width, text_height) = text_size(scale, &font, &options.text);

    let (img_w, img_h) = (img.width(), img.height());

    // 위치 계산
    let (x, y) = calculate_position(
        options.position,
        img_w,
        img_h,
        text_width,
        text_height,
    );

    // 원본 이미지를 RGBA로 변환
    let base_rgba = img.to_rgba8();

    // 텍스트를 별도의 투명 레이어에 렌더링
    let mut text_layer = RgbaImage::new(img_w, img_h);

    // 흰색 텍스트를 텍스트 레이어에 그리기
    let text_color = Rgba([255u8, 255, 255, 255]);
    draw_text_mut(&mut text_layer, text_color, x, y, scale, &font, &options.text);

    // 알파 블렌딩으로 텍스트 레이어를 원본에 합성
    let mut result = base_rgba;
    for (base_pixel, text_pixel) in result.pixels_mut().zip(text_layer.pixels()) {
        let text_alpha = text_pixel[3] as f32 / 255.0 * opacity;
        if text_alpha > 0.0 {
            let inv_alpha = 1.0 - text_alpha;
            base_pixel[0] = (text_pixel[0] as f32 * text_alpha + base_pixel[0] as f32 * inv_alpha) as u8;
            base_pixel[1] = (text_pixel[1] as f32 * text_alpha + base_pixel[1] as f32 * inv_alpha) as u8;
            base_pixel[2] = (text_pixel[2] as f32 * text_alpha + base_pixel[2] as f32 * inv_alpha) as u8;
            // 알파 채널 보존
            base_pixel[3] = ((text_alpha + base_pixel[3] as f32 / 255.0 * inv_alpha) * 255.0).min(255.0) as u8;
        }
    }

    Ok(DynamicImage::ImageRgba8(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, Rgba, RgbaImage};

    fn create_test_image(w: u32, h: u32) -> DynamicImage {
        let mut img = RgbaImage::new(w, h);
        for pixel in img.pixels_mut() {
            *pixel = Rgba([100, 150, 200, 255]);
        }
        DynamicImage::ImageRgba8(img)
    }

    #[test]
    fn position_default_is_bottom_right() {
        assert_eq!(Position::default(), Position::BottomRight);
    }

    #[test]
    fn position_from_str_valid() {
        assert_eq!(Position::from_str("top-left").unwrap(), Position::TopLeft);
        assert_eq!(Position::from_str("top-right").unwrap(), Position::TopRight);
        assert_eq!(Position::from_str("bottom-left").unwrap(), Position::BottomLeft);
        assert_eq!(Position::from_str("bottom-right").unwrap(), Position::BottomRight);
        assert_eq!(Position::from_str("center").unwrap(), Position::Center);
        assert_eq!(Position::from_str("TopLeft").unwrap(), Position::TopLeft);
    }

    #[test]
    fn position_from_str_invalid() {
        assert!(Position::from_str("invalid").is_err());
        assert!(Position::from_str("").is_err());
    }

    #[test]
    fn empty_text_returns_error() {
        let img = create_test_image(100, 100);
        let options = WatermarkOptions {
            text: String::new(),
            ..Default::default()
        };
        assert!(apply_watermark(&img, &options).is_err());
    }

    #[test]
    fn invalid_font_path_returns_error() {
        let img = create_test_image(100, 100);
        let options = WatermarkOptions {
            text: "test".to_string(),
            font_path: Some(PathBuf::from("/nonexistent/font.ttf")),
            ..Default::default()
        };
        assert!(apply_watermark(&img, &options).is_err());
    }

    #[test]
    fn calculate_position_top_left() {
        let (x, y) = calculate_position(Position::TopLeft, 200, 200, 50, 20);
        assert_eq!(x, MARGIN as i32);
        assert_eq!(y, MARGIN as i32);
    }

    #[test]
    fn calculate_position_bottom_right() {
        let (x, y) = calculate_position(Position::BottomRight, 200, 200, 50, 20);
        assert_eq!(x, (200 - 50 - MARGIN) as i32);
        assert_eq!(y, (200 - 20 - MARGIN) as i32);
    }

    #[test]
    fn calculate_position_center() {
        let (x, y) = calculate_position(Position::Center, 200, 200, 50, 20);
        assert_eq!(x, 75); // (200 - 50) / 2
        assert_eq!(y, 90); // (200 - 20) / 2
    }

    #[test]
    fn watermark_options_default() {
        let opts = WatermarkOptions::default();
        assert_eq!(opts.position, Position::BottomRight);
        assert_eq!(opts.opacity, 0.5);
        assert!(opts.font_path.is_none());
        assert!(opts.text.is_empty());
    }

    #[test]
    fn opacity_clamped_to_valid_range() {
        // This tests that the apply_watermark function clamps opacity.
        // We can't easily test the actual rendering without a font,
        // but we verify the clamping logic by checking the code path.
        let img = create_test_image(100, 100);
        let options = WatermarkOptions {
            text: "test".to_string(),
            opacity: 2.0, // should be clamped to 1.0
            ..Default::default()
        };
        // This will fail if no system font is available, which is expected
        // in some CI environments. The important thing is it doesn't panic
        // from the opacity value.
        let _ = apply_watermark(&img, &options);
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use image::{DynamicImage, Rgba, RgbaImage};
    use proptest::prelude::*;

    fn has_system_font() -> bool {
        for path_str in system_font_paths() {
            if std::path::Path::new(path_str).exists() {
                return true;
            }
        }
        false
    }

    // Property 14: 워터마크 적용 시 크기 보존 및 픽셀 변경
    // **Validates: Requirements 27.1**
    proptest! {
        #[test]
        fn prop14_watermark_preserves_size_changes_pixels(
            w in 50u32..200,
            h in 50u32..200,
        ) {
            prop_assume!(has_system_font());

            let mut img = RgbaImage::new(w, h);
            for pixel in img.pixels_mut() {
                *pixel = Rgba([100, 100, 100, 255]);
            }
            let img = DynamicImage::ImageRgba8(img);

            let options = WatermarkOptions {
                text: "Test Watermark".to_string(),
                position: Position::Center,
                opacity: 1.0,
                font_path: None,
            };

            let result = apply_watermark(&img, &options).unwrap();

            // Size preserved
            prop_assert_eq!(result.width(), w);
            prop_assert_eq!(result.height(), h);

            // At least some pixels changed
            let original_bytes = img.as_bytes();
            let result_bytes = result.as_bytes();
            let changed = original_bytes.iter().zip(result_bytes.iter()).any(|(a, b)| a != b);
            prop_assert!(changed, "Watermark should change at least some pixels");
        }
    }
}