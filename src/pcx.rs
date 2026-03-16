// imgconv - PCX 포맷 읽기/쓰기 (feature-gated)

use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use image::{DynamicImage, RgbImage};

use crate::error::ConvertError;

/// PCX 파일을 디코딩하여 DynamicImage로 반환
pub fn decode_pcx(path: &Path) -> Result<DynamicImage, ConvertError> {
    let file = File::open(path)
        .map_err(|e| ConvertError::DecodingError(format!("PCX 파일 열기 실패: {}", e)))?;
    let buf_reader = BufReader::new(file);

    let mut reader = pcx::Reader::new(buf_reader)
        .map_err(|e| ConvertError::DecodingError(format!("PCX 디코딩 실패: {}", e)))?;

    let width = reader.width() as u32;
    let height = reader.height() as u32;

    if reader.is_paletted() {
        // Read paletted rows
        let mut indices = vec![0u8; (width as usize) * (height as usize)];
        for y in 0..height {
            let start = (y as usize) * (width as usize);
            let end = start + (width as usize);
            reader
                .next_row_paletted(&mut indices[start..end])
                .map_err(|e| {
                    ConvertError::DecodingError(format!("PCX 팔레트 행 읽기 실패 (y={}): {}", y, e))
                })?;
        }

        // Read palette
        let mut palette = [0u8; 768];
        let num_colors = reader
            .read_palette(&mut palette)
            .map_err(|e| ConvertError::DecodingError(format!("PCX 팔레트 읽기 실패: {}", e)))?;

        if num_colors == 0 {
            return Err(ConvertError::DecodingError(
                "PCX 팔레트가 비어있습니다".into(),
            ));
        }

        // Map palette indices to RGB
        let mut rgb_buf = Vec::with_capacity((width as usize) * (height as usize) * 3);
        for &idx in &indices {
            let base = (idx as usize) * 3;
            if base + 2 < palette.len() {
                rgb_buf.push(palette[base]);
                rgb_buf.push(palette[base + 1]);
                rgb_buf.push(palette[base + 2]);
            } else {
                rgb_buf.push(0);
                rgb_buf.push(0);
                rgb_buf.push(0);
            }
        }

        let img = RgbImage::from_raw(width, height, rgb_buf)
            .ok_or_else(|| ConvertError::DecodingError("PCX 팔레트 이미지 생성 실패".into()))?;
        Ok(DynamicImage::ImageRgb8(img))
    } else {
        // 24-bit RGB image: read interleaved rows
        let mut rgb_buf = vec![0u8; (width as usize) * (height as usize) * 3];
        for y in 0..height {
            let start = (y as usize) * (width as usize) * 3;
            let end = start + (width as usize) * 3;
            reader.next_row_rgb(&mut rgb_buf[start..end]).map_err(|e| {
                ConvertError::DecodingError(format!("PCX RGB 행 읽기 실패 (y={}): {}", y, e))
            })?;
        }

        let img = RgbImage::from_raw(width, height, rgb_buf)
            .ok_or_else(|| ConvertError::DecodingError("PCX RGB 이미지 생성 실패".into()))?;
        Ok(DynamicImage::ImageRgb8(img))
    }
}

/// DynamicImage를 PCX 포맷으로 인코딩하여 저장
pub fn encode_pcx(img: &DynamicImage, output: &Path) -> Result<(), ConvertError> {
    let rgb_img = img.to_rgb8();
    let width = rgb_img.width() as u16;
    let height = rgb_img.height() as u16;

    let file = File::create(output)
        .map_err(|e| ConvertError::EncodingError(format!("PCX 출력 파일 생성 실패: {}", e)))?;
    let buf_writer = BufWriter::new(file);

    let mut writer = pcx::WriterRgb::new(buf_writer, (width, height), (300, 300))
        .map_err(|e| ConvertError::EncodingError(format!("PCX 라이터 생성 실패: {}", e)))?;

    for y in 0..height {
        let start = (y as usize) * (width as usize) * 3;
        let end = start + (width as usize) * 3;
        let row = &rgb_img.as_raw()[start..end];
        writer.write_row(row).map_err(|e| {
            ConvertError::EncodingError(format!("PCX 행 쓰기 실패 (y={}): {}", y, e))
        })?;
    }

    writer
        .finish()
        .map_err(|e| ConvertError::EncodingError(format!("PCX 파일 마무리 실패: {}", e)))?;

    Ok(())
}

#[cfg(test)]
mod proptests {
    use super::*;
    use image::{DynamicImage, RgbImage};
    use proptest::prelude::*;
    use tempfile::tempdir;

    // Property 2: PCX 라운드트립 — PCX 인코딩 후 디코딩 시 원본과 동일한 너비/높이
    // PCX는 RGB 데이터에 대해 무손실이므로 픽셀 값도 일치해야 함
    // **Validates: Requirements 20.1**
    proptest! {
        #[test]
        fn prop2_pcx_roundtrip(
            w in 1u32..=64,
            h in 1u32..=64,
            seed in any::<u64>(),
        ) {
            // Generate random RGB pixel data deterministically from seed
            let pixel_count = (w * h * 3) as usize;
            let mut pixels = Vec::with_capacity(pixel_count);
            let mut rng_state = seed;
            for _ in 0..pixel_count {
                // Simple xorshift-style PRNG for deterministic pixel generation
                rng_state ^= rng_state << 13;
                rng_state ^= rng_state >> 7;
                rng_state ^= rng_state << 17;
                pixels.push((rng_state & 0xFF) as u8);
            }

            let original = RgbImage::from_raw(w, h, pixels.clone())
                .expect("failed to create test image");
            let original_img = DynamicImage::ImageRgb8(original);

            let dir = tempdir().expect("failed to create temp dir");
            let pcx_path = dir.path().join("test.pcx");

            // Encode
            encode_pcx(&original_img, &pcx_path).expect("encode_pcx failed");

            // Decode
            let decoded_img = decode_pcx(&pcx_path).expect("decode_pcx failed");

            // Verify dimensions match
            prop_assert_eq!(decoded_img.width(), w, "width mismatch");
            prop_assert_eq!(decoded_img.height(), h, "height mismatch");

            // Verify pixel data matches (PCX is lossless for RGB)
            let decoded_rgb = decoded_img.to_rgb8();
            let decoded_pixels = decoded_rgb.as_raw();
            prop_assert_eq!(decoded_pixels.len(), pixels.len(), "pixel buffer length mismatch");
            prop_assert!(
                decoded_pixels == &pixels[..],
                "pixel data mismatch after PCX roundtrip"
            );
        }
    }
}
