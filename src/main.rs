use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};

use imgconv::batch::ProgressCallback;
use imgconv::convert::ConvertOptions;
use imgconv::format::parse_formats;
use imgconv::resize::ResizeOptions;
use imgconv::svg::{SvgOptions, SvgPreset};
use imgconv::webp::WebPMode;

/// 고성능 이미지 포맷 변환 도구
#[derive(Parser, Debug)]
#[command(name = "imgconv", about = "고성능 이미지 포맷 변환 도구")]
struct Cli {
    /// 입력 파일 또는 디렉토리 경로
    input: PathBuf,

    /// 대상 포맷 (쉼표 구분으로 여러 포맷 지정 가능)
    #[arg(long)]
    to: Option<String>,

    /// 출력 디렉토리
    #[arg(short = 'o', long)]
    output: Option<PathBuf>,

    /// 품질 설정 (1-100)
    #[arg(long, value_parser = clap::value_parser!(u8).range(1..=100))]
    quality: Option<u8>,

    /// 리사이즈 너비 (픽셀)
    #[arg(long)]
    width: Option<u32>,

    /// 리사이즈 높이 (픽셀)
    #[arg(long)]
    height: Option<u32>,

    /// 종횡비 유지
    #[arg(long)]
    keep_aspect: bool,

    /// 크롭 영역 (x,y,w,h)
    #[arg(long)]
    crop: Option<String>,

    /// 회전 각도 (90, 180, 270)
    #[arg(long)]
    rotate: Option<u32>,

    /// 뒤집기 방향 (horizontal, vertical)
    #[arg(long)]
    flip: Option<String>,

    /// 흑백 변환
    #[arg(long)]
    grayscale: bool,

    /// 색상 반전
    #[arg(long)]
    invert: bool,

    /// 세피아 톤
    #[arg(long)]
    sepia: bool,

    /// 가우시안 블러 (sigma 값)
    #[arg(long, conflicts_with = "sharpen")]
    blur: Option<f32>,

    /// 언샤프 마스크 (값)
    #[arg(long, conflicts_with = "blur")]
    sharpen: Option<f32>,

    /// 밝기 조정 (양수: 밝게, 음수: 어둡게)
    #[arg(long, allow_hyphen_values = true)]
    brightness: Option<i32>,

    /// 대비 조정
    #[arg(long)]
    contrast: Option<f32>,

    /// 감마 보정
    #[arg(long)]
    gamma: Option<f32>,

    /// EXIF 방향 자동 보정
    #[arg(long, conflicts_with = "no_auto_orient")]
    auto_orient: bool,

    /// EXIF 방향 보정 비활성화
    #[arg(long, conflicts_with = "auto_orient")]
    no_auto_orient: bool,

    /// 이미지 정보 출력 (변환 없음)
    #[arg(long)]
    info: bool,

    /// WebP lossy 인코딩
    #[arg(long, conflicts_with = "lossless")]
    lossy: bool,

    /// WebP lossless 인코딩
    #[arg(long, conflicts_with = "lossy")]
    lossless: bool,

    /// SVG 래스터화 DPI
    #[arg(long, default_value = "96")]
    dpi: f32,

    /// SVG 트레이싱 프리셋 (bw, poster, photo)
    #[arg(long)]
    svg_preset: Option<String>,

    /// 기존 파일 덮어쓰기
    #[arg(long)]
    overwrite: bool,

    /// 변환 계획만 출력 (실제 변환 안 함)
    #[arg(long)]
    dry_run: bool,

    /// 상세 로그 출력
    #[arg(long)]
    verbose: bool,

    /// 텍스트 워터마크
    #[arg(long)]
    watermark: Option<String>,

    /// 워터마크 위치 (top-left, top-right, bottom-left, bottom-right, center)
    #[arg(long, default_value = "bottom-right")]
    watermark_position: Option<String>,

    /// 워터마크 투명도 (0.0-1.0)
    #[arg(long, default_value = "0.5")]
    watermark_opacity: Option<f32>,

    /// 워터마크 폰트 파일 경로
    #[arg(long)]
    watermark_font: Option<PathBuf>,

    /// 오버레이 이미지 파일 경로
    #[arg(long)]
    overlay: Option<PathBuf>,

    /// 오버레이 위치 (top-left, top-right, bottom-left, bottom-right, center)
    #[arg(long, default_value = "bottom-right")]
    overlay_position: Option<String>,

    /// 오버레이 투명도 (0.0-1.0)
    #[arg(long, default_value = "1.0")]
    overlay_opacity: Option<f32>,

    /// EXIF 메타데이터 보존
    #[arg(long)]
    preserve_exif: bool,

    /// 변환 프리셋 (web, thumbnail, print, social)
    #[arg(long)]
    preset: Option<String>,

    /// 두 이미지 품질 비교 (SSIM/PSNR)
    #[arg(long, num_args = 2)]
    compare: Option<Vec<PathBuf>>,

    /// 중복 파일 건너뛰기
    #[arg(long)]
    skip_identical: bool,

    /// 애니메이션 GIF 프레임 추출 (출력 포맷: png, jpg 등)
    #[arg(long)]
    extract_frames: bool,

    /// 여러 이미지를 애니메이션 GIF로 조립 (입력: 디렉토리)
    #[arg(long)]
    assemble_gif: bool,

    /// 애니메이션 프레임 딜레이 (밀리초, 기본: 100)
    #[arg(long, default_value = "100")]
    frame_delay: u32,
}


/// SVG 프리셋 문자열을 SvgPreset 열거형으로 변환
fn parse_svg_preset(s: &str) -> Result<SvgPreset> {
    match s.to_ascii_lowercase().as_str() {
        "bw" => Ok(SvgPreset::Bw),
        "poster" => Ok(SvgPreset::Poster),
        "photo" => Ok(SvgPreset::Photo),
        other => anyhow::bail!(
            "알 수 없는 SVG 프리셋: '{other}'. 사용 가능: bw, poster, photo"
        ),
    }
}

/// CLI 인자를 ConvertOptions로 변환
fn cli_to_options(cli: &Cli) -> Result<ConvertOptions> {
    // --to 문자열 파싱 (--info 모드에서는 불필요)
    let to_str = cli.to.as_deref().unwrap_or("");
    let target_formats = if to_str.is_empty() {
        vec![]
    } else {
        parse_formats(to_str).context("대상 포맷 파싱 실패")?
    };

    // WebP 모드 결정: --lossless → Lossless, 그 외 → Lossy (기본값)
    let webp_mode = if cli.lossless {
        WebPMode::Lossless
    } else {
        WebPMode::Lossy
    };

    // SVG 프리셋 파싱
    let svg_preset = match &cli.svg_preset {
        Some(s) => parse_svg_preset(s)?,
        None => SvgPreset::Default,
    };

    // 리사이즈 옵션 (width 또는 height가 지정된 경우에만)
    let resize = if cli.width.is_some() || cli.height.is_some() {
        Some(ResizeOptions {
            width: cli.width,
            height: cli.height,
            keep_aspect: cli.keep_aspect,
        })
    } else {
        None
    };

    // 색상 필터 옵션
    let color_filter = imgconv::filter::ColorFilterOptions {
        grayscale: cli.grayscale,
        invert: cli.invert,
        sepia: cli.sepia,
    };

    // 밝기/대비/감마 옵션
    let brightness_contrast = imgconv::filter::BrightnessContrastOptions {
        brightness: cli.brightness,
        contrast: cli.contrast,
        gamma: cli.gamma,
    };

    Ok(ConvertOptions {
        target_formats,
        quality: cli.quality,
        resize,
        crop: cli.crop.clone(),
        rotate: cli.rotate,
        flip: cli.flip.clone(),
        color_filter,
        brightness_contrast,
        blur: cli.blur,
        sharpen: cli.sharpen,
        watermark: cli.watermark.clone(),
        watermark_position: cli.watermark_position.clone(),
        watermark_opacity: cli.watermark_opacity,
        watermark_font: cli.watermark_font.clone(),
        overlay: cli.overlay.clone(),
        overlay_position: cli.overlay_position.clone(),
        overlay_opacity: cli.overlay_opacity,
        auto_orient: cli.auto_orient,
        preserve_exif: cli.preserve_exif,
        preset: cli.preset.clone(),
        skip_identical: cli.skip_identical,
        webp_mode,
        svg_options: SvgOptions {
            dpi: cli.dpi,
            preset: svg_preset,
        },
        output_dir: cli.output.clone(),
        overwrite: cli.overwrite,
        dry_run: cli.dry_run,
        verbose: cli.verbose,
    })
}

/// indicatif 기반 진행률 표시 구현체
///
/// 배치 변환 시 진행률 바를 표시하고,
/// 단일 파일 변환 시에는 완료 메시지만 출력한다.
struct IndicatifProgress {
    bar: ProgressBar,
}

impl IndicatifProgress {
    fn new() -> Self {
        Self {
            bar: ProgressBar::hidden(),
        }
    }
}

impl ProgressCallback for IndicatifProgress {
    fn on_start(&self, total: usize) {
        self.bar.set_length(total as u64);
        self.bar.set_position(0);
        let style = ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40}] {pos}/{len} {msg}")
            .expect("유효한 진행률 바 템플릿");
        self.bar.set_style(style);
    }

    fn on_progress(&self, completed: usize, file: &Path) {
        self.bar.set_position(completed as u64);
        if let Some(name) = file.file_name().and_then(|n| n.to_str()) {
            self.bar.set_message(name.to_string());
        }
    }

    fn on_complete(&self) {
        self.bar.finish_and_clear();
    }
}

/// dry-run 모드: 변환 계획만 출력하고 종료 코드 반환
fn run_dry_run(input: &Path, options: &ConvertOptions) -> i32 {
    if input.is_dir() {
        let entries = match std::fs::read_dir(input) {
            Ok(e) => e,
            Err(e) => {
                eprintln!("오류: 디렉토리 읽기 실패: {e}");
                return 2;
            }
        };

        let supported = imgconv::format::ImageFormat::supported_extensions();
        let mut found = false;

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if supported.contains(&ext.to_ascii_lowercase().as_str()) {
                        found = true;
                        match imgconv::convert::plan_conversion(&path, options) {
                            Ok(plans) => {
                                for plan in &plans {
                                    println!(
                                        "[dry-run] {} ({}) → {} ({})",
                                        plan.input_path.display(),
                                        plan.input_format,
                                        plan.output_path.display(),
                                        plan.output_format,
                                    );
                                    if let Some(ref r) = plan.resize {
                                        println!(
                                            "  리사이즈: width={:?}, height={:?}, keep_aspect={}",
                                            r.width, r.height, r.keep_aspect
                                        );
                                    }
                                    if let Some(q) = plan.quality {
                                        println!("  품질: {q}");
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("[dry-run] {} 계획 실패: {e}", path.display());
                            }
                        }
                    }
                }
            }
        }

        if !found {
            eprintln!(
                "오류: 디렉토리에 변환 가능한 이미지 파일 없음: {}",
                input.display()
            );
            return 2;
        }
    } else {
        match imgconv::convert::plan_conversion(input, options) {
            Ok(plans) => {
                for plan in &plans {
                    println!(
                        "[dry-run] {} ({}) → {} ({})",
                        plan.input_path.display(),
                        plan.input_format,
                        plan.output_path.display(),
                        plan.output_format,
                    );
                    if let Some(ref r) = plan.resize {
                        println!(
                            "  리사이즈: width={:?}, height={:?}, keep_aspect={}",
                            r.width, r.height, r.keep_aspect
                        );
                    }
                    if let Some(q) = plan.quality {
                        println!("  품질: {q}");
                    }
                }
            }
            Err(e) => {
                eprintln!("오류: {e}");
                return 2;
            }
        }
    }

    0
}

/// 단일 파일 변환 실행 후 종료 코드 반환
fn run_single_file(input: &Path, options: &ConvertOptions, verbose: bool) -> i32 {
    let start = std::time::Instant::now();

    match imgconv::convert::convert_file(input, options) {
        Ok(results) => {
            let elapsed = start.elapsed();
            for r in &results {
                if verbose {
                    eprintln!(
                        "변환 완료: {} ({}) → {} ({}) [{}B → {}B, {:.2}s]",
                        r.input_path.display(),
                        r.input_format,
                        r.output_path.display(),
                        r.output_format,
                        r.input_size,
                        r.output_size,
                        elapsed.as_secs_f64(),
                    );
                }
            }
            println!(
                "완료: {} 파일 변환 성공 ({:.2}s)",
                results.len(),
                elapsed.as_secs_f64()
            );
            0
        }
        Err(e) => {
            eprintln!("오류: {e}");
            2
        }
    }
}

/// 디렉토리 배치 변환 실행 후 종료 코드 반환
fn run_directory(input: &Path, options: &ConvertOptions, verbose: bool) -> i32 {
    let progress = IndicatifProgress::new();
    let start = std::time::Instant::now();

    match imgconv::batch::convert_directory(input, options, Some(&progress)) {
        Ok(batch_result) => {
            let elapsed = start.elapsed();
            let success_count = batch_result.succeeded.len();
            let fail_count = batch_result.failed.len();

            if verbose {
                for r in &batch_result.succeeded {
                    eprintln!(
                        "성공: {} ({}) → {} ({}) [{}B → {}B]",
                        r.input_path.display(),
                        r.input_format,
                        r.output_path.display(),
                        r.output_format,
                        r.input_size,
                        r.output_size,
                    );
                }
            }

            println!(
                "완료: 성공 {success_count}, 실패 {fail_count} ({:.2}s)",
                elapsed.as_secs_f64()
            );

            if !batch_result.failed.is_empty() {
                eprintln!("실패 목록:");
                for (path, err) in &batch_result.failed {
                    eprintln!("  {} - {err}", path.display());
                }
            }

            if fail_count == 0 {
                0
            } else if success_count == 0 {
                2
            } else {
                1
            }
        }
        Err(e) => {
            eprintln!("오류: {e}");
            2
        }
    }
}

/// --info 모드: 이미지 정보 출력 후 종료 코드 반환
fn run_info(input: &Path) -> i32 {
    if input.is_dir() {
        match imgconv::info::get_directory_info(input) {
            Ok(results) => {
                if results.is_empty() {
                    eprintln!(
                        "디렉토리에 지원되는 이미지 파일 없음: {}",
                        input.display()
                    );
                    return 0;
                }
                for (path, info) in &results {
                    println!("{}:", path.display());
                    print!("{info}");
                }
                0
            }
            Err(e) => {
                eprintln!("오류: {e}");
                2
            }
        }
    } else {
        match imgconv::info::get_image_info(input) {
            Ok(info) => {
                println!("{}:", input.display());
                print!("{info}");
                0
            }
            Err(e) => {
                eprintln!("오류: {e}");
                2
            }
        }
    }
}

/// --compare 모드: 두 이미지 SSIM/PSNR 비교 후 종료 코드 반환
fn run_compare(paths: &[PathBuf]) -> i32 {
    #[cfg(feature = "compare")]
    {
        let path_a = &paths[0];
        let path_b = &paths[1];
        match imgconv::compare::compare_images(path_a, path_b) {
            Ok(result) => {
                println!("SSIM: {:.6}", result.ssim);
                if result.psnr.is_infinite() {
                    println!("PSNR: Inf (동일한 이미지)");
                } else {
                    println!("PSNR: {:.2} dB", result.psnr);
                }
                0
            }
            Err(e) => {
                eprintln!("오류: {e}");
                2
            }
        }
    }
    #[cfg(not(feature = "compare"))]
    {
        let _ = paths;
        eprintln!("오류: compare 기능이 비활성화됨. `cargo build --features compare`로 빌드하세요");
        2
    }
}

fn main() {
    let cli = Cli::parse();
    let input = &cli.input;

    // --info 모드: 변환 없이 이미지 정보 출력
    if cli.info {
        let exit_code = run_info(input);
        std::process::exit(exit_code);
    }

    // --compare 모드: 변환 없이 두 이미지 SSIM/PSNR 비교
    if let Some(ref compare_paths) = cli.compare {
        let exit_code = run_compare(compare_paths);
        std::process::exit(exit_code);
    }

    // --extract-frames 모드: 애니메이션 GIF → 프레임 추출
    if cli.extract_frames {
        let format = cli.to.as_deref().unwrap_or("png");
        let output_dir = cli.output.clone().unwrap_or_else(|| {
            let stem = input.file_stem().and_then(|s| s.to_str()).unwrap_or("frames");
            PathBuf::from(format!("{stem}_frames"))
        });
        let start = std::time::Instant::now();
        match imgconv::animation::extract_frames(input, &output_dir, format) {
            Ok(result) => {
                let elapsed = start.elapsed();
                println!(
                    "프레임 추출 완료: {} 프레임 → {} ({:.2}s)",
                    result.frame_count,
                    result.output_dir.display(),
                    elapsed.as_secs_f64()
                );
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!("오류: {e}");
                std::process::exit(2);
            }
        }
    }

    // --assemble-gif 모드: 여러 이미지 → 애니메이션 GIF 조립
    if cli.assemble_gif {
        if !input.is_dir() {
            eprintln!("오류: --assemble-gif는 디렉토리를 입력으로 받습니다");
            std::process::exit(2);
        }
        let output_path = cli.output.clone().unwrap_or_else(|| {
            let stem = input.file_name().and_then(|s| s.to_str()).unwrap_or("output");
            PathBuf::from(format!("{stem}.gif"))
        });
        // 디렉토리에서 이미지 파일 수집 (정렬)
        let mut frame_paths: Vec<PathBuf> = std::fs::read_dir(input)
            .unwrap_or_else(|e| { eprintln!("오류: {e}"); std::process::exit(2); })
            .filter_map(|entry| entry.ok())
            .map(|e| e.path())
            .filter(|p| {
                p.extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| matches!(ext.to_lowercase().as_str(), "png" | "jpg" | "jpeg" | "bmp" | "gif" | "tga" | "tiff" | "webp"))
                    .unwrap_or(false)
            })
            .collect();
        frame_paths.sort();

        let start = std::time::Instant::now();
        match imgconv::animation::assemble_gif(&frame_paths, &output_path, cli.frame_delay) {
            Ok(result) => {
                let elapsed = start.elapsed();
                println!(
                    "GIF 조립 완료: {} 프레임 → {} ({:.2}s)",
                    result.frame_count,
                    result.output_path.display(),
                    elapsed.as_secs_f64()
                );
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!("오류: {e}");
                std::process::exit(2);
            }
        }
    }

    // --info, --compare, --extract-frames, --assemble-gif 이외의 모드에서는 --to 필수
    if cli.to.is_none() {
        eprintln!("오류: --to 옵션이 필요합니다. 사용법: imgconv <입력> --to <포맷>");
        std::process::exit(2);
    }

    let options = match cli_to_options(&cli) {
        Ok(opts) => opts,
        Err(e) => {
            eprintln!("오류: {e}");
            std::process::exit(2);
        }
    };

    // dry-run 모드: 변환 계획만 출력
    if cli.dry_run {
        let exit_code = run_dry_run(input, &options);
        std::process::exit(exit_code);
    }

    // 입력 경로가 파일인지 디렉토리인지 판별
    if input.is_dir() {
        let exit_code = run_directory(input, &options, cli.verbose);
        std::process::exit(exit_code);
    } else {
        let exit_code = run_single_file(input, &options, cli.verbose);
        std::process::exit(exit_code);
    }
}


