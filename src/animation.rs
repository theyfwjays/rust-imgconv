// imgconv - 애니메이션 GIF/WebP 프레임 추출 및 조립

use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use image::codecs::gif::{GifDecoder, GifEncoder, Repeat};
use image::{AnimationDecoder, DynamicImage, Frame, RgbaImage};

use crate::error::ConvertError;

/// 프레임 추출 결과
#[derive(Debug)]
pub struct ExtractResult {
    pub frame_count: usize,
    pub output_dir: PathBuf,
    pub frame_paths: Vec<PathBuf>,
}

/// 프레임 조립 결과
#[derive(Debug)]
pub struct AssembleResult {
    pub frame_count: usize,
    pub output_path: PathBuf,
}

/// 애니메이션 GIF에서 프레임을 추출하여 개별 이미지로 저장
pub fn extract_frames(
    input: &Path,
    output_dir: &Path,
    format: &str,
) -> Result<ExtractResult, ConvertError> {
    let file = File::open(input).map_err(ConvertError::IoError)?;
    let reader = BufReader::new(file);

    let decoder = GifDecoder::new(reader)
        .map_err(|e| ConvertError::DecodingError(format!("GIF 디코딩 실패: {e}")))?;

    let frames = decoder.into_frames();
    std::fs::create_dir_all(output_dir).map_err(ConvertError::IoError)?;

    let stem = input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("frame");

    let mut frame_paths = Vec::new();
    let mut count = 0usize;

    for frame_result in frames {
        let frame = frame_result
            .map_err(|e| ConvertError::DecodingError(format!("프레임 {count} 디코딩 실패: {e}")))?;

        let img = DynamicImage::ImageRgba8(frame.into_buffer());
        let filename = format!("{stem}_{count:04}.{format}");
        let out_path = output_dir.join(&filename);

        img.save(&out_path)
            .map_err(|e| ConvertError::EncodingError(format!("프레임 {count} 저장 실패: {e}")))?;

        frame_paths.push(out_path);
        count += 1;
    }

    if count == 0 {
        return Err(ConvertError::DecodingError(
            "애니메이션 프레임이 없습니다".to_string(),
        ));
    }

    Ok(ExtractResult {
        frame_count: count,
        output_dir: output_dir.to_path_buf(),
        frame_paths,
    })
}

/// 여러 이미지 파일을 애니메이션 GIF로 조립
pub fn assemble_gif(
    input_paths: &[PathBuf],
    output: &Path,
    delay_ms: u32,
) -> Result<AssembleResult, ConvertError> {
    if input_paths.is_empty() {
        return Err(ConvertError::DecodingError(
            "조립할 프레임이 없습니다".to_string(),
        ));
    }

    let file = File::create(output).map_err(ConvertError::IoError)?;
    let mut encoder = GifEncoder::new_with_speed(file, 10);
    encoder.set_repeat(Repeat::Infinite).map_err(|e| {
        ConvertError::EncodingError(format!("GIF 인코더 설정 실패: {e}"))
    })?;

    let delay = image::Delay::from_numer_denom_ms(delay_ms, 1);

    for (i, path) in input_paths.iter().enumerate() {
        let img = image::open(path)
            .map_err(|e| ConvertError::DecodingError(format!("프레임 {i} 로드 실패: {e}")))?;
        let rgba: RgbaImage = img.to_rgba8();
        let frame = Frame::from_parts(rgba, 0, 0, delay);
        encoder.encode_frame(frame).map_err(|e| {
            ConvertError::EncodingError(format!("프레임 {i} 인코딩 실패: {e}"))
        })?;
    }

    Ok(AssembleResult {
        frame_count: input_paths.len(),
        output_path: output.to_path_buf(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{codecs::gif::GifEncoder, Frame, RgbaImage, Delay};
    use std::io::BufWriter;

    /// 테스트용 애니메이션 GIF 생성 (3프레임, 각각 다른 색)
    fn create_test_animated_gif(path: &Path, frame_count: u32) {
        let file = File::create(path).unwrap();
        let writer = BufWriter::new(file);
        let mut encoder = GifEncoder::new_with_speed(writer, 10);
        encoder.set_repeat(Repeat::Infinite).unwrap();

        let colors: Vec<[u8; 4]> = vec![
            [255, 0, 0, 255],     // red
            [0, 255, 0, 255],     // green
            [0, 0, 255, 255],     // blue
            [255, 255, 0, 255],   // yellow
            [255, 0, 255, 255],   // magenta
        ];

        for i in 0..frame_count {
            let color = colors[i as usize % colors.len()];
            let mut img = RgbaImage::new(8, 8);
            for pixel in img.pixels_mut() {
                *pixel = image::Rgba(color);
            }
            let delay = Delay::from_numer_denom_ms(100, 1);
            let frame = Frame::from_parts(img, 0, 0, delay);
            encoder.encode_frame(frame).unwrap();
        }
    }

    #[test]
    fn extract_frames_from_animated_gif() {
        let dir = tempfile::tempdir().unwrap();
        let gif_path = dir.path().join("test_anim.gif");
        create_test_animated_gif(&gif_path, 3);

        let out_dir = dir.path().join("frames");
        let result = extract_frames(&gif_path, &out_dir, "png").unwrap();

        assert_eq!(result.frame_count, 3);
        assert_eq!(result.frame_paths.len(), 3);
        for p in &result.frame_paths {
            assert!(p.exists());
        }
    }

    #[test]
    fn extract_frames_output_naming() {
        let dir = tempfile::tempdir().unwrap();
        let gif_path = dir.path().join("myimage.gif");
        create_test_animated_gif(&gif_path, 2);

        let out_dir = dir.path().join("out");
        let result = extract_frames(&gif_path, &out_dir, "png").unwrap();

        assert!(result.frame_paths[0].ends_with("myimage_0000.png"));
        assert!(result.frame_paths[1].ends_with("myimage_0001.png"));
    }

    #[test]
    fn assemble_gif_from_frames() {
        let dir = tempfile::tempdir().unwrap();

        // 프레임 이미지 3장 생성
        let mut paths = Vec::new();
        let colors: Vec<[u8; 4]> = vec![
            [255, 0, 0, 255],
            [0, 255, 0, 255],
            [0, 0, 255, 255],
        ];
        for (i, color) in colors.iter().enumerate() {
            let path = dir.path().join(format!("frame_{i}.png"));
            let mut img = RgbaImage::new(8, 8);
            for pixel in img.pixels_mut() {
                *pixel = image::Rgba(*color);
            }
            DynamicImage::ImageRgba8(img).save(&path).unwrap();
            paths.push(path);
        }

        let output = dir.path().join("assembled.gif");
        let result = assemble_gif(&paths, &output, 100).unwrap();

        assert_eq!(result.frame_count, 3);
        assert!(result.output_path.exists());

        // 조립된 GIF를 다시 디코딩해서 프레임 수 확인
        let file = File::open(&output).unwrap();
        let decoder = GifDecoder::new(BufReader::new(file)).unwrap();
        let frames: Vec<_> = decoder.into_frames().collect();
        assert_eq!(frames.len(), 3);
    }

    #[test]
    fn assemble_gif_empty_input_returns_error() {
        let dir = tempfile::tempdir().unwrap();
        let output = dir.path().join("empty.gif");
        let result = assemble_gif(&[], &output, 100);
        assert!(result.is_err());
    }

    #[test]
    fn extract_then_assemble_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let gif_path = dir.path().join("roundtrip.gif");
        create_test_animated_gif(&gif_path, 4);

        // 추출
        let frames_dir = dir.path().join("frames");
        let extract = extract_frames(&gif_path, &frames_dir, "png").unwrap();
        assert_eq!(extract.frame_count, 4);

        // 재조립
        let reassembled = dir.path().join("reassembled.gif");
        let assemble = assemble_gif(&extract.frame_paths, &reassembled, 100).unwrap();
        assert_eq!(assemble.frame_count, 4);

        // 재조립된 GIF 프레임 수 확인
        let file = File::open(&reassembled).unwrap();
        let decoder = GifDecoder::new(BufReader::new(file)).unwrap();
        let frames: Vec<_> = decoder.into_frames().collect();
        assert_eq!(frames.len(), 4);
    }
}
