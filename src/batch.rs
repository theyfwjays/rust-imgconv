// imgconv - 배치 처리

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use rayon::prelude::*;

use crate::convert::{convert_file, ConvertOptions, ConvertResult};
use crate::error::ConvertError;
use crate::format::ImageFormat;

/// 진행률 콜백 (라이브러리는 indicatif에 의존하지 않음)
pub trait ProgressCallback: Send + Sync {
    fn on_start(&self, total: usize);
    fn on_progress(&self, completed: usize, file: &Path);
    fn on_complete(&self);
}

/// 배치 변환 결과
#[derive(Debug)]
pub struct BatchResult {
    pub succeeded: Vec<ConvertResult>,
    pub failed: Vec<(PathBuf, ConvertError)>,
    pub skipped: Vec<(PathBuf, String)>,
}

/// 디렉토리 내 지원 포맷 파일을 스캔한다.
fn scan_supported_files(dir: &Path) -> Result<Vec<PathBuf>, ConvertError> {
    let supported = ImageFormat::supported_extensions();
    let mut files = Vec::new();

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                let ext_lower = ext.to_ascii_lowercase();
                if supported.contains(&ext_lower.as_str()) {
                    files.push(path);
                }
            }
        }
    }

    files.sort();
    Ok(files)
}

/// 디렉토리 내 모든 이미지 파일을 병렬 변환한다.
///
/// 지원되는 확장자를 가진 파일을 스캔하고, rayon을 사용하여 병렬로 변환한다.
/// 개별 파일 변환이 실패해도 나머지 파일의 변환을 계속 진행한다.
/// 디렉토리에 지원 파일이 없으면 `NoImagesInDirectory` 에러를 반환한다.
pub fn convert_directory(
    dir: &Path,
    options: &ConvertOptions,
    progress: Option<&dyn ProgressCallback>,
) -> Result<BatchResult, ConvertError> {
    let files = scan_supported_files(dir)?;

    if files.is_empty() {
        return Err(ConvertError::NoImagesInDirectory {
            path: dir.display().to_string(),
        });
    }

    let total = files.len();

    if let Some(cb) = progress {
        cb.on_start(total);
    }

    let completed = AtomicUsize::new(0);

    // rayon par_iter로 병렬 변환, 각 파일의 결과를 수집
    let results: Vec<(PathBuf, Result<Vec<ConvertResult>, ConvertError>)> = files
        .par_iter()
        .map(|file| {
            let result = convert_file(file, options);

            let done = completed.fetch_add(1, Ordering::Relaxed) + 1;
            if let Some(cb) = progress {
                cb.on_progress(done, file);
            }

            (file.clone(), result)
        })
        .collect();

    let mut succeeded = Vec::new();
    let mut failed = Vec::new();

    for (path, result) in results {
        match result {
            Ok(convert_results) => {
                succeeded.extend(convert_results);
            }
            Err(e) => {
                failed.push((path, e));
            }
        }
    }

    if let Some(cb) = progress {
        cb.on_complete();
    }

    Ok(BatchResult { succeeded, failed, skipped: Vec::new() })
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, RgbaImage};

    /// 테스트용 작은 PNG 이미지를 생성한다.
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
        use crate::svg::SvgOptions;
        use crate::webp::WebPMode;

        ConvertOptions {
            target_formats,
            quality: None,
            resize: None,
            crop: None,
            rotate: None,
            flip: None,
            color_filter: Default::default(),
            brightness_contrast: Default::default(),
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

    /// 테스트용 ProgressCallback 구현
    struct TestProgress {
        started: std::sync::Mutex<Option<usize>>,
        progress_count: AtomicUsize,
        completed: std::sync::atomic::AtomicBool,
    }

    impl TestProgress {
        fn new() -> Self {
            Self {
                started: std::sync::Mutex::new(None),
                progress_count: AtomicUsize::new(0),
                completed: std::sync::atomic::AtomicBool::new(false),
            }
        }
    }

    impl ProgressCallback for TestProgress {
        fn on_start(&self, total: usize) {
            *self.started.lock().unwrap() = Some(total);
        }
        fn on_progress(&self, _completed: usize, _file: &Path) {
            self.progress_count.fetch_add(1, Ordering::Relaxed);
        }
        fn on_complete(&self) {
            self.completed.store(true, Ordering::Relaxed);
        }
    }

    /// 디렉토리 스캔 및 병렬 변환 테스트
    /// Validates: Requirements 6.1, 6.2
    #[test]
    fn convert_directory_basic() {
        let dir = tempfile::tempdir().unwrap();
        let out_dir = tempfile::tempdir().unwrap();

        // 3개의 PNG 파일 생성
        for name in &["a.png", "b.png", "c.png"] {
            create_test_png(&dir.path().join(name), 8, 8);
        }

        let mut options = default_options(vec![ImageFormat::Jpeg]);
        options.output_dir = Some(out_dir.path().to_path_buf());

        let progress = TestProgress::new();
        let result = convert_directory(dir.path(), &options, Some(&progress)).unwrap();

        assert_eq!(result.succeeded.len(), 3);
        assert!(result.failed.is_empty());

        // 진행률 콜백 확인
        assert_eq!(*progress.started.lock().unwrap(), Some(3));
        assert_eq!(progress.progress_count.load(Ordering::Relaxed), 3);
        assert!(progress.completed.load(Ordering::Relaxed));
    }

    /// 빈 디렉토리 에러 테스트
    /// Validates: Requirements 6.5
    #[test]
    fn convert_directory_empty_returns_error() {
        let dir = tempfile::tempdir().unwrap();
        let options = default_options(vec![ImageFormat::Jpeg]);

        let result = convert_directory(dir.path(), &options, None);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConvertError::NoImagesInDirectory { .. }
        ));
    }

    /// 지원되지 않는 파일만 있는 디렉토리 에러 테스트
    /// Validates: Requirements 6.5
    #[test]
    fn convert_directory_no_supported_files_returns_error() {
        let dir = tempfile::tempdir().unwrap();
        // 지원되지 않는 확장자 파일 생성
        std::fs::write(dir.path().join("readme.txt"), b"hello").unwrap();
        std::fs::write(dir.path().join("data.json"), b"{}").unwrap();

        let options = default_options(vec![ImageFormat::Jpeg]);
        let result = convert_directory(dir.path(), &options, None);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConvertError::NoImagesInDirectory { .. }
        ));
    }

    /// 부분 실패 시 계속 진행 테스트
    /// Validates: Requirements 6.3
    #[test]
    fn convert_directory_partial_failure_continues() {
        let dir = tempfile::tempdir().unwrap();
        let out_dir = tempfile::tempdir().unwrap();

        // 유효한 PNG 파일
        create_test_png(&dir.path().join("good.png"), 8, 8);

        // 손상된 PNG 파일 (유효하지 않은 데이터)
        std::fs::write(dir.path().join("bad.png"), b"not a real png").unwrap();

        let mut options = default_options(vec![ImageFormat::Jpeg]);
        options.output_dir = Some(out_dir.path().to_path_buf());

        let result = convert_directory(dir.path(), &options, None).unwrap();

        // 하나는 성공, 하나는 실패
        assert_eq!(result.succeeded.len(), 1);
        assert_eq!(result.failed.len(), 1);

        // 실패한 파일 경로 확인
        assert!(result.failed[0].0.file_name().unwrap() == "bad.png");
    }

    /// 진행률 콜백 없이도 정상 동작하는지 테스트
    /// Validates: Requirements 6.1
    #[test]
    fn convert_directory_without_progress_callback() {
        let dir = tempfile::tempdir().unwrap();
        let out_dir = tempfile::tempdir().unwrap();

        create_test_png(&dir.path().join("test.png"), 8, 8);

        let mut options = default_options(vec![ImageFormat::Jpeg]);
        options.output_dir = Some(out_dir.path().to_path_buf());

        let result = convert_directory(dir.path(), &options, None).unwrap();
        assert_eq!(result.succeeded.len(), 1);
        assert!(result.failed.is_empty());
    }

    /// 배치 모드에서 그레이스케일 + 리사이즈 옵션이 모든 파일에 적용되는지 검증
    /// Validates: Requirements 23.2, 24.1, 25.4, 26.1, 27.1, 28.1, 30.5
    #[test]
    fn batch_with_grayscale_and_resize() {
        let dir = tempfile::tempdir().unwrap();
        let out_dir = tempfile::tempdir().unwrap();

        // 3개의 PNG 파일 생성 (8x8)
        for name in &["img1.png", "img2.png", "img3.png"] {
            create_test_png(&dir.path().join(name), 8, 8);
        }

        let mut options = default_options(vec![ImageFormat::Png]);
        options.output_dir = Some(out_dir.path().to_path_buf());
        options.color_filter = crate::filter::ColorFilterOptions {
            grayscale: true,
            invert: false,
            sepia: false,
        };
        options.resize = Some(crate::resize::ResizeOptions {
            width: Some(4),
            height: None,
            keep_aspect: false,
        });

        let result = convert_directory(dir.path(), &options, None).unwrap();
        assert_eq!(result.succeeded.len(), 3);
        assert!(result.failed.is_empty());

        // 모든 출력 파일이 리사이즈되고 그레이스케일이 적용되었는지 확인
        for r in &result.succeeded {
            let img = image::open(&r.output_path).unwrap();
            assert_eq!(img.width(), 4);
            // 그레이스케일 확인: R=G=B
            let rgba = img.to_rgba8();
            let pixel = rgba.get_pixel(0, 0);
            assert_eq!(pixel[0], pixel[1]);
            assert_eq!(pixel[1], pixel[2]);
        }
    }
}
