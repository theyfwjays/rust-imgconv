// imgconv - 이미지 리사이즈

use image::imageops::FilterType;
use image::DynamicImage;

use crate::error::ConvertError;

/// 리사이즈 옵션
#[derive(Debug, Clone)]
pub struct ResizeOptions {
    /// 대상 너비 (픽셀)
    pub width: Option<u32>,
    /// 대상 높이 (픽셀)
    pub height: Option<u32>,
    /// 종횡비 유지 여부 (width + height 모두 지정 시에만 의미 있음)
    pub keep_aspect: bool,
}

/// 이미지를 주어진 옵션에 따라 리사이즈한다.
///
/// - width만 지정: 종횡비 유지하며 높이 자동 계산
/// - height만 지정: 종횡비 유지하며 너비 자동 계산
/// - width + height + keep_aspect: 지정 범위 내 종횡비 유지 (fit)
/// - width + height (keep_aspect 없음): 강제 리사이즈 (stretch)
pub fn resize_image(
    img: &DynamicImage,
    options: &ResizeOptions,
) -> Result<DynamicImage, ConvertError> {
    let (orig_w, orig_h) = (img.width(), img.height());

    if orig_w == 0 || orig_h == 0 {
        return Err(ConvertError::ResizeError(
            "원본 이미지의 크기가 0입니다".to_string(),
        ));
    }

    let (new_w, new_h) = match (options.width, options.height) {
        (Some(w), Some(h)) => {
            if w == 0 || h == 0 {
                return Err(ConvertError::ResizeError(
                    "리사이즈 크기는 0보다 커야 합니다".to_string(),
                ));
            }
            if options.keep_aspect {
                fit_dimensions(orig_w, orig_h, w, h)
            } else {
                (w, h)
            }
        }
        (Some(w), None) => {
            if w == 0 {
                return Err(ConvertError::ResizeError(
                    "리사이즈 크기는 0보다 커야 합니다".to_string(),
                ));
            }
            let h = (orig_h as f64 * w as f64 / orig_w as f64).round() as u32;
            (w, h.max(1))
        }
        (None, Some(h)) => {
            if h == 0 {
                return Err(ConvertError::ResizeError(
                    "리사이즈 크기는 0보다 커야 합니다".to_string(),
                ));
            }
            let w = (orig_w as f64 * h as f64 / orig_h as f64).round() as u32;
            (w.max(1), h)
        }
        (None, None) => {
            return Err(ConvertError::ResizeError(
                "width 또는 height 중 하나 이상을 지정해야 합니다".to_string(),
            ));
        }
    };

    Ok(img.resize_exact(new_w, new_h, FilterType::Lanczos3))
}

/// 지정된 범위(max_w x max_h) 내에서 종횡비를 유지하며 맞추는 크기를 계산한다.
fn fit_dimensions(orig_w: u32, orig_h: u32, max_w: u32, max_h: u32) -> (u32, u32) {
    let ratio_w = max_w as f64 / orig_w as f64;
    let ratio_h = max_h as f64 / orig_h as f64;
    let ratio = ratio_w.min(ratio_h);

    let new_w = (orig_w as f64 * ratio).round() as u32;
    let new_h = (orig_h as f64 * ratio).round() as u32;

    (new_w.max(1), new_h.max(1))
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, RgbaImage};

    fn create_test_image(w: u32, h: u32) -> DynamicImage {
        DynamicImage::ImageRgba8(RgbaImage::new(w, h))
    }

    #[test]
    fn test_width_only_preserves_aspect_ratio() {
        let img = create_test_image(200, 100);
        let opts = ResizeOptions {
            width: Some(100),
            height: None,
            keep_aspect: false,
        };
        let result = resize_image(&img, &opts).unwrap();
        assert_eq!(result.width(), 100);
        assert_eq!(result.height(), 50);
    }

    #[test]
    fn test_height_only_preserves_aspect_ratio() {
        let img = create_test_image(200, 100);
        let opts = ResizeOptions {
            width: None,
            height: Some(50),
            keep_aspect: false,
        };
        let result = resize_image(&img, &opts).unwrap();
        assert_eq!(result.width(), 100);
        assert_eq!(result.height(), 50);
    }

    #[test]
    fn test_width_and_height_with_keep_aspect_fit() {
        let img = create_test_image(400, 200);
        let opts = ResizeOptions {
            width: Some(200),
            height: Some(200),
            keep_aspect: true,
        };
        let result = resize_image(&img, &opts).unwrap();
        // 400x200 → fit into 200x200 → ratio_w=0.5, ratio_h=1.0 → min=0.5 → 200x100
        assert_eq!(result.width(), 200);
        assert_eq!(result.height(), 100);
    }

    #[test]
    fn test_width_and_height_without_keep_aspect_stretch() {
        let img = create_test_image(400, 200);
        let opts = ResizeOptions {
            width: Some(300),
            height: Some(300),
            keep_aspect: false,
        };
        let result = resize_image(&img, &opts).unwrap();
        assert_eq!(result.width(), 300);
        assert_eq!(result.height(), 300);
    }

    #[test]
    fn test_no_dimensions_returns_error() {
        let img = create_test_image(100, 100);
        let opts = ResizeOptions {
            width: None,
            height: None,
            keep_aspect: false,
        };
        assert!(resize_image(&img, &opts).is_err());
    }

    #[test]
    fn test_zero_width_returns_error() {
        let img = create_test_image(100, 100);
        let opts = ResizeOptions {
            width: Some(0),
            height: None,
            keep_aspect: false,
        };
        assert!(resize_image(&img, &opts).is_err());
    }

    #[test]
    fn test_zero_height_returns_error() {
        let img = create_test_image(100, 100);
        let opts = ResizeOptions {
            width: None,
            height: Some(0),
            keep_aspect: false,
        };
        assert!(resize_image(&img, &opts).is_err());
    }

    #[test]
    fn test_fit_tall_image_into_wide_box() {
        let img = create_test_image(100, 400);
        let opts = ResizeOptions {
            width: Some(200),
            height: Some(200),
            keep_aspect: true,
        };
        let result = resize_image(&img, &opts).unwrap();
        // 100x400 → fit into 200x200 → ratio_w=2.0, ratio_h=0.5 → min=0.5 → 50x200
        assert_eq!(result.width(), 50);
        assert_eq!(result.height(), 200);
    }
}
