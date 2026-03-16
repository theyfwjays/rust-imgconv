// imgconv - 변환 오케스트레이터

use std::path::{Path, PathBuf};

use image::DynamicImage;

use crate::crop::{self, CropOptions};
use crate::error::ConvertError;
use crate::filter::{self, BrightnessContrastOptions, ColorFilterOptions};
use crate::format::ImageFormat;
use crate::quality;
use crate::raster;
use crate::resize::{self, ResizeOptions};
use crate::svg::{self, SvgOptions};
use crate::transform::{self, FlipDirection, RotateAngle};
use crate::webp::{self, WebPMode};

use crate::exif;
use crate::overlay::{self, OverlayOptions};
use crate::preset;
use crate::watermark::{self, WatermarkOptions};

#[cfg(feature = "avif")]
use crate::avif;

/// 변환 옵션
#[derive(Debug, Clone)]
pub struct ConvertOptions {
    /// 대상 포맷 (쉼표 구분 시 여러 포맷)
    pub target_formats: Vec<ImageFormat>,
    /// 품질 설정 (1-100, None이면 포맷별 기본값)
    pub quality: Option<u8>,
    /// 리사이즈 옵션
    pub resize: Option<ResizeOptions>,
    /// 크롭 옵션
    pub crop: Option<String>,
    /// 회전 옵션 (90, 180, 270)
    pub rotate: Option<u32>,
    /// 뒤집기 옵션 ("horizontal", "vertical")
    pub flip: Option<String>,
    /// 색상 필터 옵션
    pub color_filter: ColorFilterOptions,
    /// 밝기/대비/감마 옵션
    pub brightness_contrast: BrightnessContrastOptions,
    /// 블러 옵션 (sigma 값)
    pub blur: Option<f32>,
    /// 샤프닝 옵션 (값)
    pub sharpen: Option<f32>,
    /// 텍스트 워터마크 옵션 (텍스트 문자열)
    pub watermark: Option<String>,
    /// 워터마크 위치 (top-left, top-right, bottom-left, bottom-right, center)
    pub watermark_position: Option<String>,
    /// 워터마크 투명도 (0.0-1.0)
    pub watermark_opacity: Option<f32>,
    /// 워터마크 폰트 파일 경로
    pub watermark_font: Option<std::path::PathBuf>,
    /// 이미지 오버레이 옵션 (파일 경로)
    pub overlay: Option<std::path::PathBuf>,
    /// 오버레이 위치 (top-left, top-right, bottom-left, bottom-right, center)
    pub overlay_position: Option<String>,
    /// 오버레이 투명도 (0.0-1.0)
    pub overlay_opacity: Option<f32>,
    /// EXIF 자동 방향 보정
    pub auto_orient: bool,
    /// EXIF 메타데이터 보존
    pub preserve_exif: bool,
    /// 변환 프리셋
    pub preset: Option<String>,
    /// 중복 파일 건너뛰기
    pub skip_identical: bool,
    /// WebP 인코딩 모드
    pub webp_mode: WebPMode,
    /// SVG 관련 옵션
    pub svg_options: SvgOptions,
    /// 출력 디렉토리 (None이면 입력 파일과 동일 디렉토리)
    pub output_dir: Option<PathBuf>,
    /// 기존 파일 덮어쓰기 허용
    pub overwrite: bool,
    /// dry-run 모드
    pub dry_run: bool,
    /// 상세 로그 출력
    pub verbose: bool,
}

/// 단일 파일 변환 결과
#[derive(Debug)]
pub struct ConvertResult {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub input_format: ImageFormat,
    pub output_format: ImageFormat,
    pub input_size: u64,
    pub output_size: u64,
}

/// 변환 계획 (dry-run용)
#[derive(Debug)]
pub struct ConversionPlan {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub input_format: ImageFormat,
    pub output_format: ImageFormat,
    pub resize: Option<ResizeOptions>,
    pub quality: Option<u8>,
}

/// 입력 파일의 확장자로부터 포맷을 감지한다.
fn detect_input_format(input: &Path) -> Result<ImageFormat, ConvertError> {
    // 복합 확장자 확인 (예: .uhdr.jpg)
    if let Some(name) = input.file_name().and_then(|n| n.to_str()) {
        let name_lower = name.to_ascii_lowercase();
        if name_lower.ends_with(".uhdr.jpg") {
            return ImageFormat::from_extension("uhdr.jpg");
        }
    }

    let ext = input
        .extension()
        .and_then(|e| e.to_str())
        .ok_or_else(|| ConvertError::UnsupportedInputFormat {
            extension: input.display().to_string(),
            supported: ImageFormat::supported_extensions().join(", "),
        })?;
    ImageFormat::from_extension(ext)
}

/// 출력 파일 경로를 생성한다.
/// output_dir이 지정되면 해당 디렉토리에, 아니면 입력 파일과 동일한 디렉토리에 저장한다.
/// 파일명 = 원본 파일명(확장자 제외) + 대상 포맷 확장자
fn build_output_path(
    input: &Path,
    target_format: ImageFormat,
    output_dir: Option<&Path>,
) -> PathBuf {
    let stem = input.file_stem().unwrap_or_default();
    let ext = target_format.extension();

    let dir = match output_dir {
        Some(d) => d.to_path_buf(),
        None => input.parent().unwrap_or_else(|| Path::new(".")).to_path_buf(),
    };

    dir.join(format!("{}.{}", stem.to_string_lossy(), ext))
}

/// 포맷에 맞는 디코더로 이미지를 디코딩한다.
fn decode_image(
    input: &Path,
    format: ImageFormat,
    svg_options: &SvgOptions,
    resize_width: Option<u32>,
) -> Result<DynamicImage, ConvertError> {
    match format {
        ImageFormat::Svg => svg::rasterize_svg(input, svg_options, resize_width),
        ImageFormat::WebP => webp::decode_webp(input),
        #[cfg(feature = "avif")]
        ImageFormat::Avif => avif::decode_avif(input),
        #[cfg(feature = "jxl")]
        ImageFormat::Jxl => crate::jxl::decode_jxl(input),
        #[cfg(feature = "dds")]
        ImageFormat::Dds => crate::dds::decode_dds(input),
        #[cfg(feature = "pcx")]
        ImageFormat::Pcx => crate::pcx::decode_pcx(input),
        #[cfg(feature = "ultrahdr")]
        ImageFormat::UltraHdr => crate::ultrahdr::decode_ultrahdr(input),
        #[cfg(feature = "apng")]
        ImageFormat::Apng => crate::apng::decode_apng(input),
        _ => raster::decode_raster(input),
    }
}

/// 포맷에 맞는 인코더로 이미지를 인코딩하여 저장한다.
fn encode_image(
    img: &DynamicImage,
    output: &Path,
    target_format: ImageFormat,
    quality_val: Option<u8>,
    webp_mode: WebPMode,
    svg_preset: svg::SvgPreset,
) -> Result<(), ConvertError> {
    match target_format {
        ImageFormat::Svg => {
            let svg_str = svg::trace_to_svg(img, svg_preset)?;
            svg::save_svg(&svg_str, output)
        }
        ImageFormat::WebP => webp::encode_webp(img, output, webp_mode, quality_val),
        #[cfg(feature = "avif")]
        ImageFormat::Avif => avif::encode_avif(img, output, quality_val),
        #[cfg(feature = "pcx")]
        ImageFormat::Pcx => crate::pcx::encode_pcx(img, output),
        #[cfg(feature = "ultrahdr")]
        ImageFormat::UltraHdr => crate::ultrahdr::encode_ultrahdr(img, output),
        #[cfg(feature = "apng")]
        ImageFormat::Apng => crate::apng::encode_apng(img, output),
        _ => raster::encode_raster(img, target_format, output, quality_val),
    }
}

/// 변환 계획을 생성한다 (dry-run용).
///
/// 실제 파일 I/O(디코딩, 인코딩, 파일 쓰기)를 수행하지 않고,
/// 입력 파일에 대한 변환 계획(입력/출력 경로, 포맷, 리사이즈, 품질)을 생성한다.
pub fn plan_conversion(
    input: &Path,
    options: &ConvertOptions,
) -> Result<Vec<ConversionPlan>, ConvertError> {
    let input_format = detect_input_format(input)?;

    let mut plans = Vec::new();

    for &target_format in &options.target_formats {
        let output_path = build_output_path(input, target_format, options.output_dir.as_deref());

        let (quality_val, _warning) = quality::resolve_quality(target_format, options.quality);

        plans.push(ConversionPlan {
            input_path: input.to_path_buf(),
            output_path,
            input_format,
            output_format: target_format,
            resize: options.resize.clone(),
            quality: quality_val,
        });
    }

    Ok(plans)
}

/// 단일 파일을 변환한다.
///
/// 입력 파일을 디코딩하고, 리사이즈를 적용한 후,
/// 지정된 모든 대상 포맷으로 인코딩하여 저장한다.
///
/// 다중 출력 포맷이 지정된 경우, 일부 포맷 변환이 실패해도
/// 나머지 포맷의 변환을 계속 진행한다 (요구사항 7.2).
/// 모든 포맷이 실패하면 마지막 에러를 반환한다.
pub fn convert_file(
    input: &Path,
    options: &ConvertOptions,
) -> Result<Vec<ConvertResult>, ConvertError> {
    // 0. 프리셋 적용 (변환 시작 전, 개별 옵션이 프리셋을 덮어씀)
    let mut options = options.clone();
    if let Some(ref preset_name) = options.preset {
        let p = preset::Preset::from_str(preset_name)?;
        preset::apply_preset(p, &mut options);
    }

    // 1. 입력 파일 포맷 감지
    let input_format = detect_input_format(input)?;

    // 1.5. 대상 포맷 쓰기 지원 여부 확인
    for &target_format in &options.target_formats {
        if !target_format.supports_write() {
            return Err(ConvertError::WriteNotSupported {
                format: format!("{:?}", target_format),
            });
        }
    }

    // 2. 입력 파일 크기
    let input_size = std::fs::metadata(input)?.len();

    // SVG 래스터화 시 width 옵션 전달 (resize에서 width만 지정된 경우)
    let svg_width = options.resize.as_ref().and_then(|r| r.width);

    // 3. 디코딩
    let img = decode_image(input, input_format, &options.svg_options, svg_width)?;

    // 3.5. EXIF 방향 보정 (파이프라인 순서: 디코딩 → EXIF 방향 보정 → 크롭)
    let img = if options.auto_orient {
        exif::auto_orient(&img, input)?
    } else {
        img
    };

    // 4. 크롭 (파이프라인 순서: EXIF 방향 보정 → 크롭)
    let img = if let Some(ref crop_str) = options.crop {
        let crop_opts = CropOptions::from_str(crop_str)?;
        crop::apply_crop(&img, &crop_opts)?
    } else {
        img
    };

    // 5. 회전 (파이프라인 순서: 크롭 → 회전)
    let img = if let Some(rotate_degrees) = options.rotate {
        let angle = RotateAngle::from_degrees(rotate_degrees)?;
        transform::apply_rotate(&img, angle)
    } else {
        img
    };

    // 6. 뒤집기 (파이프라인 순서: 회전 → 뒤집기)
    let img = if let Some(ref flip_str) = options.flip {
        let direction = FlipDirection::from_str(flip_str)?;
        transform::apply_flip(&img, direction)
    } else {
        img
    };

    // 7. 리사이즈 적용 (SVG 입력이 아닌 경우에만, SVG는 래스터화 시 이미 크기 조정됨)
    let img = if let Some(ref resize_opts) = options.resize {
        if input_format == ImageFormat::Svg {
            // SVG는 rasterize_svg에서 이미 width를 적용했으므로,
            // height만 지정되었거나 width+height 조합인 경우에만 추가 리사이즈
            if resize_opts.height.is_some() {
                resize::resize_image(&img, resize_opts)?
            } else {
                img
            }
        } else {
            resize::resize_image(&img, resize_opts)?
        }
    } else {
        img
    };

    // 8. 색상 필터 (파이프라인 순서: 리사이즈 → 색상 필터)
    let img = if !options.color_filter.is_none() {
        filter::apply_color_filters(&img, &options.color_filter)
    } else {
        img
    };

    // 9. 밝기/대비/감마 (파이프라인 순서: 색상 필터 → 밝기/대비/감마)
    let img = if options.brightness_contrast.brightness.is_some()
        || options.brightness_contrast.contrast.is_some()
        || options.brightness_contrast.gamma.is_some()
    {
        filter::apply_brightness_contrast(&img, &options.brightness_contrast)?
    } else {
        img
    };

    // 10. 블러 또는 샤프닝 (파이프라인 순서: 밝기/대비/감마 → 블러/샤프닝)
    let img = if let Some(sigma) = options.blur {
        filter::apply_blur(&img, sigma)?
    } else if let Some(value) = options.sharpen {
        filter::apply_sharpen(&img, value)?
    } else {
        img
    };

    // 10.5. 워터마크 (파이프라인 순서: 블러/샤프닝 → 워터마크)
    let img = if let Some(ref wm_text) = options.watermark {
        let position = match &options.watermark_position {
            Some(s) => watermark::Position::from_str(s)?,
            None => watermark::Position::default(),
        };
        let wm_opts = WatermarkOptions {
            text: wm_text.clone(),
            position,
            opacity: options.watermark_opacity.unwrap_or(0.5),
            font_path: options.watermark_font.clone(),
        };
        watermark::apply_watermark(&img, &wm_opts)?
    } else {
        img
    };

    // 10.6. 오버레이 (파이프라인 순서: 워터마크 → 오버레이)
    let img = if let Some(ref overlay_path) = options.overlay {
        let position = match &options.overlay_position {
            Some(s) => watermark::Position::from_str(s)?,
            None => watermark::Position::default(),
        };
        let ov_opts = OverlayOptions {
            image_path: overlay_path.clone(),
            position,
            opacity: options.overlay_opacity.unwrap_or(1.0),
        };
        overlay::apply_overlay(&img, &ov_opts)?
    } else {
        img
    };

    // 11. 다중 출력 포맷 순회하며 인코딩
    let mut results = Vec::new();
    let mut last_error: Option<ConvertError> = None;

    for &target_format in &options.target_formats {
        // 출력 경로 생성
        let output_path = build_output_path(input, target_format, options.output_dir.as_deref());

        // 입력/출력 경로 동일 여부 검사 (요구사항 10.3)
        // canonicalize가 실패하면 (파일이 아직 없는 경우 등) 문자열 비교로 폴백
        let same_path = match (input.canonicalize(), output_path.canonicalize()) {
            (Ok(canon_in), Ok(canon_out)) => canon_in == canon_out,
            _ => input == output_path.as_path(),
        };
        if same_path {
            return Err(ConvertError::SameInputOutput {
                path: input.display().to_string(),
            });
        }

        // 출력 파일 존재 시 덮어쓰기 확인 (요구사항 10.1, 10.2)
        if output_path.exists() && !options.overwrite {
            if options.target_formats.len() == 1 {
                // 단일 포맷 변환: 에러 반환
                return Err(ConvertError::OutputExists {
                    path: output_path.display().to_string(),
                });
            } else {
                // 다중 포맷 변환: 건너뛰고 경고 출력
                eprintln!(
                    "경고: 출력 파일이 이미 존재하여 건너뜁니다: {}. --overwrite 옵션을 사용하세요",
                    output_path.display()
                );
                continue;
            }
        }

        // 중복 검사 (요구사항 34.1, 34.2, 34.3)
        // --skip-identical 지정 시, 출력 파일이 이미 존재하면 해시 비교
        #[cfg(feature = "dedup")]
        if options.skip_identical && output_path.exists() {
            if let Ok(true) = crate::dedup::is_identical(input, &output_path) {
                eprintln!(
                    "건너뜀: 입력과 출력이 동일합니다: {}",
                    output_path.display()
                );
                continue;
            }
        }

        // 품질 결정 및 경고 처리
        let (quality_val, warning) = quality::resolve_quality(target_format, options.quality);
        if let Some(warn_msg) = warning {
            eprintln!("경고: {}", warn_msg);
        }

        // 인코딩 및 저장
        match encode_image(
            &img,
            &output_path,
            target_format,
            quality_val,
            options.webp_mode,
            options.svg_options.preset,
        ) {
            Ok(()) => {
                // 12. EXIF 보존 (인코딩 이후, --preserve-exif 지정 시)
                if options.preserve_exif {
                    if let Err(e) = exif::preserve_exif(input, &output_path) {
                        eprintln!("경고: EXIF 보존 실패: {}. EXIF 없이 저장합니다.", e);
                    }
                }

                let output_size = std::fs::metadata(&output_path)
                    .map(|m| m.len())
                    .unwrap_or(0);

                results.push(ConvertResult {
                    input_path: input.to_path_buf(),
                    output_path,
                    input_format,
                    output_format: target_format,
                    input_size,
                    output_size,
                });
            }
            Err(e) => {
                eprintln!(
                    "경고: {} → {} 변환 실패: {}",
                    input.display(),
                    target_format,
                    e
                );
                last_error = Some(e);
            }
        }
    }

    // 모든 포맷이 실패한 경우 에러 반환
    if results.is_empty() {
        return Err(last_error.unwrap_or_else(|| {
            ConvertError::EncodingError("변환할 대상 포맷이 없습니다".into())
        }));
    }

    Ok(results)
}


#[cfg(test)]
mod tests {
    use super::*;
    use image::RgbaImage;

    /// 테스트용 작은 PNG 이미지를 생성하여 지정된 경로에 저장한다.
    fn create_test_png(path: &Path, w: u32, h: u32) {
        let mut img = RgbaImage::new(w, h);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            *pixel = image::Rgba([
                (x % 256) as u8,
                (y % 256) as u8,
                ((x + y) % 256) as u8,
                255,
            ]);
        }
        let dyn_img = DynamicImage::ImageRgba8(img);
        dyn_img.save(path).unwrap();
    }

    /// 기본 ConvertOptions를 생성한다.
    fn default_options(target_formats: Vec<ImageFormat>) -> ConvertOptions {
        ConvertOptions {
            target_formats,
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

    /// 단일 파일 변환 흐름 테스트: PNG → JPEG 변환 후 출력 파일 존재 및 ConvertResult 확인
    /// Validates: Requirements 5.1
    #[test]
    fn convert_single_file_png_to_jpeg() {
        let dir = tempfile::tempdir().unwrap();
        let input = dir.path().join("test_input.png");
        create_test_png(&input, 16, 16);

        let options = default_options(vec![ImageFormat::Jpeg]);
        let results = convert_file(&input, &options).unwrap();

        assert_eq!(results.len(), 1);
        let result = &results[0];
        assert_eq!(result.input_format, ImageFormat::Png);
        assert_eq!(result.output_format, ImageFormat::Jpeg);
        assert!(result.output_path.exists());
        assert!(result.output_size > 0);
        assert!(result.input_size > 0);
        assert_eq!(result.output_path.file_name().unwrap(), "test_input.jpg");
    }

    /// 다중 포맷 변환 테스트: PNG → PNG, BMP 동시 변환 후 모든 출력 파일 존재 확인
    /// Validates: Requirements 7.1
    #[test]
    fn convert_multi_format_png_to_bmp_and_tga() {
        let dir = tempfile::tempdir().unwrap();
        let input = dir.path().join("multi.png");
        create_test_png(&input, 16, 16);

        let options = default_options(vec![ImageFormat::Bmp, ImageFormat::Tga]);
        let results = convert_file(&input, &options).unwrap();

        assert_eq!(results.len(), 2);

        let bmp_result = results.iter().find(|r| r.output_format == ImageFormat::Bmp).unwrap();
        assert!(bmp_result.output_path.exists());
        assert_eq!(bmp_result.output_path.file_name().unwrap(), "multi.bmp");

        let tga_result = results.iter().find(|r| r.output_format == ImageFormat::Tga).unwrap();
        assert!(tga_result.output_path.exists());
        assert_eq!(tga_result.output_path.file_name().unwrap(), "multi.tga");
    }

    /// 동일 입력/출력 경로 보호 테스트: PNG → PNG 동일 디렉토리에서 SameInputOutput 에러 확인
    /// Validates: Requirements 10.3
    #[test]
    fn convert_same_input_output_path_returns_error() {
        let dir = tempfile::tempdir().unwrap();
        let input = dir.path().join("same.png");
        create_test_png(&input, 8, 8);

        // PNG → PNG in same directory → output path == input path
        let options = default_options(vec![ImageFormat::Png]);
        let result = convert_file(&input, &options);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, ConvertError::SameInputOutput { .. }),
            "expected SameInputOutput, got: {err:?}"
        );
    }

    /// 출력 파일 존재 시 보호 테스트: overwrite=false일 때 OutputExists 에러 확인
    /// Validates: Requirements 10.1
    #[test]
    fn convert_output_exists_without_overwrite_returns_error() {
        let dir = tempfile::tempdir().unwrap();
        let input = dir.path().join("exists_test.png");
        create_test_png(&input, 8, 8);

        // 출력 파일을 미리 생성
        let output = dir.path().join("exists_test.jpg");
        std::fs::write(&output, b"dummy").unwrap();

        let options = default_options(vec![ImageFormat::Jpeg]);
        let result = convert_file(&input, &options);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, ConvertError::OutputExists { .. }),
            "expected OutputExists, got: {err:?}"
        );
    }

    /// 덮어쓰기 모드 테스트: overwrite=true일 때 기존 파일을 덮어쓰고 성공
    /// Validates: Requirements 10.1
    #[test]
    fn convert_overwrite_mode_succeeds() {
        let dir = tempfile::tempdir().unwrap();
        let input = dir.path().join("overwrite_test.png");
        create_test_png(&input, 8, 8);

        // 출력 파일을 미리 생성
        let output = dir.path().join("overwrite_test.jpg");
        std::fs::write(&output, b"dummy").unwrap();
        let old_size = std::fs::metadata(&output).unwrap().len();

        let mut options = default_options(vec![ImageFormat::Jpeg]);
        options.overwrite = true;

        let results = convert_file(&input, &options).unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].output_path.exists());
        // 덮어쓰기 후 파일 크기가 변경되어야 함 (dummy 5바이트 → 실제 JPEG)
        let new_size = std::fs::metadata(&output).unwrap().len();
        assert_ne!(old_size, new_size);
    }

    /// Dry-run 모드 테스트: plan_conversion이 올바른 ConversionPlan을 반환하고 파일이 생성되지 않음
    /// Validates: Requirements 12.1
    #[test]
    fn plan_conversion_dry_run_no_files_created() {
        let dir = tempfile::tempdir().unwrap();
        let input = dir.path().join("dryrun.png");
        create_test_png(&input, 8, 8);

        let options = default_options(vec![ImageFormat::Jpeg, ImageFormat::Bmp]);
        let plans = plan_conversion(&input, &options).unwrap();

        assert_eq!(plans.len(), 2);

        // JPEG plan 확인
        let jpeg_plan = plans.iter().find(|p| p.output_format == ImageFormat::Jpeg).unwrap();
        assert_eq!(jpeg_plan.input_format, ImageFormat::Png);
        assert_eq!(jpeg_plan.input_path, input);
        assert_eq!(jpeg_plan.output_path.file_name().unwrap(), "dryrun.jpg");

        // BMP plan 확인
        let bmp_plan = plans.iter().find(|p| p.output_format == ImageFormat::Bmp).unwrap();
        assert_eq!(bmp_plan.input_format, ImageFormat::Png);
        assert_eq!(bmp_plan.output_path.file_name().unwrap(), "dryrun.bmp");

        // 실제 파일이 생성되지 않았는지 확인
        let jpeg_output = dir.path().join("dryrun.jpg");
        let bmp_output = dir.path().join("dryrun.bmp");
        assert!(!jpeg_output.exists(), "dry-run should not create JPEG file");
        assert!(!bmp_output.exists(), "dry-run should not create BMP file");
    }

    /// 출력 디렉토리 옵션 테스트: output_dir 지정 시 해당 디렉토리에 출력 파일 생성
    /// Validates: Requirements 5.1
    #[test]
    fn convert_with_output_dir() {
        let dir = tempfile::tempdir().unwrap();
        let input = dir.path().join("outdir_test.png");
        create_test_png(&input, 8, 8);

        let output_dir = dir.path().join("output");
        std::fs::create_dir_all(&output_dir).unwrap();

        let mut options = default_options(vec![ImageFormat::Jpeg]);
        options.output_dir = Some(output_dir.clone());

        let results = convert_file(&input, &options).unwrap();
        assert_eq!(results.len(), 1);

        let expected_output = output_dir.join("outdir_test.jpg");
        assert_eq!(results[0].output_path, expected_output);
        assert!(expected_output.exists());
    }

    /// JXL feature 비활성화 시 .jxl 파일 변환 요청이 JxlNotEnabled 에러를 반환하는지 확인
    /// Validates: Requirements 18.3
    #[cfg(not(feature = "jxl"))]
    #[test]
    fn convert_jxl_not_enabled_returns_error() {
        let dir = tempfile::tempdir().unwrap();
        let input = dir.path().join("test.jxl");
        // 더미 파일 생성 (내용은 중요하지 않음 — 포맷 감지 단계에서 에러 발생)
        std::fs::write(&input, b"dummy jxl content").unwrap();

        let options = default_options(vec![ImageFormat::Jpeg]);
        let result = convert_file(&input, &options);

        assert!(result.is_err());
        assert!(
            matches!(result.unwrap_err(), ConvertError::JxlNotEnabled),
            "expected JxlNotEnabled error when jxl feature is disabled"
        );
    }

    /// DDS feature 비활성화 시 .dds 파일 변환 요청이 DdsNotEnabled 에러를 반환하는지 확인
    /// Validates: Requirements 19.3
    #[cfg(not(feature = "dds"))]
    #[test]
    fn convert_dds_not_enabled_returns_error() {
        let dir = tempfile::tempdir().unwrap();
        let input = dir.path().join("test.dds");
        std::fs::write(&input, b"dummy dds content").unwrap();

        let options = default_options(vec![ImageFormat::Jpeg]);
        let result = convert_file(&input, &options);

        assert!(result.is_err());
        assert!(
            matches!(result.unwrap_err(), ConvertError::DdsNotEnabled),
            "expected DdsNotEnabled error when dds feature is disabled"
        );
    }

    /// PCX feature 비활성화 시 .pcx 파일 변환 요청이 PcxNotEnabled 에러를 반환하는지 확인
    /// Validates: Requirements 20.4
    #[cfg(not(feature = "pcx"))]
    #[test]
    fn convert_pcx_not_enabled_returns_error() {
        let dir = tempfile::tempdir().unwrap();
        let input = dir.path().join("test.pcx");
        std::fs::write(&input, b"dummy pcx content").unwrap();

        let options = default_options(vec![ImageFormat::Jpeg]);
        let result = convert_file(&input, &options);

        assert!(result.is_err());
        assert!(
            matches!(result.unwrap_err(), ConvertError::PcxNotEnabled),
            "expected PcxNotEnabled error when pcx feature is disabled"
        );
    }

    /// Ultra HDR feature 비활성화 시 .uhdr.jpg 파일 변환 요청이 UltraHdrNotEnabled 에러를 반환하는지 확인
    /// Validates: Requirements 21.3
    #[cfg(not(feature = "ultrahdr"))]
    #[test]
    fn convert_ultrahdr_not_enabled_returns_error() {
        let dir = tempfile::tempdir().unwrap();
        let input = dir.path().join("test.uhdr.jpg");
        std::fs::write(&input, b"dummy ultrahdr content").unwrap();

        let options = default_options(vec![ImageFormat::Jpeg]);
        let result = convert_file(&input, &options);

        assert!(result.is_err());
        assert!(
            matches!(result.unwrap_err(), ConvertError::UltraHdrNotEnabled),
            "expected UltraHdrNotEnabled error when ultrahdr feature is disabled"
        );
    }

    /// APNG feature 비활성화 시 .apng 파일 변환 요청이 ApngNotEnabled 에러를 반환하는지 확인
    /// Validates: Requirements 22.4
    #[cfg(not(feature = "apng"))]
    #[test]
    fn convert_apng_not_enabled_returns_error() {
        let dir = tempfile::tempdir().unwrap();
        let input = dir.path().join("test.apng");
        std::fs::write(&input, b"dummy apng content").unwrap();

        let options = default_options(vec![ImageFormat::Jpeg]);
        let result = convert_file(&input, &options);

        assert!(result.is_err());
        assert!(
            matches!(result.unwrap_err(), ConvertError::ApngNotEnabled),
            "expected ApngNotEnabled error when apng feature is disabled"
        );
    }

    /// 전체 파이프라인 통합 테스트: 크롭 + 회전 + 리사이즈 + 그레이스케일 + 블러 + 워터마크를 한 번에 적용
    /// Validates: Requirements 23.2, 24.1, 25.4, 26.1, 27.1, 30.5
    #[test]
    fn integration_multi_option_pipeline() {
        let dir = tempfile::tempdir().unwrap();
        let out_dir = dir.path().join("out");
        std::fs::create_dir_all(&out_dir).unwrap();

        let input = dir.path().join("pipeline.png");
        create_test_png(&input, 16, 16);

        let mut options = default_options(vec![ImageFormat::Png]);
        options.output_dir = Some(out_dir.clone());
        options.overwrite = true;
        // 크롭: (0,0) 에서 12x12 영역
        options.crop = Some("0,0,12,12".to_string());
        // 회전: 90도
        options.rotate = Some(90);
        // 리사이즈: 너비 8px (종횡비 유지)
        options.resize = Some(ResizeOptions {
            width: Some(8),
            height: None,
            keep_aspect: false,
        });
        // 그레이스케일
        options.color_filter = ColorFilterOptions {
            grayscale: true,
            invert: false,
            sepia: false,
        };
        // 블러
        options.blur = Some(1.0);
        // 밝기 조정
        options.brightness_contrast = BrightnessContrastOptions {
            brightness: Some(10),
            contrast: None,
            gamma: None,
        };
        // 워터마크
        options.watermark = Some("TEST".to_string());

        let results = convert_file(&input, &options).unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].output_path.exists());

        // 출력 이미지 검증: 크롭(12x12) → 회전90(12x12) → 리사이즈(w=8, h=8)
        let output_img = image::open(&results[0].output_path).unwrap();
        assert_eq!(output_img.width(), 8);
        assert_eq!(output_img.height(), 8);
    }

    /// 파이프라인 순서 테스트: 그레이스케일 적용 시 출력 픽셀의 R=G=B 확인
    /// Validates: Requirements 25.4
    #[test]
    fn integration_grayscale_produces_rgb_equal_pixels() {
        let dir = tempfile::tempdir().unwrap();
        let out_dir = dir.path().join("out");
        std::fs::create_dir_all(&out_dir).unwrap();

        let input = dir.path().join("gray_test.png");
        create_test_png(&input, 8, 8);

        let mut options = default_options(vec![ImageFormat::Png]);
        options.output_dir = Some(out_dir.clone());
        options.overwrite = true;
        options.color_filter = ColorFilterOptions {
            grayscale: true,
            invert: false,
            sepia: false,
        };

        let results = convert_file(&input, &options).unwrap();
        assert_eq!(results.len(), 1);

        let output_img = image::open(&results[0].output_path).unwrap().to_rgba8();
        for (_x, _y, pixel) in output_img.enumerate_pixels() {
            assert_eq!(pixel[0], pixel[1], "R != G in grayscale output");
            assert_eq!(pixel[1], pixel[2], "G != B in grayscale output");
        }
    }


}

#[cfg(test)]
mod proptests {
    use super::*;
    use image::{DynamicImage, RgbaImage};
    use proptest::prelude::*;

    /// 테스트용 PNG 이미지를 생성하여 저장한다.
    fn create_test_png(path: &std::path::Path, w: u32, h: u32) {
        let img = RgbaImage::from_fn(w, h, |x, y| {
            image::Rgba([
                (x % 256) as u8,
                (y % 256) as u8,
                ((x + y) % 256) as u8,
                255,
            ])
        });
        DynamicImage::ImageRgba8(img).save(path).unwrap();
    }

    // Property 4: 크롭 후 리사이즈 파이프라인 순서
    // 크롭+리사이즈 동시 지정 시 최종 크기가 리사이즈 대상 크기와 일치
    // **Validates: Requirements 23.2**
    proptest! {
        #[test]
        fn prop4_crop_then_resize_pipeline_order(
            crop_x in 0u32..50,
            crop_y in 0u32..50,
            crop_w in 10u32..100,
            crop_h in 10u32..100,
            resize_w in 5u32..80,
        ) {
            // 이미지 크기는 200x200으로 고정하여 크롭 영역이 항상 유효하도록 함
            let img_w = 200u32;
            let img_h = 200u32;

            // 크롭 영역이 이미지 범위 내인지 확인
            prop_assume!(crop_x + crop_w <= img_w);
            prop_assume!(crop_y + crop_h <= img_h);

            let dir = tempfile::tempdir().unwrap();
            let input = dir.path().join("prop4_input.png");
            create_test_png(&input, img_w, img_h);

            let crop_str = format!("{},{},{},{}", crop_x, crop_y, crop_w, crop_h);

            let options = ConvertOptions {
                target_formats: vec![ImageFormat::Png],
                quality: None,
                resize: Some(ResizeOptions {
                    width: Some(resize_w),
                    height: None,
                    keep_aspect: false,
                }),
                crop: Some(crop_str),
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
                output_dir: Some(dir.path().join("out")),
                overwrite: true,
                dry_run: false,
                verbose: false,
            };

            std::fs::create_dir_all(dir.path().join("out")).unwrap();

            let results = convert_file(&input, &options).unwrap();
            prop_assert_eq!(results.len(), 1);

            // 출력 이미지를 열어 크기 확인
            let output_img = image::open(&results[0].output_path).unwrap();

            // 최종 크기는 리사이즈 대상 크기와 일치해야 함 (크롭 크기가 아님)
            prop_assert_eq!(output_img.width(), resize_w);

            // height는 크롭 결과의 종횡비에 따라 자동 계산됨
            let expected_h = ((crop_h as f64) * (resize_w as f64) / (crop_w as f64)).round() as u32;
            let expected_h = expected_h.max(1);
            prop_assert_eq!(output_img.height(), expected_h);
        }
    }
}
