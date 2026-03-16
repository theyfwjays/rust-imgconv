// imgconv - 이미지 품질 비교 모듈
#![cfg(feature = "compare")]

use std::path::Path;

use image::DynamicImage;

use crate::error::ConvertError;

/// 이미지 비교 결과
#[derive(Debug)]
pub struct CompareResult {
    /// SSIM (Structural Similarity Index Measure), 0.0 ~ 1.0
    pub ssim: f64,
    /// PSNR (Peak Signal-to-Noise Ratio), dB 단위
    pub psnr: f64,
}

/// 두 이미지 간 SSIM 및 PSNR 메트릭 계산
///
/// 두 이미지의 크기가 동일해야 하며, 다를 경우 `CompareSizeMismatch` 에러를 반환한다.
pub fn compare_images(path_a: &Path, path_b: &Path) -> Result<CompareResult, ConvertError> {
    let img_a = image::open(path_a)
        .map_err(|e| ConvertError::CompareError(format!("이미지 A 로딩 실패: {e}")))?;
    let img_b = image::open(path_b)
        .map_err(|e| ConvertError::CompareError(format!("이미지 B 로딩 실패: {e}")))?;

    let (w1, h1) = (img_a.width(), img_a.height());
    let (w2, h2) = (img_b.width(), img_b.height());

    if w1 != w2 || h1 != h2 {
        return Err(ConvertError::CompareSizeMismatch { w1, h1, w2, h2 });
    }

    let ssim = calculate_ssim(&img_a, &img_b)?;
    let psnr = calculate_psnr(&img_a, &img_b);

    Ok(CompareResult { ssim, psnr })
}

/// image-compare 크레이트를 사용하여 SSIM 계산
fn calculate_ssim(img_a: &DynamicImage, img_b: &DynamicImage) -> Result<f64, ConvertError> {
    let rgb_a = img_a.to_rgb8();
    let rgb_b = img_b.to_rgb8();

    let result = image_compare::rgb_similarity_structure(
        &image_compare::Algorithm::MSSIMSimple,
        &rgb_a,
        &rgb_b,
    )
    .map_err(|e| ConvertError::CompareError(format!("SSIM 계산 실패: {e}")))?;

    Ok(result.score)
}

/// PSNR 수동 계산: PSNR = 10 * log10(MAX^2 / MSE)
/// MAX = 255 (8-bit 이미지)
fn calculate_psnr(img_a: &DynamicImage, img_b: &DynamicImage) -> f64 {
    let rgba_a = img_a.to_rgba8();
    let rgba_b = img_b.to_rgba8();

    let pixels_a = rgba_a.as_raw();
    let pixels_b = rgba_b.as_raw();

    let mut sum_sq_diff: f64 = 0.0;
    let mut count: u64 = 0;

    // RGB 채널만 비교 (알파 채널 제외)
    for (chunk_a, chunk_b) in pixels_a.chunks(4).zip(pixels_b.chunks(4)) {
        for i in 0..3 {
            let diff = chunk_a[i] as f64 - chunk_b[i] as f64;
            sum_sq_diff += diff * diff;
            count += 1;
        }
    }

    if count == 0 {
        return f64::INFINITY;
    }

    let mse = sum_sq_diff / count as f64;

    if mse == 0.0 {
        return f64::INFINITY; // 동일한 이미지
    }

    let max_val: f64 = 255.0;
    10.0 * (max_val * max_val / mse).log10()
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, RgbImage};
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn save_test_image(img: &DynamicImage) -> NamedTempFile {
        let mut tmp = NamedTempFile::with_suffix(".png").unwrap();
        let mut buf = std::io::Cursor::new(Vec::new());
        img.write_to(&mut buf, image::ImageFormat::Png).unwrap();
        tmp.write_all(buf.get_ref()).unwrap();
        tmp.flush().unwrap();
        tmp
    }

    #[test]
    fn test_identical_images_ssim_one() {
        let img = DynamicImage::ImageRgb8(RgbImage::from_fn(64, 64, |x, y| {
            image::Rgb([(x * 4) as u8, (y * 4) as u8, 128])
        }));
        let file_a = save_test_image(&img);
        let file_b = save_test_image(&img);

        let result = compare_images(file_a.path(), file_b.path()).unwrap();
        assert!(
            (result.ssim - 1.0).abs() < 1e-6,
            "SSIM should be 1.0 for identical images, got {}",
            result.ssim
        );
        assert!(
            result.psnr.is_infinite(),
            "PSNR should be infinity for identical images, got {}",
            result.psnr
        );
    }

    #[test]
    fn test_different_size_images_error() {
        let img_a = DynamicImage::ImageRgb8(RgbImage::new(64, 64));
        let img_b = DynamicImage::ImageRgb8(RgbImage::new(32, 32));
        let file_a = save_test_image(&img_a);
        let file_b = save_test_image(&img_b);

        let result = compare_images(file_a.path(), file_b.path());
        assert!(result.is_err());
        match result.unwrap_err() {
            ConvertError::CompareSizeMismatch { w1, h1, w2, h2 } => {
                assert_eq!((w1, h1), (64, 64));
                assert_eq!((w2, h2), (32, 32));
            }
            e => panic!("Expected CompareSizeMismatch, got: {e:?}"),
        }
    }

    #[test]
    fn test_different_images_lower_ssim() {
        let img_a = DynamicImage::ImageRgb8(RgbImage::from_fn(64, 64, |_, _| {
            image::Rgb([255, 255, 255])
        }));
        let img_b =
            DynamicImage::ImageRgb8(RgbImage::from_fn(64, 64, |_, _| image::Rgb([0, 0, 0])));
        let file_a = save_test_image(&img_a);
        let file_b = save_test_image(&img_b);

        let result = compare_images(file_a.path(), file_b.path()).unwrap();
        assert!(
            result.ssim < 0.5,
            "SSIM should be low for very different images, got {}",
            result.ssim
        );
        assert!(
            result.psnr.is_finite(),
            "PSNR should be finite for different images"
        );
        assert!(result.psnr >= 0.0, "PSNR should be non-negative");
    }

    #[test]
    fn test_psnr_calculation() {
        let img_a = DynamicImage::ImageRgb8(RgbImage::from_fn(64, 64, |_, _| {
            image::Rgb([100, 100, 100])
        }));
        let img_b = DynamicImage::ImageRgb8(RgbImage::from_fn(64, 64, |_, _| {
            image::Rgb([110, 100, 100])
        }));

        // MSE = (10^2 * 64*64) / (3 * 64*64) = 100/3 ≈ 33.33
        // PSNR = 10 * log10(255^2 / 33.33) ≈ 10 * log10(1950.75) ≈ 32.9 dB
        let psnr = calculate_psnr(&img_a, &img_b);
        assert!(
            psnr > 30.0 && psnr < 40.0,
            "PSNR should be around 33 dB, got {psnr}"
        );
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use image::{DynamicImage, Rgba, RgbaImage};
    use proptest::prelude::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn save_test_png(img: &DynamicImage) -> NamedTempFile {
        let mut tmp = NamedTempFile::with_suffix(".png").unwrap();
        let mut buf = std::io::Cursor::new(Vec::new());
        img.write_to(&mut buf, image::ImageFormat::Png).unwrap();
        tmp.write_all(buf.get_ref()).unwrap();
        tmp.flush().unwrap();
        tmp
    }

    // Property 24: 동일 이미지 비교 시 SSIM 1.0 — 자기 자신과 비교 시 SSIM = 1.0
    // **Validates: Requirements 33.1**
    proptest! {
        #[test]
        fn prop24_identical_image_ssim_one(
            w in 8u32..=64,
            h in 8u32..=64,
            r in 0u8..=255,
            g in 0u8..=255,
            b in 0u8..=255,
        ) {
            let img = DynamicImage::ImageRgba8(RgbaImage::from_fn(w, h, |x, y| {
                Rgba([
                    r.wrapping_add((x % 16) as u8),
                    g.wrapping_add((y % 16) as u8),
                    b,
                    255,
                ])
            }));
            let file_a = save_test_png(&img);
            let file_b = save_test_png(&img);

            let result = compare_images(file_a.path(), file_b.path()).unwrap();
            prop_assert!(
                (result.ssim - 1.0).abs() < 1e-6,
                "SSIM should be 1.0 for identical images, got {}",
                result.ssim
            );
            prop_assert!(
                result.psnr.is_infinite(),
                "PSNR should be infinity for identical images, got {}",
                result.psnr
            );
        }
    }

    // Property 25: 크기가 다른 이미지 비교 시 에러 — 크기 다른 이미지 비교 시 `CompareSizeMismatch` 에러
    // **Validates: Requirements 33.3**
    proptest! {
        #[test]
        fn prop25_different_size_compare_error(
            w1 in 8u32..=64,
            h1 in 8u32..=64,
            w2 in 8u32..=64,
            h2 in 8u32..=64,
        ) {
            prop_assume!(w1 != w2 || h1 != h2);

            let img_a = DynamicImage::ImageRgba8(RgbaImage::new(w1, h1));
            let img_b = DynamicImage::ImageRgba8(RgbaImage::new(w2, h2));
            let file_a = save_test_png(&img_a);
            let file_b = save_test_png(&img_b);

            let result = compare_images(file_a.path(), file_b.path());
            match result {
                Err(ConvertError::CompareSizeMismatch { w1: rw1, h1: rh1, w2: rw2, h2: rh2 }) => {
                    prop_assert_eq!(rw1, w1);
                    prop_assert_eq!(rh1, h1);
                    prop_assert_eq!(rw2, w2);
                    prop_assert_eq!(rh2, h2);
                }
                other => prop_assert!(false, "Expected CompareSizeMismatch, got: {:?}", other),
            }
        }
    }

    // Property 23: 정보/비교 모드 파일 시스템 무변경 — 비교 모드 실행 시 파일 시스템 변경 없음
    // **Validates: Requirements 33.2**
    proptest! {
        #[test]
        fn prop23_compare_no_filesystem_changes(
            w in 8u32..=64,
            h in 8u32..=64,
        ) {
            let dir = tempfile::tempdir().unwrap();
            let img = DynamicImage::ImageRgba8(RgbaImage::new(w, h));

            let path_a = dir.path().join("a.png");
            let path_b = dir.path().join("b.png");
            img.save(&path_a).unwrap();
            img.save(&path_b).unwrap();

            // Record directory contents before compare
            let before: std::collections::BTreeSet<_> = std::fs::read_dir(dir.path())
                .unwrap()
                .filter_map(|e| e.ok().map(|e| e.file_name()))
                .collect();

            let _ = compare_images(&path_a, &path_b);

            // Verify no new files created
            let after: std::collections::BTreeSet<_> = std::fs::read_dir(dir.path())
                .unwrap()
                .filter_map(|e| e.ok().map(|e| e.file_name()))
                .collect();

            prop_assert_eq!(&before, &after, "Compare should not create/delete any files");
        }
    }
}
