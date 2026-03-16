// imgconv - APNG 읽기/쓰기 (feature-gated)
// APNG 디코딩은 image 크레이트의 PNG 디코더로 첫 번째 프레임을 읽음 (APNG는 PNG 하위 호환)
// APNG 인코딩은 apng 크레이트 의존성이 없으므로 스텁 구현

use std::path::Path;

use image::DynamicImage;

use crate::error::ConvertError;

/// APNG 파일의 첫 번째 프레임을 디코딩하여 DynamicImage로 반환
///
/// APNG는 PNG와 하위 호환되므로 image 크레이트의 PNG 디코더가
/// 첫 번째 프레임을 정상적으로 읽을 수 있다.
pub fn decode_apng(path: &Path) -> Result<DynamicImage, ConvertError> {
    image::open(path).map_err(|e| ConvertError::DecodingError(format!("APNG 디코딩 실패: {e}")))
}

/// DynamicImage를 APNG 포맷으로 인코딩하여 저장 (단일 프레임)
pub fn encode_apng(img: &DynamicImage, output: &Path) -> Result<(), ConvertError> {
    let _ = (img, output);
    Err(ConvertError::EncodingError(
        "APNG 인코딩은 아직 구현되지 않았습니다. apng 크레이트 의존성이 필요합니다.".to_string(),
    ))
}
