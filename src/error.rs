use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConvertError {
    #[error("지원되지 않는 입력 포맷: {extension}. 지원 포맷: {supported}")]
    UnsupportedInputFormat { extension: String, supported: String },

    #[error("지원되지 않는 출력 포맷: {format}. 지원 포맷: {supported}")]
    UnsupportedOutputFormat { format: String, supported: String },

    #[error("쓰기를 지원하지 않는 포맷: {format}. 이 포맷은 읽기 전용입니다")]
    WriteNotSupported { format: String },

    #[error("AVIF 기능이 비활성화됨. `cargo build --features avif`로 빌드하세요")]
    AvifNotEnabled,

    #[error("JXL 기능이 비활성화됨. `cargo build --features jxl`로 빌드하세요")]
    JxlNotEnabled,

    #[error("DDS 기능이 비활성화됨. `cargo build --features dds`로 빌드하세요")]
    DdsNotEnabled,

    #[error("PCX 기능이 비활성화됨. `cargo build --features pcx`로 빌드하세요")]
    PcxNotEnabled,

    #[error("Ultra HDR 기능이 비활성화됨. `cargo build --features ultrahdr`로 빌드하세요")]
    UltraHdrNotEnabled,

    #[error("APNG 기능이 비활성화됨. `cargo build --features apng`로 빌드하세요")]
    ApngNotEnabled,

    #[error("품질 값이 유효 범위를 벗어남: {value}. 유효 범위: 1-100")]
    InvalidQuality { value: u8 },

    #[error("입력 파일과 출력 파일 경로가 동일: {path}")]
    SameInputOutput { path: String },

    #[error("출력 파일이 이미 존재: {path}. --overwrite 옵션을 사용하세요")]
    OutputExists { path: String },

    #[error("디렉토리에 변환 가능한 이미지 파일 없음: {path}")]
    NoImagesInDirectory { path: String },

    #[error("이미지 디코딩 실패: {0}")]
    DecodingError(String),

    #[error("이미지 인코딩 실패: {0}")]
    EncodingError(String),

    #[error("리사이즈 실패: {0}")]
    ResizeError(String),

    #[error("SVG 처리 실패: {0}")]
    SvgError(String),

    #[error("크롭 영역이 이미지 범위를 초과: x={x}, y={y}, w={w}, h={h}, 이미지 크기: {img_w}x{img_h}")]
    CropOutOfBounds { x: u32, y: u32, w: u32, h: u32, img_w: u32, img_h: u32 },

    #[error("크롭 옵션 형식 오류: '{input}'. 올바른 형식: x,y,w,h")]
    InvalidCropFormat { input: String },

    #[error("유효하지 않은 회전 각도: {angle}. 유효 값: 90, 180, 270")]
    InvalidRotateAngle { angle: u32 },

    #[error("유효하지 않은 뒤집기 방향: {direction}. 유효 값: horizontal, vertical")]
    InvalidFlipDirection { direction: String },

    #[error("블러와 샤프닝을 동시에 사용할 수 없습니다")]
    BlurSharpenConflict,

    #[error("블러 sigma 값은 양수여야 합니다: {value}")]
    InvalidBlurSigma { value: f32 },

    #[error("샤프닝 값은 양수여야 합니다: {value}")]
    InvalidSharpenValue { value: f32 },

    #[error("감마 값은 양수여야 합니다: {value}")]
    InvalidGamma { value: f32 },

    #[error("워터마크 처리 실패: {0}")]
    WatermarkError(String),

    #[error("오버레이 이미지 파일을 찾을 수 없음: {path}")]
    OverlayFileNotFound { path: String },

    #[error("오버레이 이미지 포맷 미지원: {path}")]
    OverlayUnsupportedFormat { path: String },

    #[error("EXIF 처리 실패: {0}")]
    ExifError(String),

    #[error("이미지 비교 실패: 두 이미지의 크기가 다릅니다 ({w1}x{h1} vs {w2}x{h2})")]
    CompareSizeMismatch { w1: u32, h1: u32, w2: u32, h2: u32 },

    #[error("이미지 비교 실패: {0}")]
    CompareError(String),

    #[error("지원되지 않는 프리셋: {name}. 지원 프리셋: web, thumbnail, print, social")]
    InvalidPreset { name: String },

    #[error("파일 I/O 오류: {0}")]
    IoError(#[from] std::io::Error),
}
