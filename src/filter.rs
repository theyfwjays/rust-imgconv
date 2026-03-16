// imgconv - 색상 필터, 블러/샤프닝, 밝기/대비/감마 모듈

use image::DynamicImage;

use crate::error::ConvertError;

/// 색상 필터 옵션
#[derive(Debug, Clone, Copy, Default)]
pub struct ColorFilterOptions {
    pub grayscale: bool,
    pub invert: bool,
    pub sepia: bool,
}

impl ColorFilterOptions {
    /// 어떤 필터도 활성화되지 않았는지 확인
    pub fn is_none(&self) -> bool {
        !self.grayscale && !self.invert && !self.sepia
    }
}

/// 밝기/대비/감마 옵션
#[derive(Debug, Clone, Copy, Default)]
pub struct BrightnessContrastOptions {
    /// 밝기 조정 (양수: 밝게, 음수: 어둡게)
    pub brightness: Option<i32>,
    /// 대비 조정
    pub contrast: Option<f32>,
    /// 감마 보정 (양수만 허용)
    pub gamma: Option<f32>,
}

/// 색상 필터 적용 (grayscale → sepia → invert 순서)
///
/// 요구사항 25.4에 따라 여러 필터가 동시에 지정되면
/// grayscale → sepia → invert 순서로 적용한다.
pub fn apply_color_filters(img: &DynamicImage, options: &ColorFilterOptions) -> DynamicImage {
    if options.is_none() {
        return img.clone();
    }

    let mut result = img.clone();

    // 1. 그레이스케일 (요구사항 25.1)
    if options.grayscale {
        result = result.grayscale();
    }

    // 2. 세피아 (요구사항 25.3)
    if options.sepia {
        result = apply_sepia(&result);
    }

    // 3. 색상 반전 (요구사항 25.2) - in-place
    if options.invert {
        result.invert();
    }

    result
}

/// 세피아 톤 변환을 적용한다.
///
/// 표준 세피아 공식:
/// - new_r = min(255, 0.393*r + 0.769*g + 0.189*b)
/// - new_g = min(255, 0.349*r + 0.686*g + 0.168*b)
/// - new_b = min(255, 0.272*r + 0.534*g + 0.131*b)
fn apply_sepia(img: &DynamicImage) -> DynamicImage {
    let mut rgba = img.to_rgba8();

    for pixel in rgba.pixels_mut() {
        let r = pixel[0] as f64;
        let g = pixel[1] as f64;
        let b = pixel[2] as f64;

        let new_r = (0.393 * r + 0.769 * g + 0.189 * b).min(255.0) as u8;
        let new_g = (0.349 * r + 0.686 * g + 0.168 * b).min(255.0) as u8;
        let new_b = (0.272 * r + 0.534 * g + 0.131 * b).min(255.0) as u8;

        pixel[0] = new_r;
        pixel[1] = new_g;
        pixel[2] = new_b;
        // alpha channel preserved
    }

    DynamicImage::ImageRgba8(rgba)
}

/// 가우시안 블러 적용
///
/// sigma 값은 양수여야 한다. 0 이하이면 `InvalidBlurSigma` 에러를 반환한다.
/// 요구사항 26.1, 26.3
pub fn apply_blur(img: &DynamicImage, sigma: f32) -> Result<DynamicImage, ConvertError> {
    if sigma <= 0.0 {
        return Err(ConvertError::InvalidBlurSigma { value: sigma });
    }
    Ok(img.blur(sigma))
}

/// 언샤프 마스크 적용 (샤프닝)
///
/// value 값은 양수여야 한다. 0 이하이면 `InvalidSharpenValue` 에러를 반환한다.
/// 요구사항 26.2, 26.4
pub fn apply_sharpen(img: &DynamicImage, value: f32) -> Result<DynamicImage, ConvertError> {
    if value <= 0.0 {
        return Err(ConvertError::InvalidSharpenValue { value });
    }
    // unsharpen(sigma, threshold): sigma controls blur amount, threshold controls edge detection
    // Using value as sigma with threshold 1 for standard unsharp mask behavior
    Ok(img.unsharpen(value, 1))
}

/// 밝기/대비/감마 적용 (brightness → contrast → gamma 순서)
///
/// 모든 옵션이 None이면 원본 이미지를 그대로 반환한다.
/// 감마 값이 0 이하이면 `InvalidGamma` 에러를 반환한다.
/// 요구사항 30.1, 30.2, 30.3, 30.4, 30.5
pub fn apply_brightness_contrast(
    img: &DynamicImage,
    options: &BrightnessContrastOptions,
) -> Result<DynamicImage, ConvertError> {
    // 모든 옵션이 None이면 클론 반환
    if options.brightness.is_none() && options.contrast.is_none() && options.gamma.is_none() {
        return Ok(img.clone());
    }

    // 감마 값 유효성 검증 (양수만 허용)
    if let Some(gamma) = options.gamma {
        if gamma <= 0.0 {
            return Err(ConvertError::InvalidGamma { value: gamma });
        }
    }

    let mut result = img.clone();

    // 1. 밝기 (요구사항 30.1)
    if let Some(brightness) = options.brightness {
        result = result.brighten(brightness);
    }

    // 2. 대비 (요구사항 30.2)
    if let Some(contrast) = options.contrast {
        result = result.adjust_contrast(contrast);
    }

    // 3. 감마 (요구사항 30.3) - 픽셀별 감마 보정
    if let Some(gamma) = options.gamma {
        result = apply_gamma(&result, gamma);
    }

    Ok(result)
}

/// 픽셀별 감마 보정을 적용한다.
///
/// 공식: new_pixel = 255 * (pixel / 255) ^ (1 / gamma)
fn apply_gamma(img: &DynamicImage, gamma: f32) -> DynamicImage {
    let inv_gamma = 1.0 / gamma;
    let mut rgba = img.to_rgba8();

    for pixel in rgba.pixels_mut() {
        pixel[0] = (255.0 * (pixel[0] as f32 / 255.0).powf(inv_gamma))
            .round()
            .clamp(0.0, 255.0) as u8;
        pixel[1] = (255.0 * (pixel[1] as f32 / 255.0).powf(inv_gamma))
            .round()
            .clamp(0.0, 255.0) as u8;
        pixel[2] = (255.0 * (pixel[2] as f32 / 255.0).powf(inv_gamma))
            .round()
            .clamp(0.0, 255.0) as u8;
        // alpha channel preserved
    }

    DynamicImage::ImageRgba8(rgba)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{Rgba, RgbaImage};

    /// 테스트용 이미지 생성
    fn create_test_image(w: u32, h: u32) -> DynamicImage {
        let mut img = RgbaImage::new(w, h);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            *pixel = Rgba([
                (x * 17 % 256) as u8,
                (y * 31 % 256) as u8,
                ((x + y) * 13 % 256) as u8,
                255,
            ]);
        }
        DynamicImage::ImageRgba8(img)
    }

    #[test]
    fn no_filters_returns_clone() {
        let img = create_test_image(4, 4);
        let options = ColorFilterOptions::default();
        let result = apply_color_filters(&img, &options);
        assert_eq!(img.to_rgba8().into_raw(), result.to_rgba8().into_raw());
    }

    #[test]
    fn grayscale_makes_rgb_equal() {
        let img = create_test_image(8, 8);
        let options = ColorFilterOptions {
            grayscale: true,
            invert: false,
            sepia: false,
        };
        let result = apply_color_filters(&img, &options);
        let rgba = result.to_rgba8();
        for pixel in rgba.pixels() {
            // After grayscale, R, G, B should be equal (or very close)
            // image crate grayscale converts to luma then back, so R=G=B
            assert_eq!(pixel[0], pixel[1], "R != G after grayscale");
            assert_eq!(pixel[1], pixel[2], "G != B after grayscale");
        }
    }

    #[test]
    fn invert_roundtrip() {
        let img = create_test_image(8, 8);
        let options = ColorFilterOptions {
            grayscale: false,
            invert: true,
            sepia: false,
        };
        // Apply invert twice
        let once = apply_color_filters(&img, &options);
        let twice = apply_color_filters(&once, &options);
        assert_eq!(img.to_rgba8().into_raw(), twice.to_rgba8().into_raw());
    }

    #[test]
    fn sepia_produces_warm_tones() {
        // Create a pure white pixel image
        let mut white_img = RgbaImage::new(1, 1);
        white_img.put_pixel(0, 0, Rgba([255, 255, 255, 255]));
        let img = DynamicImage::ImageRgba8(white_img);

        let options = ColorFilterOptions {
            grayscale: false,
            invert: false,
            sepia: true,
        };
        let result = apply_color_filters(&img, &options);
        let rgba = result.to_rgba8();
        let pixel = rgba.get_pixel(0, 0);

        // Sepia on white: R >= G >= B
        assert!(pixel[0] >= pixel[1], "sepia: R should >= G");
        assert!(pixel[1] >= pixel[2], "sepia: G should >= B");
    }

    #[test]
    fn sepia_formula_correctness() {
        // Test with a known pixel value
        let mut img = RgbaImage::new(1, 1);
        img.put_pixel(0, 0, Rgba([100, 150, 200, 255]));
        let dyn_img = DynamicImage::ImageRgba8(img);

        let result = apply_sepia(&dyn_img);
        let rgba = result.to_rgba8();
        let pixel = rgba.get_pixel(0, 0);

        // Manual calculation:
        // new_r = min(255, 0.393*100 + 0.769*150 + 0.189*200) = min(255, 39.3 + 115.35 + 37.8) = min(255, 192.45) = 192
        // new_g = min(255, 0.349*100 + 0.686*150 + 0.168*200) = min(255, 34.9 + 102.9 + 33.6) = min(255, 171.4) = 171
        // new_b = min(255, 0.272*100 + 0.534*150 + 0.131*200) = min(255, 27.2 + 80.1 + 26.2) = min(255, 133.5) = 133
        assert_eq!(pixel[0], 192, "sepia R");
        assert_eq!(pixel[1], 171, "sepia G");
        assert_eq!(pixel[2], 133, "sepia B");
        assert_eq!(pixel[3], 255, "alpha preserved");
    }

    #[test]
    fn combined_filters_apply_in_order() {
        let img = create_test_image(4, 4);
        // All filters enabled: grayscale → sepia → invert
        let options = ColorFilterOptions {
            grayscale: true,
            invert: true,
            sepia: true,
        };
        let result = apply_color_filters(&img, &options);
        // Just verify it doesn't panic and produces valid output
        assert_eq!(result.width(), img.width());
        assert_eq!(result.height(), img.height());
    }

    #[test]
    fn is_none_works() {
        assert!(ColorFilterOptions::default().is_none());
        assert!(!ColorFilterOptions {
            grayscale: true,
            ..Default::default()
        }
        .is_none());
        assert!(!ColorFilterOptions {
            invert: true,
            ..Default::default()
        }
        .is_none());
        assert!(!ColorFilterOptions {
            sepia: true,
            ..Default::default()
        }
        .is_none());
    }

    #[test]
    fn brightness_contrast_all_none_returns_clone() {
        let img = create_test_image(4, 4);
        let options = BrightnessContrastOptions::default();
        let result = apply_brightness_contrast(&img, &options).unwrap();
        assert_eq!(img.to_rgba8().into_raw(), result.to_rgba8().into_raw());
    }

    #[test]
    fn brightness_zero_is_identity() {
        let img = create_test_image(8, 8);
        let options = BrightnessContrastOptions {
            brightness: Some(0),
            contrast: None,
            gamma: None,
        };
        let result = apply_brightness_contrast(&img, &options).unwrap();
        assert_eq!(img.to_rgba8().into_raw(), result.to_rgba8().into_raw());
    }

    #[test]
    fn gamma_one_is_identity() {
        let img = create_test_image(8, 8);
        let options = BrightnessContrastOptions {
            brightness: None,
            contrast: None,
            gamma: Some(1.0),
        };
        let result = apply_brightness_contrast(&img, &options).unwrap();
        assert_eq!(img.to_rgba8().into_raw(), result.to_rgba8().into_raw());
    }

    #[test]
    fn invalid_gamma_returns_error() {
        let img = create_test_image(4, 4);

        // gamma = 0
        let options = BrightnessContrastOptions {
            brightness: None,
            contrast: None,
            gamma: Some(0.0),
        };
        let result = apply_brightness_contrast(&img, &options);
        assert!(result.is_err());
        match result.unwrap_err() {
            ConvertError::InvalidGamma { value } => assert_eq!(value, 0.0),
            other => panic!("Expected InvalidGamma, got {:?}", other),
        }

        // gamma = -1.5
        let options = BrightnessContrastOptions {
            brightness: None,
            contrast: None,
            gamma: Some(-1.5),
        };
        let result = apply_brightness_contrast(&img, &options);
        assert!(result.is_err());
        match result.unwrap_err() {
            ConvertError::InvalidGamma { value } => assert_eq!(value, -1.5),
            other => panic!("Expected InvalidGamma, got {:?}", other),
        }
    }

    #[test]
    fn brightness_contrast_gamma_combined() {
        let img = create_test_image(8, 8);
        let options = BrightnessContrastOptions {
            brightness: Some(10),
            contrast: Some(5.0),
            gamma: Some(2.0),
        };
        let result = apply_brightness_contrast(&img, &options).unwrap();
        // Just verify it doesn't panic and produces valid output
        assert_eq!(result.width(), img.width());
        assert_eq!(result.height(), img.height());
    }

    #[test]
    fn gamma_preserves_alpha() {
        let mut img = RgbaImage::new(1, 1);
        img.put_pixel(0, 0, Rgba([128, 128, 128, 100]));
        let dyn_img = DynamicImage::ImageRgba8(img);

        let options = BrightnessContrastOptions {
            brightness: None,
            contrast: None,
            gamma: Some(2.2),
        };
        let result = apply_brightness_contrast(&dyn_img, &options).unwrap();
        let rgba = result.to_rgba8();
        let pixel = rgba.get_pixel(0, 0);
        assert_eq!(pixel[3], 100, "alpha should be preserved");
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use image::{DynamicImage, Rgba, RgbaImage};
    use proptest::prelude::*;

    /// 랜덤 픽셀 값을 가진 테스트 이미지 생성
    fn test_image_with_pixels(w: u32, h: u32) -> DynamicImage {
        let mut img = RgbaImage::new(w, h);
        for y in 0..h {
            for x in 0..w {
                let r = ((x * 17 + y * 31) % 256) as u8;
                let g = ((x * 13 + y * 7) % 256) as u8;
                let b = ((x * 5 + y * 23) % 256) as u8;
                img.put_pixel(x, y, Rgba([r, g, b, 255]));
            }
        }
        DynamicImage::ImageRgba8(img)
    }

    // Property 10: 그레이스케일 픽셀 균일성 — 그레이스케일 후 모든 픽셀의 R=G=B
    // **Validates: Requirements 25.1**
    proptest! {
        #[test]
        fn prop10_grayscale_pixel_uniformity(
            w in 1u32..=100,
            h in 1u32..=100,
        ) {
            let img = test_image_with_pixels(w, h);
            let options = ColorFilterOptions {
                grayscale: true,
                invert: false,
                sepia: false,
            };
            let result = apply_color_filters(&img, &options);
            let rgba = result.to_rgba8();

            for pixel in rgba.pixels() {
                prop_assert_eq!(pixel[0], pixel[1], "R != G after grayscale");
                prop_assert_eq!(pixel[1], pixel[2], "G != B after grayscale");
            }
        }
    }

    // Property 11: 색상 반전 라운드트립 — 두 번 반전 시 원본과 동일
    // **Validates: Requirements 25.2**
    proptest! {
        #[test]
        fn prop11_invert_roundtrip(
            w in 1u32..=100,
            h in 1u32..=100,
        ) {
            let img = test_image_with_pixels(w, h);
            let options = ColorFilterOptions {
                grayscale: false,
                invert: true,
                sepia: false,
            };

            // Apply invert twice
            let once = apply_color_filters(&img, &options);
            let twice = apply_color_filters(&once, &options);

            prop_assert_eq!(
                img.to_rgba8().into_raw(),
                twice.to_rgba8().into_raw(),
                "Double invert should return original pixel data"
            );
        }
    }

    // Property 12: 블러/샤프닝 크기 보존 — 적용 후 이미지 크기 동일
    // **Validates: Requirements 26.1, 26.2**
    proptest! {
        #[test]
        fn prop12_blur_sharpen_preserves_dimensions(
            w in 1u32..=50,
            h in 1u32..=50,
            sigma in 0.1f32..=10.0,
            sharpen_val in 0.1f32..=10.0,
        ) {
            let img = test_image_with_pixels(w, h);

            // Blur should preserve dimensions
            let blurred = apply_blur(&img, sigma).unwrap();
            prop_assert_eq!(blurred.width(), w, "Blur changed width");
            prop_assert_eq!(blurred.height(), h, "Blur changed height");

            // Sharpen should preserve dimensions
            let sharpened = apply_sharpen(&img, sharpen_val).unwrap();
            prop_assert_eq!(sharpened.width(), w, "Sharpen changed width");
            prop_assert_eq!(sharpened.height(), h, "Sharpen changed height");
        }
    }

    // Property 13: 유효하지 않은 블러/샤프닝 값 거부 — 0 이하 값 시 에러
    // **Validates: Requirements 26.3, 26.4**
    proptest! {
        #[test]
        fn prop13_invalid_blur_sharpen_rejected(
            w in 1u32..=50,
            h in 1u32..=50,
            bad_sigma in -10.0f32..=0.0,
            bad_sharpen in -10.0f32..=0.0,
        ) {
            let img = test_image_with_pixels(w, h);

            // Blur with non-positive sigma should return InvalidBlurSigma
            let blur_result = apply_blur(&img, bad_sigma);
            prop_assert!(blur_result.is_err(), "Blur should reject sigma={}", bad_sigma);
            match blur_result.unwrap_err() {
                ConvertError::InvalidBlurSigma { value } => {
                    prop_assert_eq!(value, bad_sigma);
                }
                other => prop_assert!(false, "Expected InvalidBlurSigma, got {:?}", other),
            }

            // Sharpen with non-positive value should return InvalidSharpenValue
            let sharpen_result = apply_sharpen(&img, bad_sharpen);
            prop_assert!(sharpen_result.is_err(), "Sharpen should reject value={}", bad_sharpen);
            match sharpen_result.unwrap_err() {
                ConvertError::InvalidSharpenValue { value } => {
                    prop_assert_eq!(value, bad_sharpen);
                }
                other => prop_assert!(false, "Expected InvalidSharpenValue, got {:?}", other),
            }
        }
    }

    // Property 18: 밝기 0 항등성 — 밝기 0 조정 시 원본과 동일
    // **Validates: Requirements 30.1**
    proptest! {
        #[test]
        fn prop18_brightness_zero_identity(
            w in 1u32..=50,
            h in 1u32..=50,
        ) {
            let img = test_image_with_pixels(w, h);
            let options = BrightnessContrastOptions {
                brightness: Some(0),
                contrast: None,
                gamma: None,
            };
            let result = apply_brightness_contrast(&img, &options).unwrap();
            prop_assert_eq!(
                img.to_rgba8().into_raw(),
                result.to_rgba8().into_raw(),
                "Brightness 0 should be identity"
            );
        }
    }

    // Property 19: 감마 1.0 항등성 — 감마 1.0 보정 시 원본과 동일
    // **Validates: Requirements 30.3**
    proptest! {
        #[test]
        fn prop19_gamma_one_identity(
            w in 1u32..=50,
            h in 1u32..=50,
        ) {
            let img = test_image_with_pixels(w, h);
            let options = BrightnessContrastOptions {
                brightness: None,
                contrast: None,
                gamma: Some(1.0),
            };
            let result = apply_brightness_contrast(&img, &options).unwrap();
            prop_assert_eq!(
                img.to_rgba8().into_raw(),
                result.to_rgba8().into_raw(),
                "Gamma 1.0 should be identity"
            );
        }
    }

    // Property 20: 유효하지 않은 감마 값 거부 — 0 이하 감마 시 `InvalidGamma` 에러
    // **Validates: Requirements 30.4**
    proptest! {
        #[test]
        fn prop20_invalid_gamma_rejected(
            w in 1u32..=50,
            h in 1u32..=50,
            bad_gamma in -10.0f32..=0.0,
        ) {
            let img = test_image_with_pixels(w, h);
            let options = BrightnessContrastOptions {
                brightness: None,
                contrast: None,
                gamma: Some(bad_gamma),
            };
            let result = apply_brightness_contrast(&img, &options);
            prop_assert!(result.is_err(), "Gamma should reject value={}", bad_gamma);
            match result.unwrap_err() {
                ConvertError::InvalidGamma { value } => {
                    prop_assert_eq!(value, bad_gamma);
                }
                other => prop_assert!(false, "Expected InvalidGamma, got {:?}", other),
            }
        }
    }
}
