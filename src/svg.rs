// imgconv - SVG 래스터화/트레이싱

use std::path::Path;

use image::DynamicImage;
use image::RgbaImage;
use resvg::usvg;
use resvg::tiny_skia::{Pixmap, Transform};

use crate::error::ConvertError;

/// SVG 변환 옵션
#[derive(Debug, Clone)]
pub struct SvgOptions {
    /// SVG 래스터화 DPI (기본값: 96)
    pub dpi: f32,
    /// SVG 트레이싱 프리셋
    pub preset: SvgPreset,
}

impl Default for SvgOptions {
    fn default() -> Self {
        Self {
            dpi: 96.0,
            preset: SvgPreset::Default,
        }
    }
}

/// SVG 트레이싱 프리셋
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SvgPreset {
    Default,
    Bw,
    Poster,
    Photo,
}

/// SVG 파일을 래스터 이미지로 변환
///
/// resvg + tiny-skia를 사용하여 SVG를 래스터화한다.
/// DPI 옵션으로 해상도를 조절하고, width 옵션으로 출력 너비를 지정할 수 있다.
pub fn rasterize_svg(
    path: &Path,
    options: &SvgOptions,
    width: Option<u32>,
) -> Result<DynamicImage, ConvertError> {
    // SVG 파일 읽기
    let svg_data = std::fs::read(path).map_err(|e| {
        ConvertError::SvgError(format!("SVG 파일 읽기 실패: {e}"))
    })?;

    // usvg 옵션 설정 (DPI 적용)
    let opt = usvg::Options {
        dpi: options.dpi,
        ..usvg::Options::default()
    };

    // SVG 파싱
    let tree = usvg::Tree::from_data(&svg_data, &opt).map_err(|e| {
        ConvertError::SvgError(format!("SVG 파싱 실패: {e}"))
    })?;

    // 원본 SVG 크기
    let svg_size = tree.size();
    let original_width = svg_size.width();
    let original_height = svg_size.height();

    // DPI 스케일 계산 (기본 96 DPI 기준)
    let dpi_scale = options.dpi / 96.0;

    // 최종 출력 크기 계산
    let (target_width, target_height, scale_x, scale_y) = if let Some(w) = width {
        // width 옵션이 지정된 경우: 종횡비 유지하며 해당 너비로 스케일
        let aspect = original_height / original_width;
        let h = (w as f32 * aspect).round() as u32;
        let sx = w as f32 / original_width;
        let sy = h as f32 / original_height;
        (w, h, sx, sy)
    } else {
        // DPI만 적용
        let w = (original_width * dpi_scale).round() as u32;
        let h = (original_height * dpi_scale).round() as u32;
        (w, h, dpi_scale, dpi_scale)
    };

    // Pixmap 생성
    let mut pixmap = Pixmap::new(target_width, target_height).ok_or_else(|| {
        ConvertError::SvgError(format!(
            "Pixmap 생성 실패: {target_width}x{target_height}"
        ))
    })?;

    // 스케일 트랜스폼 적용하여 렌더링
    let transform = Transform::from_scale(scale_x, scale_y);
    resvg::render(&tree, transform, &mut pixmap.as_mut());

    // tiny-skia Pixmap → DynamicImage 변환
    // Pixmap 데이터는 premultiplied RGBA이므로 demultiply 필요
    let pixmap_data = pixmap.data();
    let mut rgba_data = Vec::with_capacity(pixmap_data.len());

    for pixel in pixmap_data.chunks_exact(4) {
        let a = pixel[3];
        if a == 0 {
            rgba_data.extend_from_slice(&[0, 0, 0, 0]);
        } else if a == 255 {
            rgba_data.extend_from_slice(pixel);
        } else {
            // Demultiply alpha
            let r = ((pixel[0] as u16 * 255) / a as u16).min(255) as u8;
            let g = ((pixel[1] as u16 * 255) / a as u16).min(255) as u8;
            let b = ((pixel[2] as u16 * 255) / a as u16).min(255) as u8;
            rgba_data.extend_from_slice(&[r, g, b, a]);
        }
    }

    let img = RgbaImage::from_raw(target_width, target_height, rgba_data).ok_or_else(|| {
        ConvertError::SvgError("Pixmap → DynamicImage 변환 실패".to_string())
    })?;

    Ok(DynamicImage::ImageRgba8(img))
}

/// SvgPreset을 vtracer Config으로 변환
fn to_vtracer_config(preset: SvgPreset) -> vtracer::Config {
    match preset {
        SvgPreset::Default => vtracer::Config::default(),
        SvgPreset::Bw => vtracer::Config::from_preset(vtracer::Preset::Bw),
        SvgPreset::Poster => vtracer::Config::from_preset(vtracer::Preset::Poster),
        SvgPreset::Photo => vtracer::Config::from_preset(vtracer::Preset::Photo),
    }
}

/// 래스터 이미지를 SVG 문자열로 트레이싱
///
/// vtracer를 사용하여 DynamicImage를 SVG 벡터 그래픽으로 변환한다.
/// 프리셋에 따라 트레이싱 파라미터가 조정된다.
pub fn trace_to_svg(img: &DynamicImage, preset: SvgPreset) -> Result<String, ConvertError> {
    let rgba = img.to_rgba8();
    let (width, height) = (rgba.width() as usize, rgba.height() as usize);
    let pixels = rgba.into_raw();

    let color_image = vtracer::ColorImage {
        pixels,
        width,
        height,
    };

    let config = to_vtracer_config(preset);

    let svg_file = vtracer::convert(color_image, config)
        .map_err(|e| ConvertError::SvgError(format!("SVG 트레이싱 실패: {e}")))?;

    Ok(svg_file.to_string())
}

/// SVG 문자열을 파일로 저장
pub fn save_svg(svg_content: &str, path: &Path) -> Result<(), ConvertError> {
    std::fs::write(path, svg_content).map_err(|e| {
        ConvertError::SvgError(format!("SVG 파일 저장 실패: {e}"))
    })
}


#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, RgbaImage};
    use std::path::PathBuf;

    const SIMPLE_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
  <rect width="100" height="100" fill="red"/>
</svg>"#;

    fn temp_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("imgconv_svg_test_{name}"))
    }

    fn write_test_svg(name: &str) -> PathBuf {
        let path = temp_path(name);
        std::fs::write(&path, SIMPLE_SVG).unwrap();
        path
    }

    fn create_test_image(w: u32, h: u32) -> DynamicImage {
        let mut img = RgbaImage::new(w, h);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            *pixel = image::Rgba([
                (x % 256) as u8,
                (y % 256) as u8,
                ((x + y) % 256) as u8,
                255,
            ]);
        }
        DynamicImage::ImageRgba8(img)
    }

    /// SVG 래스터화 기본 DPI(96) 테스트: 100x100 SVG → 100x100 래스터
    /// Validates: Requirements 3.1, 3.3
    #[test]
    fn rasterize_svg_default_dpi() {
        let svg_path = write_test_svg("default_dpi.svg");
        let options = SvgOptions::default();

        let img = rasterize_svg(&svg_path, &options, None).unwrap();

        assert_eq!(img.width(), 100);
        assert_eq!(img.height(), 100);

        std::fs::remove_file(&svg_path).ok();
    }

    /// SVG 래스터화 커스텀 DPI 테스트: DPI 192 → 크기 2배
    /// Validates: Requirements 3.2
    #[test]
    fn rasterize_svg_custom_dpi() {
        let svg_path = write_test_svg("custom_dpi.svg");
        let options = SvgOptions {
            dpi: 192.0,
            preset: SvgPreset::Default,
        };

        let img = rasterize_svg(&svg_path, &options, None).unwrap();

        // 192 / 96 = 2x scale
        assert_eq!(img.width(), 200);
        assert_eq!(img.height(), 200);

        std::fs::remove_file(&svg_path).ok();
    }

    /// SVG 래스터화 width 옵션 테스트: 종횡비 유지
    /// Validates: Requirements 3.1, 3.3
    #[test]
    fn rasterize_svg_with_width_option() {
        let svg_path = write_test_svg("with_width.svg");
        let options = SvgOptions::default();

        let img = rasterize_svg(&svg_path, &options, Some(50)).unwrap();

        assert_eq!(img.width(), 50);
        // 100x100 SVG → width 50 → height 50 (1:1 aspect ratio)
        assert_eq!(img.height(), 50);

        std::fs::remove_file(&svg_path).ok();
    }

    /// Default 프리셋으로 SVG 트레이싱: 유효한 SVG 문자열 생성 확인
    /// Validates: Requirements 3.5, 3.7
    #[test]
    fn trace_to_svg_default_preset() {
        let img = create_test_image(32, 32);

        let svg_str = trace_to_svg(&img, SvgPreset::Default).unwrap();

        assert!(svg_str.contains("<svg"));
        assert!(svg_str.contains("</svg>"));
    }

    /// Bw, Poster, Photo 프리셋으로 SVG 트레이싱: 각각 유효한 SVG 출력 확인
    /// Validates: Requirements 3.5, 3.6
    #[test]
    fn trace_to_svg_all_presets() {
        let img = create_test_image(32, 32);

        for preset in [SvgPreset::Bw, SvgPreset::Poster, SvgPreset::Photo] {
            let svg_str = trace_to_svg(&img, preset).unwrap();
            assert!(
                svg_str.contains("<svg"),
                "{preset:?} preset did not produce valid SVG"
            );
            assert!(
                svg_str.contains("</svg>"),
                "{preset:?} preset SVG not properly closed"
            );
        }
    }

    /// save_svg 유틸리티: SVG 문자열이 파일에 올바르게 저장되는지 확인
    /// Validates: Requirements 3.5
    #[test]
    fn save_svg_writes_file_correctly() {
        let path = temp_path("save_test.svg");
        let content = r#"<svg xmlns="http://www.w3.org/2000/svg"><circle r="10"/></svg>"#;

        save_svg(content, &path).unwrap();

        let read_back = std::fs::read_to_string(&path).unwrap();
        assert_eq!(read_back, content);

        std::fs::remove_file(&path).ok();
    }
}
