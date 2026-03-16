// imgconv - 품질 설정 관리

use crate::error::ConvertError;
use crate::format::ImageFormat;

/// 포맷별 기본 품질 값 반환
///
/// 손실 압축을 지원하는 포맷에 대해 기본 품질 값을 반환한다.
/// 무손실 포맷에 대해서는 `None`을 반환한다.
///
/// - JPEG: 85
/// - WebP (lossy): 75
/// - AVIF: 70
pub fn default_quality(format: ImageFormat) -> Option<u8> {
    match format {
        ImageFormat::Jpeg => Some(85),
        ImageFormat::WebP => Some(75),
        #[cfg(feature = "avif")]
        ImageFormat::Avif => Some(70),
        _ => None,
    }
}

/// 품질 값 유효성 검증 (1-100 범위)
///
/// 유효한 값이면 그대로 반환하고, 범위를 벗어나면 `ConvertError::InvalidQuality`를 반환한다.
pub fn validate_quality(value: u8) -> Result<u8, ConvertError> {
    if value >= 1 && value <= 100 {
        Ok(value)
    } else {
        Err(ConvertError::InvalidQuality { value })
    }
}

/// 최종 품질 값을 결정하고, 무손실 포맷에 품질 지정 시 경고 메시지를 반환한다.
///
/// 반환값: `(최종 품질 값, 경고 메시지)`
/// - 손실 포맷 + 사용자 품질 지정: 사용자 값 사용
/// - 손실 포맷 + 품질 미지정: 포맷별 기본값 사용
/// - 무손실 포맷 + 품질 지정: `None` + 경고 메시지
/// - 무손실 포맷 + 품질 미지정: `None`, 경고 없음
pub fn resolve_quality(
    format: ImageFormat,
    user_quality: Option<u8>,
) -> (Option<u8>, Option<String>) {
    if format.supports_quality() {
        let quality = user_quality.or_else(|| default_quality(format));
        (quality, None)
    } else {
        if user_quality.is_some() {
            let warning = format!(
                "{} 포맷은 품질 설정을 지원하지 않습니다. --quality 옵션이 무시됩니다.",
                format
            );
            (None, Some(warning))
        } else {
            (None, None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- default_quality 테스트 ---

    /// JPEG 기본 품질은 85
    /// Validates: Requirements 9.2
    #[test]
    fn default_quality_jpeg() {
        assert_eq!(default_quality(ImageFormat::Jpeg), Some(85));
    }

    /// WebP 기본 품질은 75
    /// Validates: Requirements 9.2
    #[test]
    fn default_quality_webp() {
        assert_eq!(default_quality(ImageFormat::WebP), Some(75));
    }

    /// AVIF 기본 품질은 70
    /// Validates: Requirements 9.2
    #[cfg(feature = "avif")]
    #[test]
    fn default_quality_avif() {
        assert_eq!(default_quality(ImageFormat::Avif), Some(70));
    }

    /// 무손실 포맷은 None 반환
    /// Validates: Requirements 9.2
    #[test]
    fn default_quality_lossless_formats() {
        assert_eq!(default_quality(ImageFormat::Png), None);
        assert_eq!(default_quality(ImageFormat::Gif), None);
        assert_eq!(default_quality(ImageFormat::Bmp), None);
        assert_eq!(default_quality(ImageFormat::Tiff), None);
        assert_eq!(default_quality(ImageFormat::Svg), None);
    }

    // --- validate_quality 테스트 ---

    /// 유효 범위 내 값은 Ok 반환
    /// Validates: Requirements 9.1
    #[test]
    fn validate_quality_valid_range() {
        assert_eq!(validate_quality(1).unwrap(), 1);
        assert_eq!(validate_quality(50).unwrap(), 50);
        assert_eq!(validate_quality(100).unwrap(), 100);
    }

    /// 0은 유효 범위 밖이므로 에러 반환
    /// Validates: Requirements 9.3
    #[test]
    fn validate_quality_zero_is_invalid() {
        let err = validate_quality(0).unwrap_err();
        assert!(matches!(err, ConvertError::InvalidQuality { value: 0 }));
    }

    // --- resolve_quality 테스트 ---

    /// 손실 포맷에 사용자 품질 지정 시 해당 값 사용
    /// Validates: Requirements 9.1
    #[test]
    fn resolve_quality_lossy_with_user_value() {
        let (quality, warning) = resolve_quality(ImageFormat::Jpeg, Some(50));
        assert_eq!(quality, Some(50));
        assert!(warning.is_none());
    }

    /// 손실 포맷에 품질 미지정 시 기본값 사용
    /// Validates: Requirements 9.2
    #[test]
    fn resolve_quality_lossy_default() {
        let (quality, warning) = resolve_quality(ImageFormat::Jpeg, None);
        assert_eq!(quality, Some(85));
        assert!(warning.is_none());

        let (quality, warning) = resolve_quality(ImageFormat::WebP, None);
        assert_eq!(quality, Some(75));
        assert!(warning.is_none());
    }

    /// 무손실 포맷에 품질 지정 시 None + 경고 반환
    /// Validates: Requirements 9.4
    #[test]
    fn resolve_quality_lossless_with_user_value_warns() {
        let (quality, warning) = resolve_quality(ImageFormat::Png, Some(80));
        assert!(quality.is_none());
        assert!(warning.is_some());
        let msg = warning.unwrap();
        assert!(msg.contains("png"));
        assert!(msg.contains("품질 설정을 지원하지 않습니다"));
    }

    /// 무손실 포맷에 품질 미지정 시 None, 경고 없음
    /// Validates: Requirements 9.4
    #[test]
    fn resolve_quality_lossless_no_user_value() {
        let (quality, warning) = resolve_quality(ImageFormat::Png, None);
        assert!(quality.is_none());
        assert!(warning.is_none());
    }
}
