// imgconv - 변환 프리셋 모듈

use crate::convert::ConvertOptions;
use crate::error::ConvertError;
use crate::format::ImageFormat;
use crate::resize::ResizeOptions;
use crate::webp::WebPMode;

/// 프리셋 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Preset {
    Web,
    Thumbnail,
    Print,
    Social,
}

impl Preset {
    /// 문자열로부터 프리셋 파싱
    pub fn parse(s: &str) -> Result<Self, ConvertError> {
        match s.to_lowercase().as_str() {
            "web" => Ok(Preset::Web),
            "thumbnail" => Ok(Preset::Thumbnail),
            "print" => Ok(Preset::Print),
            "social" => Ok(Preset::Social),
            _ => Err(ConvertError::InvalidPreset {
                name: s.to_string(),
            }),
        }
    }
}

/// 프리셋 설정
#[derive(Debug, Clone)]
pub struct PresetConfig {
    pub format: ImageFormat,
    pub quality: Option<u8>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub keep_aspect: bool,
    pub webp_mode: Option<WebPMode>,
    pub dpi: Option<f32>,
}

/// 프리셋에 해당하는 설정 반환
pub fn get_preset_config(preset: Preset) -> PresetConfig {
    match preset {
        Preset::Web => PresetConfig {
            format: ImageFormat::WebP,
            quality: Some(80),
            width: Some(1920),
            height: None,
            keep_aspect: true,
            webp_mode: Some(WebPMode::Lossy),
            dpi: None,
        },
        Preset::Thumbnail => PresetConfig {
            format: ImageFormat::Jpeg,
            quality: Some(70),
            width: Some(200),
            height: Some(200),
            keep_aspect: true,
            webp_mode: None,
            dpi: None,
        },
        Preset::Print => PresetConfig {
            format: ImageFormat::Tiff,
            quality: None,
            width: None,
            height: None,
            keep_aspect: false,
            webp_mode: None,
            dpi: Some(300.0),
        },
        Preset::Social => PresetConfig {
            format: ImageFormat::Jpeg,
            quality: Some(85),
            width: Some(1200),
            height: Some(630),
            keep_aspect: false,
            webp_mode: None,
            dpi: None,
        },
    }
}

/// 프리셋 설정을 ConvertOptions에 적용 (개별 옵션이 프리셋을 덮어씀)
pub fn apply_preset(preset: Preset, options: &mut ConvertOptions) {
    let config = get_preset_config(preset);

    // 포맷: target_formats가 비어있을 때만 프리셋 포맷 적용
    if options.target_formats.is_empty() {
        options.target_formats = vec![config.format];
    }

    // 품질: 개별 옵션이 없을 때만 프리셋 값 적용
    if options.quality.is_none() {
        options.quality = config.quality;
    }

    // WebP 모드: 프리셋에 webp_mode가 있고 사용자가 명시적으로 설정하지 않은 경우 적용
    if let Some(mode) = config.webp_mode {
        // WebPMode의 기본값은 Lossy이므로, 사용자가 명시적으로 lossless를 설정하지 않았다면 프리셋 적용
        // 프리셋은 기본값 역할이므로 항상 설정 (개별 옵션이 나중에 덮어씀)
        if options.webp_mode == WebPMode::default() {
            options.webp_mode = mode;
        }
    }

    // 리사이즈: 개별 옵션이 없을 때만 프리셋 값 적용
    if options.resize.is_none() && (config.width.is_some() || config.height.is_some()) {
        options.resize = Some(ResizeOptions {
            width: config.width,
            height: config.height,
            keep_aspect: config.keep_aspect,
        });
    }

    // DPI: SVG 옵션의 dpi가 기본값(96.0)일 때만 프리셋 값 적용
    if let Some(dpi) = config.dpi {
        if (options.svg_options.dpi - 96.0).abs() < f32::EPSILON {
            options.svg_options.dpi = dpi;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::convert::ConvertOptions;
    use crate::filter::{BrightnessContrastOptions, ColorFilterOptions};
    use crate::svg::SvgOptions;

    fn default_options() -> ConvertOptions {
        ConvertOptions {
            target_formats: vec![],
            quality: None,
            resize: None,
            crop: None,
            rotate: None,
            flip: None,
            color_filter: ColorFilterOptions::default(),
            brightness_contrast: BrightnessContrastOptions::default(),
            blur: None,
            sharpen: None,
            watermark: None,
            watermark_position: None,
            watermark_opacity: None,
            watermark_font: None,
            overlay: None,
            overlay_position: None,
            overlay_opacity: None,
            auto_orient: false,
            preserve_exif: false,
            preset: None,
            skip_identical: false,
            webp_mode: WebPMode::default(),
            svg_options: SvgOptions::default(),
            output_dir: None,
            overwrite: false,
            dry_run: false,
            verbose: false,
        }
    }

    #[test]
    fn preset_from_str_valid() {
        assert_eq!(Preset::parse("web").unwrap(), Preset::Web);
        assert_eq!(Preset::parse("Web").unwrap(), Preset::Web);
        assert_eq!(Preset::parse("WEB").unwrap(), Preset::Web);
        assert_eq!(Preset::parse("thumbnail").unwrap(), Preset::Thumbnail);
        assert_eq!(Preset::parse("print").unwrap(), Preset::Print);
        assert_eq!(Preset::parse("social").unwrap(), Preset::Social);
    }

    #[test]
    fn preset_from_str_invalid() {
        let err = Preset::parse("unknown").unwrap_err();
        match err {
            ConvertError::InvalidPreset { name } => assert_eq!(name, "unknown"),
            _ => panic!("expected InvalidPreset error"),
        }
    }

    #[test]
    fn web_preset_config() {
        let config = get_preset_config(Preset::Web);
        assert_eq!(config.format, ImageFormat::WebP);
        assert_eq!(config.quality, Some(80));
        assert_eq!(config.width, Some(1920));
        assert_eq!(config.height, None);
        assert!(config.keep_aspect);
        assert_eq!(config.webp_mode, Some(WebPMode::Lossy));
        assert_eq!(config.dpi, None);
    }

    #[test]
    fn thumbnail_preset_config() {
        let config = get_preset_config(Preset::Thumbnail);
        assert_eq!(config.format, ImageFormat::Jpeg);
        assert_eq!(config.quality, Some(70));
        assert_eq!(config.width, Some(200));
        assert_eq!(config.height, Some(200));
        assert!(config.keep_aspect);
    }

    #[test]
    fn print_preset_config() {
        let config = get_preset_config(Preset::Print);
        assert_eq!(config.format, ImageFormat::Tiff);
        assert_eq!(config.quality, None);
        assert_eq!(config.dpi, Some(300.0));
    }

    #[test]
    fn social_preset_config() {
        let config = get_preset_config(Preset::Social);
        assert_eq!(config.format, ImageFormat::Jpeg);
        assert_eq!(config.quality, Some(85));
        assert_eq!(config.width, Some(1200));
        assert_eq!(config.height, Some(630));
        assert!(!config.keep_aspect);
    }

    #[test]
    fn apply_preset_sets_defaults() {
        let mut opts = default_options();
        apply_preset(Preset::Web, &mut opts);

        assert_eq!(opts.target_formats, vec![ImageFormat::WebP]);
        assert_eq!(opts.quality, Some(80));
        assert_eq!(opts.webp_mode, WebPMode::Lossy);
        let resize = opts.resize.unwrap();
        assert_eq!(resize.width, Some(1920));
        assert_eq!(resize.height, None);
        assert!(resize.keep_aspect);
    }

    #[test]
    fn apply_preset_individual_options_override() {
        let mut opts = default_options();
        // 개별 옵션 먼저 설정
        opts.target_formats = vec![ImageFormat::Png];
        opts.quality = Some(50);
        opts.resize = Some(ResizeOptions {
            width: Some(800),
            height: None,
            keep_aspect: false,
        });

        apply_preset(Preset::Web, &mut opts);

        // 개별 옵션이 프리셋을 덮어써야 함
        assert_eq!(opts.target_formats, vec![ImageFormat::Png]);
        assert_eq!(opts.quality, Some(50));
        let resize = opts.resize.unwrap();
        assert_eq!(resize.width, Some(800));
    }

    #[test]
    fn apply_preset_print_sets_dpi() {
        let mut opts = default_options();
        apply_preset(Preset::Print, &mut opts);

        assert_eq!(opts.target_formats, vec![ImageFormat::Tiff]);
        assert!((opts.svg_options.dpi - 300.0).abs() < f32::EPSILON);
    }

    #[test]
    fn apply_preset_social_sets_exact_dimensions() {
        let mut opts = default_options();
        apply_preset(Preset::Social, &mut opts);

        let resize = opts.resize.unwrap();
        assert_eq!(resize.width, Some(1200));
        assert_eq!(resize.height, Some(630));
        assert!(!resize.keep_aspect);
    }

    // ===== Property-Based Tests =====
    use proptest::prelude::*;

    fn preset_strategy() -> impl Strategy<Value = Preset> {
        prop_oneof![
            Just(Preset::Web),
            Just(Preset::Thumbnail),
            Just(Preset::Print),
            Just(Preset::Social),
        ]
    }

    // Property 27: 프리셋 개별 옵션 덮어쓰기 — 프리셋+개별 옵션 시 개별 옵션 값이 최종 설정에 반영
    // **Validates: Requirements 35.5**
    proptest! {
        #[test]
        fn prop27_preset_individual_option_override(
            preset in preset_strategy(),
            user_quality in 1u8..=100,
            user_width in 1u32..=4000,
        ) {
            let mut opts = default_options();
            // Set individual options before applying preset
            opts.quality = Some(user_quality);
            opts.resize = Some(ResizeOptions {
                width: Some(user_width),
                height: None,
                keep_aspect: false,
            });
            opts.target_formats = vec![ImageFormat::Png];

            apply_preset(preset, &mut opts);

            // Individual options must override preset defaults
            prop_assert_eq!(opts.quality, Some(user_quality));
            prop_assert_eq!(opts.target_formats, vec![ImageFormat::Png]);
            let resize = opts.resize.unwrap();
            prop_assert_eq!(resize.width, Some(user_width));
        }
    }

    // Property 28: 유효하지 않은 프리셋 이름 거부 — 잘못된 프리셋 이름 시 `InvalidPreset` 에러
    // **Validates: Requirements 35.6**
    proptest! {
        #[test]
        fn prop28_invalid_preset_name_rejected(
            name in "[a-zA-Z0-9_]{1,30}"
                .prop_filter("must not be a valid preset name",
                    |s| !["web", "thumbnail", "print", "social"].contains(&s.to_lowercase().as_str()))
        ) {
            let result = Preset::parse(&name);
            prop_assert!(result.is_err());
            match result.unwrap_err() {
                ConvertError::InvalidPreset { name: err_name } => {
                    prop_assert_eq!(err_name, name);
                }
                other => {
                    prop_assert!(false, "Expected InvalidPreset error, got: {:?}", other);
                }
            }
        }
    }
}

