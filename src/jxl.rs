// imgconv - JPEG XL 디코딩 (feature-gated, 읽기 전용)

use std::path::Path;

use image::{DynamicImage, GrayAlphaImage, GrayImage, ImageBuffer, RgbImage, RgbaImage};
use jxl_oxide::{JxlImage, PixelFormat};

use crate::error::ConvertError;

/// f32 값(0.0~1.0)을 u8(0~255)로 변환
fn f32_to_u8(v: f32) -> u8 {
    (v.clamp(0.0, 1.0) * 255.0 + 0.5) as u8
}

/// JPEG XL 파일을 디코딩하여 DynamicImage로 반환 (읽기 전용)
pub fn decode_jxl(path: &Path) -> Result<DynamicImage, ConvertError> {
    let image = JxlImage::builder()
        .open(path)
        .map_err(|e| ConvertError::DecodingError(format!("JXL 디코딩 실패: {}", e)))?;

    let render = image
        .render_frame(0)
        .map_err(|e| ConvertError::DecodingError(format!("JXL 프레임 렌더링 실패: {}", e)))?;

    let mut stream = render.stream();
    let width = stream.width() as u32;
    let height = stream.height() as u32;
    let channels = stream.channels() as usize;

    let mut pixels = vec![0.0f32; (width as usize) * (height as usize) * channels];
    stream.write_to_buffer(&mut pixels);

    let pixel_format = image.pixel_format();

    match pixel_format {
        PixelFormat::Gray => {
            let buf: Vec<u8> = pixels.iter().map(|&v| f32_to_u8(v)).collect();
            let img = GrayImage::from_raw(width, height, buf).ok_or_else(|| {
                ConvertError::DecodingError("JXL 그레이스케일 이미지 생성 실패".into())
            })?;
            Ok(DynamicImage::ImageLuma8(img))
        }
        PixelFormat::Graya => {
            let buf: Vec<u8> = pixels.iter().map(|&v| f32_to_u8(v)).collect();
            let img = GrayAlphaImage::from_raw(width, height, buf).ok_or_else(|| {
                ConvertError::DecodingError("JXL 그레이스케일+알파 이미지 생성 실패".into())
            })?;
            Ok(DynamicImage::ImageLumaA8(img))
        }
        PixelFormat::Rgb => {
            let buf: Vec<u8> = pixels.iter().map(|&v| f32_to_u8(v)).collect();
            let img = RgbImage::from_raw(width, height, buf)
                .ok_or_else(|| ConvertError::DecodingError("JXL RGB 이미지 생성 실패".into()))?;
            Ok(DynamicImage::ImageRgb8(img))
        }
        PixelFormat::Rgba => {
            let buf: Vec<u8> = pixels.iter().map(|&v| f32_to_u8(v)).collect();
            let img = RgbaImage::from_raw(width, height, buf)
                .ok_or_else(|| ConvertError::DecodingError("JXL RGBA 이미지 생성 실패".into()))?;
            Ok(DynamicImage::ImageRgba8(img))
        }
        PixelFormat::Cmyk | PixelFormat::Cmyka => {
            // CMYK → RGB 변환: 간단한 공식 사용
            let px_count = (width as usize) * (height as usize);
            let has_alpha = pixel_format == PixelFormat::Cmyka;
            let mut rgba_buf = Vec::with_capacity(px_count * 4);

            for i in 0..px_count {
                let base = i * channels;
                let c = pixels[base];
                let m = pixels[base + 1];
                let y = pixels[base + 2];
                let k = pixels[base + 3];

                let r = (1.0 - c) * (1.0 - k);
                let g = (1.0 - m) * (1.0 - k);
                let b = (1.0 - y) * (1.0 - k);
                let a = if has_alpha { pixels[base + 4] } else { 1.0 };

                rgba_buf.push(f32_to_u8(r));
                rgba_buf.push(f32_to_u8(g));
                rgba_buf.push(f32_to_u8(b));
                rgba_buf.push(f32_to_u8(a));
            }

            let img: ImageBuffer<image::Rgba<u8>, Vec<u8>> =
                ImageBuffer::from_raw(width, height, rgba_buf).ok_or_else(|| {
                    ConvertError::DecodingError("JXL CMYK→RGBA 이미지 생성 실패".into())
                })?;
            Ok(DynamicImage::ImageRgba8(img))
        }
    }
}
