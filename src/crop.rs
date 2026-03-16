// imgconv - 이미지 크롭 모듈

use image::DynamicImage;

use crate::error::ConvertError;

/// 크롭 옵션
#[derive(Debug, Clone, Copy)]
pub struct CropOptions {
    /// 시작 X 좌표
    pub x: u32,
    /// 시작 Y 좌표
    pub y: u32,
    /// 크롭 너비
    pub width: u32,
    /// 크롭 높이
    pub height: u32,
}

impl CropOptions {
    /// "x,y,w,h" 형식의 문자열을 파싱
    pub fn parse(s: &str) -> Result<Self, ConvertError> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 4 {
            return Err(ConvertError::InvalidCropFormat {
                input: s.to_string(),
            });
        }

        let parse = |part: &str| {
            part.trim()
                .parse::<u32>()
                .map_err(|_| ConvertError::InvalidCropFormat {
                    input: s.to_string(),
                })
        };

        Ok(Self {
            x: parse(parts[0])?,
            y: parse(parts[1])?,
            width: parse(parts[2])?,
            height: parse(parts[3])?,
        })
    }

    /// 이미지 크기에 대해 크롭 영역이 유효한지 검증
    pub fn validate(&self, image_width: u32, image_height: u32) -> Result<(), ConvertError> {
        if self.x + self.width > image_width || self.y + self.height > image_height {
            return Err(ConvertError::CropOutOfBounds {
                x: self.x,
                y: self.y,
                w: self.width,
                h: self.height,
                img_w: image_width,
                img_h: image_height,
            });
        }
        Ok(())
    }
}

/// 이미지에 크롭을 적용
pub fn apply_crop(img: &DynamicImage, options: &CropOptions) -> Result<DynamicImage, ConvertError> {
    options.validate(img.width(), img.height())?;
    Ok(img.crop_imm(options.x, options.y, options.width, options.height))
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, RgbaImage};

    fn test_image(w: u32, h: u32) -> DynamicImage {
        DynamicImage::ImageRgba8(RgbaImage::new(w, h))
    }

    #[test]
    fn from_str_valid() {
        let opts = CropOptions::parse("10,20,100,200").unwrap();
        assert_eq!(
            (opts.x, opts.y, opts.width, opts.height),
            (10, 20, 100, 200)
        );
    }

    #[test]
    fn from_str_with_spaces() {
        let opts = CropOptions::parse(" 5 , 10 , 50 , 60 ").unwrap();
        assert_eq!((opts.x, opts.y, opts.width, opts.height), (5, 10, 50, 60));
    }

    #[test]
    fn from_str_too_few_parts() {
        assert!(CropOptions::parse("10,20,100").is_err());
    }

    #[test]
    fn from_str_too_many_parts() {
        assert!(CropOptions::parse("10,20,100,200,5").is_err());
    }

    #[test]
    fn from_str_non_numeric() {
        assert!(CropOptions::parse("a,b,c,d").is_err());
    }

    #[test]
    fn from_str_negative() {
        assert!(CropOptions::parse("-1,0,10,10").is_err());
    }

    #[test]
    fn validate_within_bounds() {
        let opts = CropOptions {
            x: 0,
            y: 0,
            width: 50,
            height: 50,
        };
        assert!(opts.validate(100, 100).is_ok());
    }

    #[test]
    fn validate_exact_bounds() {
        let opts = CropOptions {
            x: 50,
            y: 50,
            width: 50,
            height: 50,
        };
        assert!(opts.validate(100, 100).is_ok());
    }

    #[test]
    fn validate_exceeds_width() {
        let opts = CropOptions {
            x: 60,
            y: 0,
            width: 50,
            height: 50,
        };
        assert!(opts.validate(100, 100).is_err());
    }

    #[test]
    fn validate_exceeds_height() {
        let opts = CropOptions {
            x: 0,
            y: 60,
            width: 50,
            height: 50,
        };
        assert!(opts.validate(100, 100).is_err());
    }

    #[test]
    fn apply_crop_result_size() {
        let img = test_image(200, 200);
        let opts = CropOptions {
            x: 10,
            y: 10,
            width: 80,
            height: 60,
        };
        let cropped = apply_crop(&img, &opts).unwrap();
        assert_eq!(cropped.width(), 80);
        assert_eq!(cropped.height(), 60);
    }

    #[test]
    fn apply_crop_out_of_bounds() {
        let img = test_image(100, 100);
        let opts = CropOptions {
            x: 50,
            y: 50,
            width: 60,
            height: 60,
        };
        assert!(apply_crop(&img, &opts).is_err());
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use image::{DynamicImage, RgbaImage};
    use proptest::prelude::*;

    fn test_image(w: u32, h: u32) -> DynamicImage {
        DynamicImage::ImageRgba8(RgbaImage::new(w, h))
    }

    // Property 3: 크롭 결과 크기 — 유효한 크롭 영역의 결과 이미지 크기가 w, h와 정확히 일치
    // Validates: 요구사항 23.1
    proptest! {
        #[test]
        fn prop3_crop_result_dimensions(
            img_w in 1u32..=500,
            img_h in 1u32..=500,
        ) {
            // Generate valid crop region within image bounds
            let max_x = img_w.saturating_sub(1);
            let max_y = img_h.saturating_sub(1);

            // Pick a random starting point, then a valid size
            let x = max_x / 2;
            let y = max_y / 2;
            let crop_w = ((img_w - x) / 2).max(1);
            let crop_h = ((img_h - y) / 2).max(1);

            let img = test_image(img_w, img_h);
            let opts = CropOptions { x, y, width: crop_w, height: crop_h };
            let result = apply_crop(&img, &opts).unwrap();

            prop_assert_eq!(result.width(), crop_w);
            prop_assert_eq!(result.height(), crop_h);
        }
    }

    // Property 5: 크롭 범위 초과 시 에러 — 이미지 범위 초과 크롭 시 CropOutOfBounds 에러
    // Validates: 요구사항 23.3
    proptest! {
        #[test]
        fn prop5_crop_out_of_bounds_error(
            img_w in 1u32..=200,
            img_h in 1u32..=200,
            overflow in 1u32..=100,
        ) {
            let img = test_image(img_w, img_h);

            // Crop that exceeds width
            let opts_w = CropOptions { x: 0, y: 0, width: img_w + overflow, height: 1 };
            let err_w = apply_crop(&img, &opts_w);
            prop_assert!(err_w.is_err());
            match err_w.unwrap_err() {
                ConvertError::CropOutOfBounds { .. } => {}
                other => prop_assert!(false, "Expected CropOutOfBounds, got: {:?}", other),
            }

            // Crop that exceeds height
            let opts_h = CropOptions { x: 0, y: 0, width: 1, height: img_h + overflow };
            let err_h = apply_crop(&img, &opts_h);
            prop_assert!(err_h.is_err());
            match err_h.unwrap_err() {
                ConvertError::CropOutOfBounds { .. } => {}
                other => prop_assert!(false, "Expected CropOutOfBounds, got: {:?}", other),
            }
        }
    }

    // Property 6: 크롭 형식 파싱 — 잘못된 형식 문자열 시 InvalidCropFormat 에러
    // Validates: 요구사항 23.4
    proptest! {
        #[test]
        fn prop6_invalid_crop_format_error(
            // Generate strings with wrong number of comma-separated parts
            parts_count in (0usize..=3).prop_union(5usize..=8),
        ) {
            let input: String = (0..parts_count).map(|i| i.to_string()).collect::<Vec<_>>().join(",");
            let result = CropOptions::parse(&input);
            prop_assert!(result.is_err());
            match result.unwrap_err() {
                ConvertError::InvalidCropFormat { .. } => {}
                other => prop_assert!(false, "Expected InvalidCropFormat, got: {:?}", other),
            }
        }

        #[test]
        fn prop6_non_numeric_crop_format_error(
            a in "[a-z]+",
            b in "[a-z]+",
            c in "[a-z]+",
            d in "[a-z]+",
        ) {
            let input = format!("{},{},{},{}", a, b, c, d);
            let result = CropOptions::parse(&input);
            prop_assert!(result.is_err());
            match result.unwrap_err() {
                ConvertError::InvalidCropFormat { .. } => {}
                other => prop_assert!(false, "Expected InvalidCropFormat, got: {:?}", other),
            }
        }
    }
}
