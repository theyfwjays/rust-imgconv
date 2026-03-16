# 구현 계획: imgconv

## 개요

Rust로 작성된 고성능 이미지 포맷 일괄 변환 CLI 도구를 구현한다. 프로젝트 구조 설정부터 시작하여 핵심 라이브러리 모듈을 순차적으로 구현하고, 최종적으로 CLI 바이너리와 배치 처리를 통합한다. 각 단계는 이전 단계 위에 점진적으로 빌드된다. 태스크 15번부터는 이미지 처리 기능, 추가 포맷 지원, 메타데이터 관리, 유틸리티 기능을 Phase별로 점진적으로 구현한다.

## Tasks

- [x] 1. 프로젝트 구조 및 에러 타입 설정
  - [x] 1.1 Cargo.toml 생성 및 의존성 설정
    - `imgconv` 크레이트 생성 (lib + bin)
    - 의존성 추가: image v0.25, zenwebp v0.2, resvg, tiny-skia, vtracer, rayon, indicatif, clap (derive), anyhow, thiserror
    - `[features]` 섹션에 `avif = ["ravif", "avif-parse", "rav1d"]` optional feature 정의
    - _요구사항: 15.1, 15.2, 15.3, 15.4, 16.1, 16.2_
  - [x] 1.2 에러 타입 구현 (src/error.rs)
    - `ConvertError` 열거형을 thiserror로 정의
    - 설계 문서의 모든 에러 변형 구현: `UnsupportedInputFormat`, `UnsupportedOutputFormat`, `AvifNotEnabled`, `InvalidQuality`, `SameInputOutput`, `OutputExists`, `NoImagesInDirectory`, `DecodingError`, `EncodingError`, `ResizeError`, `SvgError`, `IoError`
    - _요구사항: 1.3, 1.4, 4.4, 9.3, 10.1, 10.3_
  - [x] 1.3 라이브러리 진입점 설정 (src/lib.rs)
    - 모듈 선언 및 공개 API re-export
    - `format`, `convert`, `raster`, `webp`, `svg`, `resize`, `quality`, `batch`, `error` 모듈 선언
    - `#[cfg(feature = "avif")]` 조건부로 `avif` 모듈 선언
    - _요구사항: 16.1, 16.3_

- [x] 2. 포맷 감지 및 열거형 구현
  - [x] 2.1 ImageFormat 열거형 구현 (src/format.rs)
    - `ImageFormat` 열거형 정의 (Jpeg, Png, Gif, Bmp, Tiff, Tga, Ico, Qoi, Pnm, OpenExr, Hdr, Farbfeld, WebP, Svg, Avif)
    - `#[cfg(feature = "avif")]` 조건부로 Avif 변형 포함
    - `from_extension()`: 확장자 문자열로부터 포맷 감지
    - `extension()`: 포맷에 해당하는 기본 확장자 반환
    - `supports_quality()`: 손실 압축 지원 여부 (JPEG, WebP, AVIF)
    - `supported_extensions()`: 지원 확장자 목록 반환
    - 쉼표 구분 문자열에서 여러 포맷 파싱하는 유틸리티 함수
    - _요구사항: 1.1, 1.2, 1.3, 1.4, 7.1_
  - [x] 2.2 ImageFormat 단위 테스트 작성
    - 모든 확장자에 대한 `from_extension` 왕복 테스트
    - 지원되지 않는 확장자에 대한 에러 반환 테스트
    - `supports_quality` 정확성 테스트
    - 쉼표 구분 파싱 테스트
    - _요구사항: 1.2, 1.3, 1.4_

- [x] 3. 품질 설정 및 리사이즈 모듈 구현
  - [x] 3.1 품질 설정 모듈 구현 (src/quality.rs)
    - 포맷별 기본 품질 값 정의 (JPEG: 85, WebP lossy: 75, AVIF: 70)
    - 품질 값 유효성 검증 (1-100 범위)
    - 무손실 포맷에 품질 지정 시 경고 반환 로직
    - _요구사항: 9.1, 9.2, 9.3, 9.4_
  - [x] 3.2 리사이즈 모듈 구현 (src/resize.rs)
    - `ResizeOptions` 구조체 정의
    - `resize_image` 구현 (4가지 리사이즈 모드)
    - _요구사항: 8.1, 8.2, 8.3, 8.4, 8.5, 8.6_
  - [x] 3.3 리사이즈 단위 테스트 작성
    - _요구사항: 8.1, 8.2, 8.3, 8.4, 8.5, 8.6_

- [x] 4. 래스터 포맷 코덱 구현
  - [x] 4.1 래스터 디코딩/인코딩 구현 (src/raster.rs)
    - _요구사항: 1.1, 1.2, 9.1, 9.2_
  - [x] 4.2 래스터 코덱 단위 테스트 작성
    - _요구사항: 1.1, 1.2_

- [x] 5. Checkpoint - 기본 빌드 확인
  - 모든 테스트 통과 확인, `cargo build` 성공 확인, 사용자에게 질문이 있으면 문의.

- [x] 6. WebP 코덱 구현
  - [x] 6.1 WebP 인코딩/디코딩 구현 (src/webp.rs)
    - _요구사항: 2.1, 2.2, 2.3, 2.4, 2.5_
  - [x] 6.2 WebP 코덱 단위 테스트 작성
    - _요구사항: 2.1, 2.2, 2.3, 2.4, 2.5_

- [x] 7. SVG 변환 모듈 구현
  - [x] 7.1 SVG 래스터화 구현 (src/svg.rs)
    - _요구사항: 3.1, 3.2, 3.3, 3.4_
  - [x] 7.2 SVG 트레이싱 구현 (src/svg.rs)
    - _요구사항: 3.5, 3.6, 3.7_
  - [x] 7.3 SVG 변환 단위 테스트 작성
    - _요구사항: 3.1, 3.2, 3.3, 3.5, 3.6, 3.7_

- [x] 8. AVIF 코덱 구현 (feature-gated)
  - [x] 8.1 AVIF 인코딩/디코딩 구현 (src/avif.rs)
    - _요구사항: 4.1, 4.2, 4.5_
  - [x] 8.2 AVIF 비활성화 시 에러 처리
    - _요구사항: 4.3, 4.4_
  - [x] 8.3 AVIF 코덱 단위 테스트 작성
    - _요구사항: 4.1, 4.2_

- [x] 9. Checkpoint - 모든 코덱 빌드 확인
  - 모든 테스트 통과 확인, `cargo build` 및 `cargo build --features avif` 성공 확인, 사용자에게 질문이 있으면 문의.

- [x] 10. 변환 오케스트레이터 구현
  - [x] 10.1 ConvertOptions 및 결과 타입 정의 (src/convert.rs)
    - _요구사항: 5.1, 5.2, 5.3, 5.4, 12.1_
  - [x] 10.2 단일 파일 변환 로직 구현 (src/convert.rs)
    - _요구사항: 5.1, 5.2, 5.3, 5.4, 7.1, 7.2, 7.3_
  - [x] 10.3 파일 보호 및 덮어쓰기 로직 구현 (src/convert.rs)
    - _요구사항: 10.1, 10.2, 10.3, 9.4_
  - [x] 10.4 Dry-run 모드 구현 (src/convert.rs)
    - _요구사항: 12.1, 12.2_
  - [x] 10.5 변환 오케스트레이터 단위 테스트 작성
    - _요구사항: 5.1, 7.1, 10.1, 10.3, 12.1_

- [x] 11. 배치 처리 모듈 구현
  - [x] 11.1 배치 프로세서 구현 (src/batch.rs)
    - _요구사항: 6.1, 6.2, 6.3, 6.4, 6.5_
  - [x] 11.2 배치 프로세서 단위 테스트 작성
    - _요구사항: 6.1, 6.3, 6.4, 6.5_

- [x] 12. Checkpoint - 라이브러리 크레이트 완성 확인
  - 모든 테스트 통과 확인, 라이브러리 API가 CLI 의존성(clap, indicatif) 없이 독립적으로 사용 가능한지 확인, 사용자에게 질문이 있으면 문의.

- [x] 13. CLI 바이너리 구현
  - [x] 13.1 CLI 인자 파싱 구현 (src/main.rs)
    - _요구사항: 17.1, 17.2, 17.3, 17.4_
  - [x] 13.2 진행률 표시 구현 (src/main.rs)
    - _요구사항: 11.1, 11.2, 11.3_
  - [x] 13.3 메인 실행 흐름 및 종료 코드 구현 (src/main.rs)
    - _요구사항: 5.1, 6.4, 7.3, 11.3, 12.1, 13.1, 13.2, 14.1, 14.2, 14.3_

- [x] 14. Checkpoint - 전체 통합 확인
  - 모든 테스트 통과 확인, `cargo build --release` 성공 확인, 사용자에게 질문이 있으면 문의.

### Phase 1 — 기존 의존성만으로 즉시 구현 가능

- [x] 15. Phase 1 기반 작업: 에러 타입 확장 및 모듈 선언
  - [x] 15.1 에러 타입 확장 (src/error.rs)
    - `ConvertError`에 새 에러 변형 추가: `WriteNotSupported`, `CropOutOfBounds`, `InvalidCropFormat`, `InvalidRotateAngle`, `InvalidFlipDirection`, `BlurSharpenConflict`, `InvalidBlurSigma`, `InvalidSharpenValue`, `InvalidGamma`, `WatermarkError`, `OverlayFileNotFound`, `OverlayUnsupportedFormat`, `ExifError`, `CompareSizeMismatch`, `CompareError`, `InvalidPreset`, `JxlNotEnabled`, `DdsNotEnabled`, `PcxNotEnabled`, `UltraHdrNotEnabled`, `ApngNotEnabled`
    - 설계 문서의 에러 메시지 형식 준수
    - _요구사항: 23.3, 23.4, 24.6, 24.7, 26.3, 26.4, 26.5, 30.4, 18.3, 19.3, 20.4, 21.3, 22.4, 27.1, 28.6, 28.7, 33.3, 35.6_
  - [x] 15.2 lib.rs 모듈 선언 업데이트 (src/lib.rs)
    - 새 모듈 선언 추가: `crop`, `transform`, `filter`, `watermark`, `overlay`, `exif`, `info`, `compare`, `dedup`, `preset`
    - Feature-gated 모듈 선언: `#[cfg(feature = "jxl")] pub mod jxl`, `#[cfg(feature = "dds")] pub mod dds`, `#[cfg(feature = "pcx")] pub mod pcx`, `#[cfg(feature = "ultrahdr")] pub mod ultrahdr`, `#[cfg(feature = "apng")] pub mod apng`
    - 공개 API re-export 업데이트
    - _요구사항: 16.1, 16.3_
  - [x] 15.3 ImageFormat 열거형 확장 (src/format.rs)
    - Feature-gated 변형 추가: `Jxl`, `Dds`, `Pcx`, `UltraHdr`, `Apng`
    - `from_extension()` 확장: .jxl, .dds, .pcx, .uhdr.jpg, .apng 지원
    - `supports_write()` 메서드 추가: JXL, DDS는 `false` 반환
    - `supports_quality()` 확장: UltraHdr 추가
    - _요구사항: 18.1, 19.1, 20.1, 21.1, 22.1_
  - [x] 15.4 ConvertOptions 구조체 확장 (src/convert.rs)
    - 새 필드 추가: `crop`, `rotate`, `flip`, `color_filter`, `brightness_contrast`, `blur`, `sharpen`, `watermark`, `overlay`, `auto_orient`, `preserve_exif`, `skip_identical`
    - `ColorFilterOptions`, `BrightnessContrastOptions` 구조체 정의
    - _요구사항: 23.1, 24.1, 25.1, 26.1, 27.1, 28.1, 29.1, 30.1, 31.1, 34.1_

- [x] 16. 이미지 크롭 모듈 구현
  - [x] 16.1 크롭 모듈 구현 (src/crop.rs)
    - `CropOptions` 구조체 정의 (x, y, width, height)
    - `CropOptions::from_str()`: "x,y,w,h" 형식 문자열 파싱
    - `CropOptions::validate()`: 이미지 크기에 대한 크롭 영역 유효성 검증
    - `apply_crop()`: `image::DynamicImage::crop_imm()` 사용하여 크롭 적용
    - _요구사항: 23.1, 23.2, 23.3, 23.4_
  - [x] 16.2 크롭 속성 기반 테스트 작성
    - **Property 3: 크롭 결과 크기** — 유효한 크롭 영역의 결과 이미지 크기가 w, h와 정확히 일치
    - **Property 5: 크롭 범위 초과 시 에러** — 이미지 범위 초과 크롭 시 `CropOutOfBounds` 에러
    - **Property 6: 크롭 형식 파싱** — 잘못된 형식 문자열 시 `InvalidCropFormat` 에러
    - **Validates: 요구사항 23.1, 23.3, 23.4**

- [x] 17. 이미지 회전/뒤집기 모듈 구현
  - [x] 17.1 회전/뒤집기 모듈 구현 (src/transform.rs)
    - `RotateAngle` 열거형 정의 (Rotate90, Rotate180, Rotate270)
    - `FlipDirection` 열거형 정의 (Horizontal, Vertical)
    - `apply_rotate()`: `DynamicImage::rotate90()`, `rotate180()`, `rotate270()` 사용
    - `apply_flip()`: `DynamicImage::fliph()`, `flipv()` 사용
    - _요구사항: 24.1, 24.2, 24.3, 24.4, 24.5_
  - [x] 17.2 회전/뒤집기 속성 기반 테스트 작성
    - **Property 7: 회전 후 이미지 크기** — 90/270도 회전 시 너비/높이 교환, 180도 시 보존
    - **Property 8: 뒤집기 라운드트립** — 같은 방향 두 번 뒤집기 시 원본과 동일
    - **Property 9: 유효하지 않은 회전/뒤집기 값 거부** — 잘못된 값 시 에러 반환
    - **Validates: 요구사항 24.1, 24.2, 24.3, 24.4, 24.5, 24.6, 24.7**

- [x] 18. 색상 필터 모듈 구현
  - [x] 18.1 색상 필터 구현 (src/filter.rs)
    - `ColorFilterOptions` 구조체 정의 (grayscale, invert, sepia)
    - `apply_color_filters()`: grayscale → sepia → invert 순서로 적용
    - 그레이스케일: `DynamicImage::grayscale()` 사용
    - 색상 반전: `DynamicImage::invert()` 사용 (in-place)
    - 세피아: 픽셀별 세피아 톤 변환 공식 적용
    - _요구사항: 25.1, 25.2, 25.3, 25.4_
  - [x] 18.2 색상 필터 속성 기반 테스트 작성
    - **Property 10: 그레이스케일 픽셀 균일성** — 그레이스케일 후 모든 픽셀의 R=G=B
    - **Property 11: 색상 반전 라운드트립** — 두 번 반전 시 원본과 동일
    - **Validates: 요구사항 25.1, 25.2**

- [x] 19. 블러/샤프닝 구현
  - [x] 19.1 블러/샤프닝 함수 구현 (src/filter.rs)
    - `apply_blur()`: `image::imageops::blur()` 사용 (가우시안 블러)
    - `apply_sharpen()`: `image::imageops::unsharpen()` 사용 (언샤프 마스크)
    - sigma/value 값 유효성 검증 (양수만 허용)
    - _요구사항: 26.1, 26.2, 26.3, 26.4, 26.5_
  - [x] 19.2 블러/샤프닝 속성 기반 테스트 작성
    - **Property 12: 블러/샤프닝 크기 보존** — 적용 후 이미지 크기 동일
    - **Property 13: 유효하지 않은 블러/샤프닝 값 거부** — 0 이하 값 시 에러
    - **Validates: 요구사항 26.1, 26.2, 26.3, 26.4**

- [x] 20. 밝기/대비/감마 조정 구현
  - [x] 20.1 밝기/대비/감마 함수 구현 (src/filter.rs)
    - `BrightnessContrastOptions` 구조체 정의 (brightness, contrast, gamma)
    - `apply_brightness_contrast()`: brightness → contrast → gamma 순서로 적용
    - 밝기: `image::imageops::brighten()` 사용
    - 대비: `image::imageops::contrast()` 사용
    - 감마: 픽셀별 감마 보정 공식 적용 (pixel = pixel^(1/gamma))
    - 감마 값 유효성 검증 (양수만 허용)
    - _요구사항: 30.1, 30.2, 30.3, 30.4, 30.5_
  - [x] 20.2 밝기/대비/감마 속성 기반 테스트 작성
    - **Property 18: 밝기 0 항등성** — 밝기 0 조정 시 원본과 동일
    - **Property 19: 감마 1.0 항등성** — 감마 1.0 보정 시 원본과 동일
    - **Property 20: 유효하지 않은 감마 값 거부** — 0 이하 감마 시 `InvalidGamma` 에러
    - **Validates: 요구사항 30.1, 30.3, 30.4**

- [x] 21. 이미지 정보 출력 모듈 구현
  - [x] 21.1 이미지 정보 모듈 구현 (src/info.rs)
    - `ImageInfo` 구조체 정의 (width, height, format, color_type, bit_depth, file_size, exif_summary)
    - `ExifSummary` 구조체 정의 (camera_model, date_taken, iso, shutter_speed, aperture)
    - `get_image_info()`: 이미지 파일 메타데이터 읽기
    - `get_directory_info()`: 디렉토리 내 모든 이미지 정보 수집
    - _요구사항: 32.1, 32.2, 32.3, 32.4_
  - [x] 21.2 이미지 정보 속성 기반 테스트 작성
    - **Property 22: 이미지 정보 출력 시 유효한 메타데이터** — 유효한 이미지에서 0보다 큰 너비/높이 반환
    - **Validates: 요구사항 32.1**

- [x] 22. Phase 1 파이프라인 통합
  - [x] 22.1 변환 파이프라인 업데이트 (src/convert.rs)
    - `convert_file()` 함수에 이미지 처리 파이프라인 단계 추가
    - 파이프라인 순서: 디코딩 → 크롭 → 회전/뒤집기 → 리사이즈 → 색상 필터 → 밝기/대비/감마 → 블러/샤프닝 → 인코딩
    - 각 단계는 `ConvertOptions`의 해당 필드가 `Some`/`true`일 때만 실행
    - _요구사항: 23.2, 24.1, 25.4, 26.1, 30.5_
  - [x] 22.2 크롭 후 리사이즈 파이프라인 순서 속성 테스트
    - **Property 4: 크롭 후 리사이즈 파이프라인 순서** — 크롭+리사이즈 동시 지정 시 최종 크기가 리사이즈 대상 크기와 일치
    - **Validates: 요구사항 23.2**
  - [x] 22.3 CLI 인자 업데이트 (src/main.rs)
    - 새 CLI 인자 추가: `--crop`, `--rotate`, `--flip`, `--grayscale`, `--invert`, `--sepia`, `--blur`, `--sharpen`, `--brightness`, `--contrast`, `--gamma`, `--info`
    - `--blur`와 `--sharpen` 상호 배타 설정 (`conflicts_with`)
    - `--brightness`에 `allow_hyphen_values = true` 설정
    - CLI 인자 → `ConvertOptions` 변환 로직 업데이트
    - `--info` 모드: 변환 없이 이미지 정보 출력 흐름 구현
    - _요구사항: 17.2, 23.1, 24.1, 25.1, 26.1, 30.1, 32.1_

- [x] 23. Checkpoint - Phase 1 완료 확인
  - 모든 테스트 통과 확인, `cargo build` 성공 확인, 크롭/회전/뒤집기/색상 필터/블러/샤프닝/밝기/대비/감마/이미지 정보 기능이 정상 동작하는지 확인, 사용자에게 질문이 있으면 문의.

### Phase 2 — 소규모 의존성 추가

- [x] 24. Phase 2 의존성 추가
  - [x] 24.1 Cargo.toml 의존성 업데이트
    - `kamadak-exif` 의존성 추가 (EXIF 읽기)
    - `[features]` 섹션에 `dds = ["image/dds"]` feature 추가
    - _요구사항: 19.1, 29.5_

- [x] 25. EXIF 방향 보정 모듈 구현
  - [x] 25.1 EXIF 방향 보정 구현 (src/exif.rs)
    - `ExifOrientation` 열거형 정의 (Normal, FlipHorizontal, Rotate180, FlipVertical, Transpose, Rotate90, Transverse, Rotate270)
    - `auto_orient()`: kamadak-exif로 EXIF Orientation 태그 읽기 → 이미지 회전/뒤집기 적용
    - EXIF 태그 없는 경우 원본 이미지 그대로 반환
    - _요구사항: 29.1, 29.2, 29.3, 29.4_
  - [x] 25.2 EXIF 방향 보정 속성 기반 테스트 작성
    - **Property 17: EXIF 없는 이미지의 auto-orient 무변경** — EXIF Orientation 태그 없는 이미지에 auto-orient 적용 시 원본과 동일
    - **Validates: 요구사항 29.4**

- [x] 26. DDS 읽기 모듈 구현 (feature-gated)
  - [x] 26.1 DDS 디코더 구현 (src/dds.rs)
    - 전체 모듈을 `#[cfg(feature = "dds")]`로 게이트
    - `decode_dds()`: image 크레이트의 DDS 디코더로 DDS 파일 디코딩
    - 읽기 전용 — 인코딩 함수 없음
    - _요구사항: 19.1, 19.2, 19.4_
  - [x] 26.2 DDS 비활성화 시 에러 처리
    - feature flag 비활성화 상태에서 DDS 변환 요청 시 `ConvertError::DdsNotEnabled` 반환
    - _요구사항: 19.3_

- [x] 27. Phase 2 파이프라인 통합
  - [x] 27.1 변환 파이프라인에 EXIF 방향 보정 추가 (src/convert.rs)
    - 디코딩 직후, 크롭 이전에 `auto_orient()` 호출 (`--auto-orient` 지정 시)
    - 포맷별 디코더 라우팅에 DDS 추가
    - _요구사항: 29.1, 19.2_
  - [x] 27.2 읽기 전용 포맷 인코딩 거부 속성 테스트
    - **Property 1: 읽기 전용 포맷 인코딩 거부** — JXL, DDS 포맷으로 인코딩 요청 시 `WriteNotSupported` 에러
    - **Validates: 요구사항 18.4, 19.4**
  - [x] 27.3 CLI 인자 업데이트 (src/main.rs)
    - 새 CLI 인자 추가: `--auto-orient`, `--no-auto-orient` (상호 배타)
    - CLI 인자 → `ConvertOptions` 변환 로직 업데이트
    - _요구사항: 17.2, 29.1, 29.2_

- [x] 28. Checkpoint - Phase 2 완료 확인
  - 모든 테스트 통과 확인, `cargo build` 및 `cargo build --features dds` 성공 확인, EXIF 방향 보정 및 DDS 읽기 기능 정상 동작 확인, 사용자에게 질문이 있으면 문의.

### Phase 3 — 중규모 기능

- [x] 29. Phase 3 의존성 추가
  - [x] 29.1 Cargo.toml 의존성 업데이트
    - `imageproc` 의존성 추가 (텍스트 워터마크)
    - `ab_glyph` 의존성 추가 (폰트 렌더링)
    - `img-parts` 의존성 추가 (EXIF 쓰기)
    - `jxl-oxide` optional 의존성 추가
    - `[features]` 섹션에 `jxl = ["jxl-oxide"]` feature 추가
    - _요구사항: 18.5, 27.8, 31.5_

- [x] 30. 텍스트 워터마크 모듈 구현
  - [x] 30.1 텍스트 워터마크 구현 (src/watermark.rs)
    - `Position` 열거형 정의 (TopLeft, TopRight, BottomLeft, BottomRight, Center)
    - `WatermarkOptions` 구조체 정의 (text, position, opacity, font_path)
    - `apply_watermark()`: imageproc + ab_glyph로 텍스트 렌더링
    - 위치별 좌표 계산 로직
    - 투명도 적용 (알파 블렌딩)
    - 내장 기본 폰트 또는 사용자 지정 폰트 로딩
    - _요구사항: 27.1, 27.2, 27.3, 27.4, 27.5, 27.6, 27.7_
  - [x] 30.2 워터마크 속성 기반 테스트 작성
    - **Property 14: 워터마크 적용 시 크기 보존 및 픽셀 변경** — 워터마크 후 크기 동일, 픽셀 변경됨
    - **Validates: 요구사항 27.1**

- [x] 31. 이미지 오버레이 모듈 구현
  - [x] 31.1 이미지 오버레이 구현 (src/overlay.rs)
    - `OverlayOptions` 구조체 정의 (image_path, position, opacity)
    - `apply_overlay()`: `image::imageops::overlay()` 사용
    - 오버레이 이미지 로딩 및 디코딩
    - 위치별 좌표 계산 (watermark.rs의 `Position` 재사용)
    - 투명도 적용
    - 파일 존재 여부 및 포맷 유효성 검증
    - _요구사항: 28.1, 28.2, 28.3, 28.4, 28.5, 28.6, 28.7_
  - [x] 31.2 오버레이 속성 기반 테스트 작성
    - **Property 15: 오버레이 적용 시 크기 보존** — 오버레이 후 베이스 이미지 크기 동일
    - **Property 16: 존재하지 않는 오버레이 파일 에러** — 없는 파일 경로 시 `OverlayFileNotFound` 에러
    - **Validates: 요구사항 28.1, 28.6**

- [x] 32. EXIF 보존 모듈 구현
  - [x] 32.1 EXIF 보존 구현 (src/exif.rs)
    - `preserve_exif()`: kamadak-exif로 원본 EXIF 읽기 → img-parts로 변환 결과에 EXIF 기록
    - JPEG→JPEG 변환 시 EXIF 데이터 복사
    - 크로스 포맷 변환 시 경고 메시지 출력
    - EXIF 쓰기 실패 시 경고 후 EXIF 없이 저장
    - _요구사항: 31.1, 31.2, 31.3, 31.4_
  - [x] 32.2 EXIF 보존 속성 기반 테스트 작성
    - **Property 21: EXIF 보존 라운드트립** — EXIF 있는 JPEG의 JPEG→JPEG 변환 시 출력에 EXIF 존재
    - **Validates: 요구사항 31.1**

- [x] 33. JPEG XL 디코딩 모듈 구현 (feature-gated)
  - [x] 33.1 JXL 디코더 구현 (src/jxl.rs)
    - 전체 모듈을 `#[cfg(feature = "jxl")]`로 게이트
    - `decode_jxl()`: jxl-oxide로 JPEG XL 파일 디코딩 → `DynamicImage` 반환
    - 읽기 전용 — 인코딩 함수 없음
    - _요구사항: 18.1, 18.2, 18.4, 18.5_
  - [x] 33.2 JXL 비활성화 시 에러 처리
    - feature flag 비활성화 상태에서 JXL 변환 요청 시 `ConvertError::JxlNotEnabled` 반환
    - _요구사항: 18.3_

- [x] 34. 변환 프리셋 모듈 구현
  - [x] 34.1 프리셋 모듈 구현 (src/preset.rs)
    - `Preset` 열거형 정의 (Web, Thumbnail, Print, Social)
    - `PresetConfig` 구조체 정의 (format, quality, width, height, keep_aspect, webp_mode, dpi)
    - `get_preset_config()`: 프리셋별 설정 반환 (web: WebP lossy q80 w1920, thumbnail: JPEG q70 200x200, print: TIFF 300dpi, social: JPEG q85 1200x630)
    - `apply_preset()`: 프리셋 설정을 ConvertOptions에 적용, 개별 옵션이 프리셋을 덮어씀
    - _요구사항: 35.1, 35.2, 35.3, 35.4, 35.5, 35.6_
  - [x] 34.2 프리셋 속성 기반 테스트 작성
    - **Property 27: 프리셋 개별 옵션 덮어쓰기** — 프리셋+개별 옵션 시 개별 옵션 값이 최종 설정에 반영
    - **Property 28: 유효하지 않은 프리셋 이름 거부** — 잘못된 프리셋 이름 시 `InvalidPreset` 에러
    - **Validates: 요구사항 35.5, 35.6**

- [x] 35. Phase 3 파이프라인 통합
  - [x] 35.1 변환 파이프라인에 워터마크/오버레이/EXIF 보존 추가 (src/convert.rs)
    - 블러/샤프닝 이후, 인코딩 이전에 워터마크/오버레이 적용
    - 인코딩 이후 EXIF 보존 적용 (`--preserve-exif` 지정 시)
    - 포맷별 디코더 라우팅에 JXL 추가
    - 프리셋 적용 로직 추가 (변환 시작 전 프리셋 → 개별 옵션 덮어쓰기)
    - _요구사항: 27.1, 28.1, 31.1, 18.2, 35.5_
  - [x] 35.2 CLI 인자 업데이트 (src/main.rs)
    - 새 CLI 인자 추가: `--watermark`, `--watermark-position`, `--watermark-opacity`, `--watermark-font`, `--overlay`, `--overlay-position`, `--overlay-opacity`, `--preserve-exif`, `--preset`
    - 프리셋 적용 후 개별 옵션 덮어쓰기 로직
    - _요구사항: 17.2, 27.1, 28.1, 31.1, 35.1_

- [x] 36. Checkpoint - Phase 3 완료 확인
  - 모든 테스트 통과 확인, `cargo build` 및 `cargo build --features jxl` 성공 확인, 워터마크/오버레이/EXIF 보존/JXL 디코딩/프리셋 기능 정상 동작 확인, 사용자에게 질문이 있으면 문의.

### Phase 4 — 실험적/장기

- [x] 37. Phase 4 의존성 추가
  - [x] 37.1 Cargo.toml 의존성 업데이트
    - `image-compare` optional 의존성 추가
    - `sha2` optional 의존성 추가
    - `pcx` optional 의존성 추가
    - `ultrahdr-core` optional 의존성 추가
    - `apng` optional 의존성 추가
    - `[features]` 섹션에 추가: `compare = ["image-compare"]`, `dedup = ["sha2"]`, `pcx = ["dep:pcx"]`, `ultrahdr = ["ultrahdr-core"]`, `apng = ["dep:apng"]`
    - _요구사항: 20.5, 21.4, 22.5, 33.4, 34.4_

- [x] 38. 이미지 품질 비교 모듈 구현 (feature-gated)
  - [x] 38.1 이미지 비교 구현 (src/compare.rs)
    - `CompareResult` 구조체 정의 (ssim, psnr)
    - `compare_images()`: image-compare 크레이트로 SSIM/PSNR 계산
    - 이미지 크기 불일치 시 `CompareSizeMismatch` 에러
    - _요구사항: 33.1, 33.2, 33.3_
  - [x] 38.2 이미지 비교 속성 기반 테스트 작성
    - **Property 24: 동일 이미지 비교 시 SSIM 1.0** — 자기 자신과 비교 시 SSIM = 1.0
    - **Property 25: 크기가 다른 이미지 비교 시 에러** — 크기 다른 이미지 비교 시 `CompareSizeMismatch` 에러
    - **Property 23: 정보/비교 모드 파일 시스템 무변경** — 비교 모드 실행 시 파일 시스템 변경 없음
    - **Validates: 요구사항 33.1, 33.2, 33.3**

- [x] 39. 중복 파일 건너뛰기 모듈 구현 (feature-gated)
  - [x] 39.1 중복 검사 구현 (src/dedup.rs)
    - `file_hash()`: sha2 크레이트로 SHA-256 해시 계산
    - `is_identical()`: 두 파일의 해시 비교
    - _요구사항: 34.1, 34.2, 34.3_
  - [x] 39.2 중복 검사 속성 기반 테스트 작성
    - **Property 26: SHA-256 해시 일관성 및 중복 감지** — 같은 파일 해시 두 번 계산 시 동일, 동일 파일 쌍 `is_identical` = true
    - **Validates: 요구사항 34.1, 34.2**

- [x] 40. PCX 코덱 구현 (feature-gated)
  - [x] 40.1 PCX 코덱 구현 (src/pcx.rs)
    - 전체 모듈을 `#[cfg(feature = "pcx")]`로 게이트
    - `decode_pcx()`: pcx 크레이트로 PCX 파일 디코딩
    - `encode_pcx()`: pcx 크레이트로 PCX 파일 인코딩
    - _요구사항: 20.1, 20.2, 20.3_
  - [x] 40.2 PCX 비활성화 시 에러 처리
    - feature flag 비활성화 상태에서 PCX 변환 요청 시 `ConvertError::PcxNotEnabled` 반환
    - _요구사항: 20.4_
  - [x] 40.3 PCX 속성 기반 테스트 작성
    - **Property 2: PCX 라운드트립** — PCX 인코딩 후 디코딩 시 원본과 동일한 너비/높이
    - **Validates: 요구사항 20.1**

- [x] 41. Ultra HDR JPEG 코덱 구현 (feature-gated)
  - [x] 41.1 Ultra HDR 코덱 구현 (src/ultrahdr.rs)
    - 전체 모듈을 `#[cfg(feature = "ultrahdr")]`로 게이트
    - `decode_ultrahdr()`: ultrahdr-core로 Ultra HDR JPEG 디코딩
    - `encode_ultrahdr()`: ultrahdr-core로 Ultra HDR JPEG 인코딩
    - _요구사항: 21.1, 21.2_
  - [x] 41.2 Ultra HDR 비활성화 시 에러 처리
    - feature flag 비활성화 상태에서 Ultra HDR 변환 요청 시 `ConvertError::UltraHdrNotEnabled` 반환
    - _요구사항: 21.3_

- [x] 42. APNG 코덱 구현 (feature-gated)
  - [x] 42.1 APNG 코덱 구현 (src/apng.rs)
    - 전체 모듈을 `#[cfg(feature = "apng")]`로 게이트
    - `decode_apng()`: image 크레이트 PNG 디코더로 첫 번째 프레임 디코딩
    - `encode_apng()`: apng 크레이트로 단일 프레임 APNG 인코딩
    - _요구사항: 22.1, 22.2, 22.3_
  - [x] 42.2 APNG 비활성화 시 에러 처리
    - feature flag 비활성화 상태에서 APNG 변환 요청 시 `ConvertError::ApngNotEnabled` 반환
    - _요구사항: 22.4_

- [x] 43. Phase 4 파이프라인 통합
  - [x] 43.1 변환 파이프라인에 Phase 4 기능 추가 (src/convert.rs)
    - 포맷별 디코더 라우팅에 PCX, Ultra HDR, APNG 추가
    - 포맷별 인코더 라우팅에 PCX, Ultra HDR, APNG 추가
    - 중복 검사 로직 추가 (`--skip-identical` 지정 시, 변환 전 해시 비교)
    - `BatchResult`에 `skipped` 필드 활용
    - _요구사항: 20.2, 20.3, 21.2, 22.3, 34.1, 34.2, 34.3_
  - [x] 43.2 CLI 인자 업데이트 (src/main.rs)
    - 새 CLI 인자 추가: `--compare` (num_args = 2), `--skip-identical`
    - `--compare` 모드: 변환 없이 두 이미지 SSIM/PSNR 출력 흐름 구현
    - `--skip-identical` 옵션 → `ConvertOptions` 변환
    - _요구사항: 17.2, 33.1, 34.1_

- [x] 44. Checkpoint - Phase 4 완료 확인
  - 모든 테스트 통과 확인, 모든 feature flag 조합 빌드 성공 확인 (`cargo build --features "dds,jxl,pcx,ultrahdr,apng,compare,dedup"`), 사용자에게 질문이 있으면 문의.

- [x] 45. 최종 통합 및 검증
  - [x] 45.1 전체 파이프라인 통합 테스트
    - 모든 이미지 처리 단계가 올바른 순서로 적용되는지 검증
    - 다중 옵션 조합 테스트 (크롭 + 회전 + 리사이즈 + 필터 + 워터마크)
    - 배치 모드에서 새 기능들이 정상 동작하는지 검증
    - _요구사항: 23.2, 24.1, 25.4, 26.1, 27.1, 28.1, 30.5_
  - [x] 45.2 CLI 전체 인자 검증
    - 모든 새 CLI 인자가 올바르게 파싱되는지 확인
    - 잘못된 인자 조합 시 적절한 에러 메시지 출력 확인
    - `--help` 출력에 모든 새 옵션이 포함되는지 확인
    - _요구사항: 17.2, 17.3, 17.4_

- [x] 46. Final Checkpoint - 전체 기능 완료 확인
  - 모든 테스트 통과 확인, `cargo build --release` 성공 확인, 모든 feature flag 조합 빌드 확인, 사용자에게 질문이 있으면 문의.

## 참고

- `*` 표시된 태스크는 선택적이며 빠른 MVP를 위해 건너뛸 수 있음
- 각 태스크는 추적 가능성을 위해 구체적인 요구사항을 참조함
- Checkpoint에서 점진적 검증 수행
- AVIF 관련 코드는 `#[cfg(feature = "avif")]`로 조건부 컴파일
- Phase 2~4의 feature-gated 모듈은 각각 `#[cfg(feature = "...")]`로 조건부 컴파일
- 속성 기반 테스트는 `proptest` 크레이트를 사용하며, 각 속성에 설계 문서의 Property 번호를 태그로 기록
- 이미지 처리 파이프라인 순서: EXIF 방향 보정 → 크롭 → 회전/뒤집기 → 리사이즈 → 색상 필터 → 밝기/대비/감마 → 블러/샤프닝 → 워터마크/오버레이 → 인코딩 → EXIF 보존
