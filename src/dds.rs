// imgconv - DDS 포맷 읽기 (feature-gated, 읽기 전용)

use std::path::Path;

use image::DynamicImage;

use crate::error::ConvertError;

/// DDS 파일을 디코딩하여 DynamicImage로 반환 (읽기 전용)
pub fn decode_dds(path: &Path) -> Result<DynamicImage, ConvertError> {
    let img = image::open(path)
        .map_err(|e| ConvertError::DecodingError(format!("DDS 디코딩 실패: {}", e)))?;
    Ok(img)
}
