// imgconv - 포맷 감지 및 열거형

use crate::error::ConvertError;

/// 지원하는 모든 이미지 포맷
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImageFormat {
    Jpeg,
    Png,
    Gif,
    Bmp,
    Tiff,
    Tga,
    Ico,
    Qoi,
    Pnm,
    OpenExr,
    Hdr,
    Farbfeld,
    WebP,
    Svg,
    #[cfg(feature = "avif")]
    Avif,
    #[cfg(feature = "jxl")]
    Jxl,
    #[cfg(feature = "dds")]
    Dds,
    #[cfg(feature = "pcx")]
    Pcx,
    #[cfg(feature = "ultrahdr")]
    UltraHdr,
    #[cfg(feature = "apng")]
    Apng,
}

impl ImageFormat {
    /// 파일 확장자로부터 포맷 감지
    pub fn from_extension(ext: &str) -> Result<Self, ConvertError> {
        let ext = ext.trim_start_matches('.').to_ascii_lowercase();
        match ext.as_str() {
            "jpg" | "jpeg" => Ok(Self::Jpeg),
            "png" => Ok(Self::Png),
            "gif" => Ok(Self::Gif),
            "bmp" => Ok(Self::Bmp),
            "tif" | "tiff" => Ok(Self::Tiff),
            "tga" => Ok(Self::Tga),
            "ico" => Ok(Self::Ico),
            "qoi" => Ok(Self::Qoi),
            "ppm" | "pgm" | "pbm" | "pam" => Ok(Self::Pnm),
            "exr" => Ok(Self::OpenExr),
            "hdr" => Ok(Self::Hdr),
            "ff" => Ok(Self::Farbfeld),
            "webp" => Ok(Self::WebP),
            "svg" => Ok(Self::Svg),
            #[cfg(feature = "avif")]
            "avif" => Ok(Self::Avif),
            #[cfg(not(feature = "avif"))]
            "avif" => Err(ConvertError::AvifNotEnabled),
            #[cfg(feature = "jxl")]
            "jxl" => Ok(Self::Jxl),
            #[cfg(not(feature = "jxl"))]
            "jxl" => Err(ConvertError::JxlNotEnabled),
            #[cfg(feature = "dds")]
            "dds" => Ok(Self::Dds),
            #[cfg(not(feature = "dds"))]
            "dds" => Err(ConvertError::DdsNotEnabled),
            #[cfg(feature = "pcx")]
            "pcx" => Ok(Self::Pcx),
            #[cfg(not(feature = "pcx"))]
            "pcx" => Err(ConvertError::PcxNotEnabled),
            #[cfg(feature = "ultrahdr")]
            "uhdr.jpg" => Ok(Self::UltraHdr),
            #[cfg(not(feature = "ultrahdr"))]
            "uhdr.jpg" => Err(ConvertError::UltraHdrNotEnabled),
            #[cfg(feature = "apng")]
            "apng" => Ok(Self::Apng),
            #[cfg(not(feature = "apng"))]
            "apng" => Err(ConvertError::ApngNotEnabled),
            _ => Err(ConvertError::UnsupportedInputFormat {
                extension: ext,
                supported: Self::supported_extensions().join(", "),
            }),
        }
    }

    /// 포맷에 해당하는 기본 확장자 반환
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Jpeg => "jpg",
            Self::Png => "png",
            Self::Gif => "gif",
            Self::Bmp => "bmp",
            Self::Tiff => "tiff",
            Self::Tga => "tga",
            Self::Ico => "ico",
            Self::Qoi => "qoi",
            Self::Pnm => "ppm",
            Self::OpenExr => "exr",
            Self::Hdr => "hdr",
            Self::Farbfeld => "ff",
            Self::WebP => "webp",
            Self::Svg => "svg",
            #[cfg(feature = "avif")]
            Self::Avif => "avif",
            #[cfg(feature = "jxl")]
            Self::Jxl => "jxl",
            #[cfg(feature = "dds")]
            Self::Dds => "dds",
            #[cfg(feature = "pcx")]
            Self::Pcx => "pcx",
            #[cfg(feature = "ultrahdr")]
            Self::UltraHdr => "uhdr.jpg",
            #[cfg(feature = "apng")]
            Self::Apng => "apng",
        }
    }

    /// 손실 압축을 지원하는 포맷인지 확인
    pub fn supports_quality(&self) -> bool {
        match self {
            Self::Jpeg | Self::WebP => true,
            #[cfg(feature = "avif")]
            Self::Avif => true,
            #[cfg(feature = "ultrahdr")]
            Self::UltraHdr => true,
            _ => false,
        }
    }

    /// 쓰기를 지원하는 포맷인지 확인
    pub fn supports_write(&self) -> bool {
        match self {
            #[cfg(feature = "jxl")]
            Self::Jxl => false,
            #[cfg(feature = "dds")]
            Self::Dds => false,
            _ => true,
        }
    }

    /// 지원되는 모든 확장자 목록 반환
    pub fn supported_extensions() -> Vec<&'static str> {
        #[allow(unused_mut)]
        let mut exts = vec![
            "jpg", "jpeg", "png", "gif", "bmp", "tif", "tiff", "tga", "ico", "qoi",
            "ppm", "pgm", "pbm", "pam", "exr", "hdr", "ff", "webp", "svg",
        ];
        #[cfg(feature = "avif")]
        exts.push("avif");
        #[cfg(feature = "jxl")]
        exts.push("jxl");
        #[cfg(feature = "dds")]
        exts.push("dds");
        #[cfg(feature = "pcx")]
        exts.push("pcx");
        #[cfg(feature = "ultrahdr")]
        exts.push("uhdr.jpg");
        #[cfg(feature = "apng")]
        exts.push("apng");
        exts
    }
}

/// 쉼표 구분 문자열에서 여러 포맷 파싱
///
/// 예: "png,webp,jpeg" → vec![ImageFormat::Png, ImageFormat::WebP, ImageFormat::Jpeg]
pub fn parse_formats(s: &str) -> Result<Vec<ImageFormat>, ConvertError> {
    s.split(',')
        .map(|part| {
            let name = part.trim().to_ascii_lowercase();
            match name.as_str() {
                "jpeg" | "jpg" => Ok(ImageFormat::Jpeg),
                "png" => Ok(ImageFormat::Png),
                "gif" => Ok(ImageFormat::Gif),
                "bmp" => Ok(ImageFormat::Bmp),
                "tiff" | "tif" => Ok(ImageFormat::Tiff),
                "tga" => Ok(ImageFormat::Tga),
                "ico" => Ok(ImageFormat::Ico),
                "qoi" => Ok(ImageFormat::Qoi),
                "pnm" | "ppm" | "pgm" | "pbm" | "pam" => Ok(ImageFormat::Pnm),
                "openexr" | "exr" => Ok(ImageFormat::OpenExr),
                "hdr" => Ok(ImageFormat::Hdr),
                "farbfeld" | "ff" => Ok(ImageFormat::Farbfeld),
                "webp" => Ok(ImageFormat::WebP),
                "svg" => Ok(ImageFormat::Svg),
                #[cfg(feature = "avif")]
                "avif" => Ok(ImageFormat::Avif),
                #[cfg(not(feature = "avif"))]
                "avif" => Err(ConvertError::AvifNotEnabled),
                #[cfg(feature = "jxl")]
                "jxl" => Ok(ImageFormat::Jxl),
                #[cfg(not(feature = "jxl"))]
                "jxl" => Err(ConvertError::JxlNotEnabled),
                #[cfg(feature = "dds")]
                "dds" => Ok(ImageFormat::Dds),
                #[cfg(not(feature = "dds"))]
                "dds" => Err(ConvertError::DdsNotEnabled),
                #[cfg(feature = "pcx")]
                "pcx" => Ok(ImageFormat::Pcx),
                #[cfg(not(feature = "pcx"))]
                "pcx" => Err(ConvertError::PcxNotEnabled),
                #[cfg(feature = "ultrahdr")]
                "ultrahdr" | "uhdr" => Ok(ImageFormat::UltraHdr),
                #[cfg(not(feature = "ultrahdr"))]
                "ultrahdr" | "uhdr" => Err(ConvertError::UltraHdrNotEnabled),
                #[cfg(feature = "apng")]
                "apng" => Ok(ImageFormat::Apng),
                #[cfg(not(feature = "apng"))]
                "apng" => Err(ConvertError::ApngNotEnabled),
                _ => Err(ConvertError::UnsupportedOutputFormat {
                    format: name,
                    supported: ImageFormat::supported_extensions().join(", "),
                }),
            }
        })
        .collect()
}

impl std::fmt::Display for ImageFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.extension())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 모든 지원 확장자에 대한 from_extension 왕복 테스트
    /// Validates: Requirements 1.2
    #[test]
    fn from_extension_round_trip() {
        let extensions_and_formats: &[(&str, ImageFormat)] = &[
            ("jpg", ImageFormat::Jpeg),
            ("jpeg", ImageFormat::Jpeg),
            ("png", ImageFormat::Png),
            ("gif", ImageFormat::Gif),
            ("bmp", ImageFormat::Bmp),
            ("tif", ImageFormat::Tiff),
            ("tiff", ImageFormat::Tiff),
            ("tga", ImageFormat::Tga),
            ("ico", ImageFormat::Ico),
            ("qoi", ImageFormat::Qoi),
            ("ppm", ImageFormat::Pnm),
            ("pgm", ImageFormat::Pnm),
            ("pbm", ImageFormat::Pnm),
            ("pam", ImageFormat::Pnm),
            ("exr", ImageFormat::OpenExr),
            ("hdr", ImageFormat::Hdr),
            ("ff", ImageFormat::Farbfeld),
            ("webp", ImageFormat::WebP),
            ("svg", ImageFormat::Svg),
        ];

        for (ext, expected_format) in extensions_and_formats {
            let format = ImageFormat::from_extension(ext)
                .unwrap_or_else(|_| panic!("from_extension failed for '{ext}'"));
            assert_eq!(format, *expected_format, "format mismatch for extension '{ext}'");

            // Round-trip: format -> extension -> from_extension should yield the same format
            let canonical_ext = format.extension();
            let round_tripped = ImageFormat::from_extension(canonical_ext)
                .unwrap_or_else(|_| panic!("round-trip failed for '{canonical_ext}'"));
            assert_eq!(round_tripped, format, "round-trip mismatch for '{ext}' -> '{canonical_ext}'");
        }
    }

    /// 점(.) 접두사가 있는 확장자도 처리되는지 확인
    /// Validates: Requirements 1.2
    #[test]
    fn from_extension_with_dot_prefix() {
        assert_eq!(ImageFormat::from_extension(".png").unwrap(), ImageFormat::Png);
        assert_eq!(ImageFormat::from_extension(".jpg").unwrap(), ImageFormat::Jpeg);
    }

    /// 대소문자 무관하게 확장자를 인식하는지 확인
    /// Validates: Requirements 1.2
    #[test]
    fn from_extension_case_insensitive() {
        assert_eq!(ImageFormat::from_extension("PNG").unwrap(), ImageFormat::Png);
        assert_eq!(ImageFormat::from_extension("Jpeg").unwrap(), ImageFormat::Jpeg);
        assert_eq!(ImageFormat::from_extension("WEBP").unwrap(), ImageFormat::WebP);
    }

    /// 지원되지 않는 확장자에 대한 에러 반환 테스트
    /// Validates: Requirements 1.3
    #[test]
    fn unsupported_extension_returns_error() {
        let result = ImageFormat::from_extension("xyz");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, ConvertError::UnsupportedInputFormat { .. }),
            "expected UnsupportedInputFormat, got: {err:?}"
        );
    }

    /// avif feature 비활성화 시 avif 확장자가 AvifNotEnabled 에러를 반환하는지 확인
    /// Validates: Requirements 1.3
    #[cfg(not(feature = "avif"))]
    #[test]
    fn avif_extension_without_feature_returns_error() {
        let result = ImageFormat::from_extension("avif");
        assert!(result.is_err());
        assert!(
            matches!(result.unwrap_err(), ConvertError::AvifNotEnabled),
            "expected AvifNotEnabled error"
        );
    }

    /// apng feature 비활성화 시 apng 확장자가 ApngNotEnabled 에러를 반환하는지 확인
    /// Validates: Requirements 22.4
    #[cfg(not(feature = "apng"))]
    #[test]
    fn apng_extension_without_feature_returns_error() {
        let result = ImageFormat::from_extension("apng");
        assert!(result.is_err());
        assert!(
            matches!(result.unwrap_err(), ConvertError::ApngNotEnabled),
            "expected ApngNotEnabled error"
        );
    }

    /// supports_quality가 Jpeg, WebP에 대해서만 true를 반환하는지 확인
    /// Validates: Requirements 1.4
    #[test]
    fn supports_quality_correctness() {
        // 품질 설정을 지원하는 포맷
        assert!(ImageFormat::Jpeg.supports_quality());
        assert!(ImageFormat::WebP.supports_quality());

        // 품질 설정을 지원하지 않는 포맷
        assert!(!ImageFormat::Png.supports_quality());
        assert!(!ImageFormat::Gif.supports_quality());
        assert!(!ImageFormat::Bmp.supports_quality());
        assert!(!ImageFormat::Tiff.supports_quality());
        assert!(!ImageFormat::Tga.supports_quality());
        assert!(!ImageFormat::Ico.supports_quality());
        assert!(!ImageFormat::Qoi.supports_quality());
        assert!(!ImageFormat::Pnm.supports_quality());
        assert!(!ImageFormat::OpenExr.supports_quality());
        assert!(!ImageFormat::Hdr.supports_quality());
        assert!(!ImageFormat::Farbfeld.supports_quality());
        assert!(!ImageFormat::Svg.supports_quality());
    }

    /// avif feature 활성화 시 Avif가 supports_quality true를 반환하는지 확인
    #[cfg(feature = "avif")]
    #[test]
    fn avif_supports_quality() {
        assert!(ImageFormat::Avif.supports_quality());
    }

    /// 쉼표 구분 문자열 파싱 테스트
    /// Validates: Requirements 1.2, 1.4
    #[test]
    fn parse_formats_comma_separated() {
        let formats = parse_formats("png,webp,jpeg").unwrap();
        assert_eq!(formats, vec![ImageFormat::Png, ImageFormat::WebP, ImageFormat::Jpeg]);
    }

    /// 공백이 포함된 쉼표 구분 문자열 파싱 테스트
    #[test]
    fn parse_formats_with_whitespace() {
        let formats = parse_formats("png , webp , jpeg").unwrap();
        assert_eq!(formats, vec![ImageFormat::Png, ImageFormat::WebP, ImageFormat::Jpeg]);
    }

    /// 단일 포맷 파싱 테스트
    #[test]
    fn parse_formats_single() {
        let formats = parse_formats("png").unwrap();
        assert_eq!(formats, vec![ImageFormat::Png]);
    }

    /// 지원되지 않는 포맷 이름이 포함된 경우 에러 반환 테스트
    /// Validates: Requirements 1.4
    #[test]
    fn parse_formats_unsupported_returns_error() {
        let result = parse_formats("png,xyz,jpeg");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, ConvertError::UnsupportedOutputFormat { .. }),
            "expected UnsupportedOutputFormat, got: {err:?}"
        );
    }

    /// parse_formats에서 다양한 포맷 이름 별칭 테스트
    #[test]
    fn parse_formats_aliases() {
        // jpg와 jpeg 모두 Jpeg으로 파싱
        assert_eq!(parse_formats("jpg").unwrap(), vec![ImageFormat::Jpeg]);
        assert_eq!(parse_formats("jpeg").unwrap(), vec![ImageFormat::Jpeg]);

        // tif와 tiff 모두 Tiff로 파싱
        assert_eq!(parse_formats("tif").unwrap(), vec![ImageFormat::Tiff]);
        assert_eq!(parse_formats("tiff").unwrap(), vec![ImageFormat::Tiff]);

        // pnm 관련 별칭
        assert_eq!(parse_formats("pnm").unwrap(), vec![ImageFormat::Pnm]);
        assert_eq!(parse_formats("ppm").unwrap(), vec![ImageFormat::Pnm]);

        // openexr 별칭
        assert_eq!(parse_formats("openexr").unwrap(), vec![ImageFormat::OpenExr]);
        assert_eq!(parse_formats("exr").unwrap(), vec![ImageFormat::OpenExr]);

        // farbfeld 별칭
        assert_eq!(parse_formats("farbfeld").unwrap(), vec![ImageFormat::Farbfeld]);
        assert_eq!(parse_formats("ff").unwrap(), vec![ImageFormat::Farbfeld]);
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    /// 표준 쓰기 가능 포맷 목록 (feature flag 없이 항상 사용 가능)
    fn writable_format_strategy() -> impl Strategy<Value = ImageFormat> {
        prop_oneof![
            Just(ImageFormat::Jpeg),
            Just(ImageFormat::Png),
            Just(ImageFormat::Gif),
            Just(ImageFormat::Bmp),
            Just(ImageFormat::Tiff),
            Just(ImageFormat::Tga),
            Just(ImageFormat::Ico),
            Just(ImageFormat::Qoi),
            Just(ImageFormat::Pnm),
            Just(ImageFormat::OpenExr),
            Just(ImageFormat::Hdr),
            Just(ImageFormat::Farbfeld),
            Just(ImageFormat::WebP),
            Just(ImageFormat::Svg),
        ]
    }

    // Property 1: 읽기 전용 포맷 인코딩 거부
    // JXL, DDS 포맷으로 인코딩 요청 시 WriteNotSupported 에러 또는 NotEnabled 에러
    // **Validates: Requirements 18.4, 19.4**
    proptest! {
        #[test]
        fn prop1_writable_formats_support_write(format in writable_format_strategy()) {
            // 모든 표준 포맷은 supports_write() == true
            prop_assert!(
                format.supports_write(),
                "Format {:?} should support write but returned false",
                format
            );
        }

        #[test]
        fn prop1_readonly_format_encoding_rejected(
            read_only_name in prop_oneof![Just("jxl"), Just("dds")]
        ) {
            // JXL, DDS feature 비활성화 시 parse_formats로 해당 포맷 요청 시 에러 반환
            let result = parse_formats(read_only_name);
            prop_assert!(
                result.is_err(),
                "parse_formats('{}') should return error for read-only format, got: {:?}",
                read_only_name,
                result
            );

            let err = result.unwrap_err();
            match read_only_name {
                "jxl" => {
                    // feature 비활성화 시 JxlNotEnabled, 활성화 시 WriteNotSupported 가능
                    let is_expected = matches!(
                        err,
                        ConvertError::JxlNotEnabled | ConvertError::WriteNotSupported { .. }
                    );
                    prop_assert!(
                        is_expected,
                        "Expected JxlNotEnabled or WriteNotSupported for 'jxl', got: {:?}",
                        err
                    );
                }
                "dds" => {
                    let is_expected = matches!(
                        err,
                        ConvertError::DdsNotEnabled | ConvertError::WriteNotSupported { .. }
                    );
                    prop_assert!(
                        is_expected,
                        "Expected DdsNotEnabled or WriteNotSupported for 'dds', got: {:?}",
                        err
                    );
                }
                _ => unreachable!(),
            }
        }
    }

    // Feature-gated: JXL이 활성화된 경우 supports_write() == false 확인
    #[cfg(feature = "jxl")]
    proptest! {
        #[test]
        fn prop1_jxl_does_not_support_write(_dummy in 0u8..1) {
            let format = ImageFormat::Jxl;
            prop_assert!(
                !format.supports_write(),
                "JXL format should NOT support write"
            );
        }
    }

    // Feature-gated: DDS가 활성화된 경우 supports_write() == false 확인
    #[cfg(feature = "dds")]
    proptest! {
        #[test]
        fn prop1_dds_does_not_support_write(_dummy in 0u8..1) {
            let format = ImageFormat::Dds;
            prop_assert!(
                !format.supports_write(),
                "DDS format should NOT support write"
            );
        }
    }
}
