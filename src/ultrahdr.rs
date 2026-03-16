// imgconv - Ultra HDR JPEG 읽기/쓰기 (feature-gated)
// ultrahdr-core 크레이트가 아직 사용 불가하므로 스텁 구현

use std::path::Path;

use image::DynamicImage;

use crate::error::ConvertError;

/// Ultra HDR JPEG 파일을 디코딩하여 DynamicImage로 반환
pub fn decode_ultrahdr(path: &Path) -> Result<DynamicImage, ConvertError> {
    let _ = path;
    Err(ConvertError::DecodingError(
        "Ultra HDR 디코딩은 아직 구현되지 않았습니다. ultrahdr-core 크레이트가 필요합니다.".to_string(),
    ))
}

/// DynamicImage를 Ultra HDR JPEG 포맷으로 인코딩하여 저장
pub fn encode_ultrahdr(img: &DynamicImage, output: &Path) -> Result<(), ConvertError> {
    let _ = (img, output);
    Err(ConvertError::EncodingError(
        "Ultra HDR 인코딩은 아직 구현되지 않았습니다. ultrahdr-core 크레이트가 필요합니다.".to_string(),
    ))
}
