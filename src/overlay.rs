use crate::error::ConvertError;
use crate::watermark::Position;
use image::{DynamicImage, RgbaImage};
use std::path::PathBuf;

const MARGIN: u32 = 10;

#[derive(Debug, Clone)]
pub struct OverlayOptions {
    pub image_path: PathBuf,
    pub position: Position,
    pub opacity: f32,
}

impl Default for OverlayOptions {
    fn default() -> Self {
        Self {
            image_path: PathBuf::new(),
            position: Position::default(),
            opacity: 1.0,
        }
    }
}

fn calculate_overlay_position(
    position: Position,
    base_width: u32,
    base_height: u32,
    overlay_width: u32,
    overlay_height: u32,
) -> (i64, i64) {
    match position {
        Position::TopLeft => (MARGIN as i64, MARGIN as i64),
        Position::TopRight => {
            let x = base_width
                .saturating_sub(overlay_width)
                .saturating_sub(MARGIN) as i64;
            (x, MARGIN as i64)
        }
        Position::BottomLeft => {
            let y = base_height
                .saturating_sub(overlay_height)
                .saturating_sub(MARGIN) as i64;
            (MARGIN as i64, y)
        }
        Position::BottomRight => {
            let x = base_width
                .saturating_sub(overlay_width)
                .saturating_sub(MARGIN) as i64;
            let y = base_height
                .saturating_sub(overlay_height)
                .saturating_sub(MARGIN) as i64;
            (x, y)
        }
        Position::Center => {
            let x = (base_width.saturating_sub(overlay_width) / 2) as i64;
            let y = (base_height.saturating_sub(overlay_height) / 2) as i64;
            (x, y)
        }
    }
}

fn apply_opacity(overlay: &DynamicImage, opacity: f32) -> RgbaImage {
    let opacity = opacity.clamp(0.0, 1.0);
    let mut rgba = overlay.to_rgba8();
    if (opacity - 1.0).abs() > f32::EPSILON {
        for pixel in rgba.pixels_mut() {
            pixel[3] = (pixel[3] as f32 * opacity).round().min(255.0) as u8;
        }
    }
    rgba
}
pub fn apply_overlay(
    img: &DynamicImage,
    options: &OverlayOptions,
) -> Result<DynamicImage, ConvertError> {
    if !options.image_path.exists() {
        return Err(ConvertError::OverlayFileNotFound {
            path: options.image_path.display().to_string(),
        });
    }

    let overlay_img =
        image::open(&options.image_path).map_err(|e| ConvertError::OverlayUnsupportedFormat {
            path: format!("{}: {}", options.image_path.display(), e),
        })?;

    let overlay_rgba = apply_opacity(&overlay_img, options.opacity);

    let (base_w, base_h) = (img.width(), img.height());
    let (overlay_w, overlay_h) = (overlay_rgba.width(), overlay_rgba.height());
    let (x, y) = calculate_overlay_position(options.position, base_w, base_h, overlay_w, overlay_h);

    let mut result = img.to_rgba8();
    image::imageops::overlay(&mut result, &overlay_rgba, x, y);

    Ok(DynamicImage::ImageRgba8(result))
}
#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, Rgba, RgbaImage};
    use std::path::PathBuf;

    fn create_test_image(w: u32, h: u32, color: [u8; 4]) -> DynamicImage {
        let mut img = RgbaImage::new(w, h);
        for pixel in img.pixels_mut() {
            *pixel = Rgba(color);
        }
        DynamicImage::ImageRgba8(img)
    }

    #[test]
    fn overlay_options_default() {
        let opts = OverlayOptions::default();
        assert_eq!(opts.position, Position::BottomRight);
        assert_eq!(opts.opacity, 1.0);
        assert_eq!(opts.image_path, PathBuf::new());
    }

    #[test]
    fn overlay_file_not_found() {
        let img = create_test_image(100, 100, [100, 150, 200, 255]);
        let options = OverlayOptions {
            image_path: PathBuf::from("/nonexistent/overlay.png"),
            ..Default::default()
        };
        let result = apply_overlay(&img, &options);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConvertError::OverlayFileNotFound { .. }
        ));
    }

    #[test]
    fn overlay_unsupported_format() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("bad.png");
        std::fs::write(&path, b"not an image").unwrap();
        let img = create_test_image(100, 100, [100, 150, 200, 255]);
        let options = OverlayOptions {
            image_path: path,
            ..Default::default()
        };
        let result = apply_overlay(&img, &options);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConvertError::OverlayUnsupportedFormat { .. }
        ));
    }
    #[test]
    fn overlay_preserves_base_size() {
        let dir = tempfile::tempdir().unwrap();
        let overlay_path = dir.path().join("overlay.png");
        let overlay_img = create_test_image(30, 30, [255, 0, 0, 128]);
        overlay_img.save(&overlay_path).unwrap();
        let base = create_test_image(100, 100, [0, 0, 255, 255]);
        let options = OverlayOptions {
            image_path: overlay_path,
            position: Position::Center,
            opacity: 1.0,
        };
        let result = apply_overlay(&base, &options).unwrap();
        assert_eq!(result.width(), 100);
        assert_eq!(result.height(), 100);
    }

    #[test]
    fn overlay_changes_pixels() {
        let dir = tempfile::tempdir().unwrap();
        let overlay_path = dir.path().join("overlay.png");
        let overlay_img = create_test_image(30, 30, [255, 0, 0, 255]);
        overlay_img.save(&overlay_path).unwrap();
        let base = create_test_image(100, 100, [0, 0, 255, 255]);
        let options = OverlayOptions {
            image_path: overlay_path,
            position: Position::Center,
            opacity: 1.0,
        };
        let result = apply_overlay(&base, &options).unwrap();
        let base_bytes = base.as_bytes();
        let result_bytes = result.as_bytes();
        let changed = base_bytes
            .iter()
            .zip(result_bytes.iter())
            .any(|(a, b)| a != b);
        assert!(changed, "Overlay should change at least some pixels");
    }

    #[test]
    fn overlay_with_zero_opacity_no_visible_change() {
        let dir = tempfile::tempdir().unwrap();
        let overlay_path = dir.path().join("overlay.png");
        let overlay_img = create_test_image(30, 30, [255, 0, 0, 255]);
        overlay_img.save(&overlay_path).unwrap();
        let base = create_test_image(100, 100, [0, 0, 255, 255]);
        let options = OverlayOptions {
            image_path: overlay_path,
            position: Position::Center,
            opacity: 0.0,
        };
        let result = apply_overlay(&base, &options).unwrap();
        let base_bytes = base.as_bytes();
        let result_bytes = result.as_bytes();
        assert_eq!(
            base_bytes, result_bytes,
            "Zero opacity should not change base"
        );
    }
    #[test]
    fn calc_pos_top_left() {
        let (x, y) = calculate_overlay_position(Position::TopLeft, 200, 200, 50, 30);
        assert_eq!(x, MARGIN as i64);
        assert_eq!(y, MARGIN as i64);
    }

    #[test]
    fn calc_pos_bottom_right() {
        let (x, y) = calculate_overlay_position(Position::BottomRight, 200, 200, 50, 30);
        assert_eq!(x, (200 - 50 - MARGIN) as i64);
        assert_eq!(y, (200 - 30 - MARGIN) as i64);
    }

    #[test]
    fn calc_pos_center() {
        let (x, y) = calculate_overlay_position(Position::Center, 200, 200, 50, 30);
        assert_eq!(x, 75);
        assert_eq!(y, 85);
    }

    #[test]
    fn opacity_full_preserves_alpha() {
        let img = create_test_image(10, 10, [255, 0, 0, 200]);
        let result = apply_opacity(&img, 1.0);
        assert_eq!(result.get_pixel(0, 0)[3], 200);
    }

    #[test]
    fn opacity_half() {
        let img = create_test_image(10, 10, [255, 0, 0, 200]);
        let result = apply_opacity(&img, 0.5);
        assert_eq!(result.get_pixel(0, 0)[3], 100);
    }

    #[test]
    fn opacity_zero() {
        let img = create_test_image(10, 10, [255, 0, 0, 200]);
        let result = apply_opacity(&img, 0.0);
        assert_eq!(result.get_pixel(0, 0)[3], 0);
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use image::{DynamicImage, Rgba, RgbaImage};
    use proptest::prelude::*;

    // Property 15: 오버레이 적용 시 크기 보존
    // **Validates: Requirements 28.1**
    proptest! {
        #[test]
        fn prop15_overlay_preserves_base_size(
            base_w in 20u32..200,
            base_h in 20u32..200,
            overlay_w in 5u32..50,
            overlay_h in 5u32..50,
        ) {
            let dir = tempfile::tempdir().unwrap();
            let overlay_path = dir.path().join("overlay.png");

            let mut overlay_img = RgbaImage::new(overlay_w, overlay_h);
            for pixel in overlay_img.pixels_mut() {
                *pixel = Rgba([255, 0, 0, 128]);
            }
            DynamicImage::ImageRgba8(overlay_img).save(&overlay_path).unwrap();

            let mut base_img = RgbaImage::new(base_w, base_h);
            for pixel in base_img.pixels_mut() {
                *pixel = Rgba([0, 0, 255, 255]);
            }
            let base = DynamicImage::ImageRgba8(base_img);

            let options = OverlayOptions {
                image_path: overlay_path,
                position: Position::Center,
                opacity: 0.5,
            };

            let result = apply_overlay(&base, &options).unwrap();
            prop_assert_eq!(result.width(), base_w);
            prop_assert_eq!(result.height(), base_h);
        }
    }

    // Property 16: 존재하지 않는 오버레이 파일 에러
    // **Validates: Requirements 28.6**
    proptest! {
        #[test]
        fn prop16_nonexistent_overlay_file_error(
            filename in "[a-z]{3,10}\\.png"
        ) {
            let base = DynamicImage::ImageRgba8(RgbaImage::new(50, 50));
            let options = OverlayOptions {
                image_path: std::path::PathBuf::from(format!("/nonexistent/{}", filename)),
                position: Position::Center,
                opacity: 1.0,
            };
            let result = apply_overlay(&base, &options);
            prop_assert!(result.is_err());
            let err = result.unwrap_err();
            assert!(matches!(err, ConvertError::OverlayFileNotFound { .. }));
        }
    }
}
