// imgconv - 이미지 회전/뒤집기 모듈

use image::DynamicImage;

use crate::error::ConvertError;

/// 회전 각도
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotateAngle {
    Rotate90,
    Rotate180,
    Rotate270,
}

impl RotateAngle {
    /// 정수 값으로부터 회전 각도를 파싱
    pub fn from_degrees(degrees: u32) -> Result<Self, ConvertError> {
        match degrees {
            90 => Ok(Self::Rotate90),
            180 => Ok(Self::Rotate180),
            270 => Ok(Self::Rotate270),
            other => Err(ConvertError::InvalidRotateAngle { angle: other }),
        }
    }
}

/// 뒤집기 방향
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlipDirection {
    Horizontal,
    Vertical,
}

impl FlipDirection {
    /// 문자열로부터 뒤집기 방향을 파싱
    pub fn parse(s: &str) -> Result<Self, ConvertError> {
        match s.to_lowercase().as_str() {
            "horizontal" | "h" => Ok(Self::Horizontal),
            "vertical" | "v" => Ok(Self::Vertical),
            other => Err(ConvertError::InvalidFlipDirection {
                direction: other.to_string(),
            }),
        }
    }
}

/// 이미지에 회전을 적용
pub fn apply_rotate(img: &DynamicImage, angle: RotateAngle) -> DynamicImage {
    match angle {
        RotateAngle::Rotate90 => img.rotate90(),
        RotateAngle::Rotate180 => img.rotate180(),
        RotateAngle::Rotate270 => img.rotate270(),
    }
}

/// 이미지에 뒤집기를 적용
pub fn apply_flip(img: &DynamicImage, direction: FlipDirection) -> DynamicImage {
    match direction {
        FlipDirection::Horizontal => img.fliph(),
        FlipDirection::Vertical => img.flipv(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, RgbaImage};

    fn test_image(w: u32, h: u32) -> DynamicImage {
        DynamicImage::ImageRgba8(RgbaImage::new(w, h))
    }

    #[test]
    fn rotate90_swaps_dimensions() {
        let img = test_image(100, 50);
        let rotated = apply_rotate(&img, RotateAngle::Rotate90);
        assert_eq!(rotated.width(), 50);
        assert_eq!(rotated.height(), 100);
    }

    #[test]
    fn rotate180_preserves_dimensions() {
        let img = test_image(100, 50);
        let rotated = apply_rotate(&img, RotateAngle::Rotate180);
        assert_eq!(rotated.width(), 100);
        assert_eq!(rotated.height(), 50);
    }

    #[test]
    fn rotate270_swaps_dimensions() {
        let img = test_image(100, 50);
        let rotated = apply_rotate(&img, RotateAngle::Rotate270);
        assert_eq!(rotated.width(), 50);
        assert_eq!(rotated.height(), 100);
    }

    #[test]
    fn flip_horizontal_preserves_dimensions() {
        let img = test_image(100, 50);
        let flipped = apply_flip(&img, FlipDirection::Horizontal);
        assert_eq!(flipped.width(), 100);
        assert_eq!(flipped.height(), 50);
    }

    #[test]
    fn flip_vertical_preserves_dimensions() {
        let img = test_image(100, 50);
        let flipped = apply_flip(&img, FlipDirection::Vertical);
        assert_eq!(flipped.width(), 100);
        assert_eq!(flipped.height(), 50);
    }

    #[test]
    fn from_degrees_valid() {
        assert_eq!(
            RotateAngle::from_degrees(90).unwrap(),
            RotateAngle::Rotate90
        );
        assert_eq!(
            RotateAngle::from_degrees(180).unwrap(),
            RotateAngle::Rotate180
        );
        assert_eq!(
            RotateAngle::from_degrees(270).unwrap(),
            RotateAngle::Rotate270
        );
    }

    #[test]
    fn from_degrees_invalid() {
        assert!(RotateAngle::from_degrees(45).is_err());
        assert!(RotateAngle::from_degrees(0).is_err());
        assert!(RotateAngle::from_degrees(360).is_err());
    }

    #[test]
    fn flip_direction_from_str_valid() {
        assert_eq!(
            FlipDirection::parse("horizontal").unwrap(),
            FlipDirection::Horizontal
        );
        assert_eq!(
            FlipDirection::parse("Horizontal").unwrap(),
            FlipDirection::Horizontal
        );
        assert_eq!(
            FlipDirection::parse("h").unwrap(),
            FlipDirection::Horizontal
        );
        assert_eq!(
            FlipDirection::parse("vertical").unwrap(),
            FlipDirection::Vertical
        );
        assert_eq!(FlipDirection::parse("V").unwrap(), FlipDirection::Vertical);
    }

    #[test]
    fn flip_direction_from_str_invalid() {
        assert!(FlipDirection::parse("diagonal").is_err());
        assert!(FlipDirection::parse("").is_err());
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use image::{DynamicImage, Rgba, RgbaImage};
    use proptest::prelude::*;

    fn test_image(w: u32, h: u32) -> DynamicImage {
        DynamicImage::ImageRgba8(RgbaImage::new(w, h))
    }

    /// Create a test image with unique pixel values so we can verify content preservation
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

    // Property 7: 회전 후 이미지 크기 — 90/270도 회전 시 너비/높이 교환, 180도 시 보존
    // **Validates: Requirements 24.1, 24.2, 24.3**
    proptest! {
        #[test]
        fn prop7_rotate_dimensions(
            w in 1u32..=500,
            h in 1u32..=500,
        ) {
            let img = test_image(w, h);

            // 90 degrees: width/height swap
            let r90 = apply_rotate(&img, RotateAngle::Rotate90);
            prop_assert_eq!(r90.width(), h, "90° rotation: width should be original height");
            prop_assert_eq!(r90.height(), w, "90° rotation: height should be original width");

            // 180 degrees: dimensions preserved
            let r180 = apply_rotate(&img, RotateAngle::Rotate180);
            prop_assert_eq!(r180.width(), w, "180° rotation: width should be preserved");
            prop_assert_eq!(r180.height(), h, "180° rotation: height should be preserved");

            // 270 degrees: width/height swap
            let r270 = apply_rotate(&img, RotateAngle::Rotate270);
            prop_assert_eq!(r270.width(), h, "270° rotation: width should be original height");
            prop_assert_eq!(r270.height(), w, "270° rotation: height should be original width");
        }
    }

    // Property 8: 뒤집기 라운드트립 — 같은 방향 두 번 뒤집기 시 원본과 동일
    // **Validates: Requirements 24.4, 24.5**
    proptest! {
        #[test]
        fn prop8_flip_roundtrip(
            w in 1u32..=100,
            h in 1u32..=100,
        ) {
            let img = test_image_with_pixels(w, h);

            // Horizontal flip twice = original
            let flipped_h1 = apply_flip(&img, FlipDirection::Horizontal);
            let flipped_h2 = apply_flip(&flipped_h1, FlipDirection::Horizontal);
            prop_assert_eq!(flipped_h2.width(), img.width());
            prop_assert_eq!(flipped_h2.height(), img.height());
            prop_assert_eq!(flipped_h2.as_bytes(), img.as_bytes(), "Double horizontal flip should return original");

            // Vertical flip twice = original
            let flipped_v1 = apply_flip(&img, FlipDirection::Vertical);
            let flipped_v2 = apply_flip(&flipped_v1, FlipDirection::Vertical);
            prop_assert_eq!(flipped_v2.width(), img.width());
            prop_assert_eq!(flipped_v2.height(), img.height());
            prop_assert_eq!(flipped_v2.as_bytes(), img.as_bytes(), "Double vertical flip should return original");
        }
    }

    // Property 9: 유효하지 않은 회전/뒤집기 값 거부 — 잘못된 값 시 에러 반환
    // **Validates: Requirements 24.6, 24.7**
    proptest! {
        #[test]
        fn prop9_invalid_rotate_angle_rejected(
            angle in (0u32..=360).prop_filter("not a valid angle", |a| *a != 90 && *a != 180 && *a != 270),
        ) {
            let result = RotateAngle::from_degrees(angle);
            prop_assert!(result.is_err(), "Angle {} should be rejected", angle);
            match result.unwrap_err() {
                ConvertError::InvalidRotateAngle { angle: a } => {
                    prop_assert_eq!(a, angle);
                }
                other => prop_assert!(false, "Expected InvalidRotateAngle, got: {:?}", other),
            }
        }

        #[test]
        fn prop9_invalid_flip_direction_rejected(
            direction in "[a-z]{1,20}".prop_filter("not a valid direction", |s| {
                let lower = s.to_lowercase();
                lower != "horizontal" && lower != "h" && lower != "vertical" && lower != "v"
            }),
        ) {
            let result = FlipDirection::parse(&direction);
            prop_assert!(result.is_err(), "Direction '{}' should be rejected", direction);
            match result.unwrap_err() {
                ConvertError::InvalidFlipDirection { .. } => {}
                other => prop_assert!(false, "Expected InvalidFlipDirection, got: {:?}", other),
            }
        }
    }
}
