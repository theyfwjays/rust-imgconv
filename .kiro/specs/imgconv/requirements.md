# 요구사항 문서

## 소개

imgconv는 Rust로 작성된 고성능 이미지 포맷 일괄 변환 CLI 도구이다. 래스터 이미지, WebP, SVG, AVIF, JPEG XL, DDS, PCX, Ultra HDR, APNG 등 주요 이미지 포맷 간 상호 변환을 지원하며, pure Rust만 사용하여 C 컴파일러 없이 빌드 가능한 싱글 바이너리로 배포된다. 단일 파일 변환, 디렉토리 일괄 변환, 다중 출력 포맷 동시 변환, 병렬 처리, 리사이즈, 품질 설정 등 기본 변환 기능과 함께 크롭, 회전/뒤집기, 색상 필터, 블러/샤프닝, 워터마크, 밝기/대비 조정 등 이미지 처리 기능, EXIF 메타데이터 관리, 이미지 품질 비교, 용도별 변환 프리셋 등 현업에서 필요한 종합적인 이미지 변환 및 처리 기능을 제공한다.

## 용어집

- **CLI**: imgconv 커맨드라인 인터페이스 바이너리
- **Converter**: 이미지 포맷 변환을 수행하는 핵심 라이브러리 모듈
- **Rasterizer**: SVG를 래스터 이미지(픽셀)로 변환하는 모듈 (resvg + tiny-skia 기반)
- **Tracer**: 래스터 이미지를 SVG 벡터로 변환하는 모듈 (vtracer 기반)
- **Batch_Processor**: 디렉토리 내 여러 파일을 병렬로 일괄 변환하는 모듈 (rayon 기반)
- **Progress_Reporter**: 변환 진행률을 터미널에 표시하는 모듈 (indicatif 기반)
- **Resizer**: 이미지 크기를 조정하는 모듈
- **Quality_Controller**: 손실 압축 포맷의 품질 파라미터를 관리하는 모듈
- **래스터_포맷**: JPEG, PNG, GIF, BMP, TIFF, TGA, ICO, QOI, PNM, OpenEXR, Radiance HDR, Farbfeld 등 픽셀 기반 이미지 포맷
- **손실_압축_포맷**: JPEG, WebP(lossy), AVIF 등 품질 설정이 가능한 포맷
- **무손실_포맷**: PNG, GIF, BMP, TIFF, TGA, ICO, QOI, PNM, OpenEXR, HDR, Farbfeld, WebP(lossless) 등 품질 손실 없는 포맷
- **SVG_Preset**: SVG 트레이싱 시 사용하는 사전 설정 (bw, poster, photo)
- **DPI**: SVG 래스터화 시 해상도를 결정하는 인치당 도트 수
- **Feature_Flag**: Cargo의 조건부 컴파일 기능으로, 선택적 의존성을 포함/제외하는 메커니즘
- **JXL_Decoder**: JPEG XL 파일을 디코딩하는 모듈 (jxl-oxide 기반, 읽기 전용)
- **DDS_Decoder**: DDS(DirectDraw Surface) 파일을 디코딩하는 모듈 (image 크레이트 dds feature 기반, 읽기 전용)
- **PCX_Codec**: PCX 포맷의 읽기/쓰기를 수행하는 모듈 (pcx 크레이트 기반)
- **UltraHDR_Codec**: Ultra HDR JPEG(Gain Map) 포맷의 읽기/쓰기를 수행하는 모듈 (ultrahdr-core 크레이트 기반)
- **APNG_Codec**: APNG(Animated PNG) 포맷의 읽기/쓰기를 수행하는 모듈 (image + apng 크레이트 기반)
- **Cropper**: 이미지의 특정 영역을 잘라내는 모듈 (image 크레이트 crop_imm 기반)
- **Transformer**: 이미지 회전 및 뒤집기를 수행하는 모듈 (image 크레이트 기반)
- **Color_Filter**: 이미지에 색상 필터(흑백, 반전, 세피아)를 적용하는 모듈
- **Blur_Sharpen**: 이미지에 블러 또는 샤프닝 효과를 적용하는 모듈 (image 크레이트 기반)
- **Text_Watermarker**: 이미지에 텍스트 워터마크를 삽입하는 모듈 (imageproc + ab_glyph 기반)
- **Overlay_Composer**: 이미지 위에 다른 이미지를 오버레이(합성)하는 모듈 (image 크레이트 overlay 기반)
- **EXIF_Orienter**: EXIF Orientation 태그를 읽어 이미지 방향을 자동 보정하는 모듈 (kamadak-exif 기반)
- **Brightness_Contrast_Controller**: 이미지의 밝기, 대비, 감마를 조정하는 모듈 (image 크레이트 기반)
- **EXIF_Preserver**: EXIF 메타데이터를 원본에서 변환 결과로 복사하는 모듈 (kamadak-exif + img-parts 기반)
- **Info_Reporter**: 이미지의 메타데이터(크기, 포맷, 색상 공간 등)를 출력하는 모듈
- **Image_Comparator**: 두 이미지 간 품질 차이를 SSIM/PSNR 메트릭으로 비교하는 모듈 (image-compare 기반)
- **Dedup_Checker**: 입력/출력 파일의 해시를 비교하여 중복 변환을 건너뛰는 모듈 (sha2 기반)
- **Preset_Manager**: 용도별 사전 정의된 변환 설정을 관리하는 모듈
- **SSIM**: 구조적 유사성 지수(Structural Similarity Index Measure), 두 이미지 간 지각적 품질 차이를 측정하는 메트릭
- **PSNR**: 최대 신호 대 잡음비(Peak Signal-to-Noise Ratio), 두 이미지 간 픽셀 수준 차이를 측정하는 메트릭
- **Gain_Map**: Ultra HDR JPEG에서 SDR과 HDR 간 밝기 차이를 보정하는 보조 데이터

## 요구사항

### 요구사항 1: 래스터 포맷 간 상호 변환

**사용자 스토리:** 개발자로서, 주요 래스터 이미지 포맷 간 변환을 수행하고 싶다. 이를 통해 다양한 플랫폼과 용도에 맞는 이미지 포맷을 생성할 수 있다.

#### 인수 조건

1. WHEN 사용자가 지원되는 래스터 포맷 파일과 `--to` 옵션으로 대상 래스터 포맷을 지정하면, THE Converter SHALL 입력 파일을 대상 포맷으로 변환하여 저장한다.
2. THE Converter SHALL JPEG(.jpg, .jpeg), PNG(.png), GIF(.gif), BMP(.bmp), TIFF(.tif, .tiff), TGA(.tga), ICO(.ico), QOI(.qoi), PNM(.ppm, .pgm, .pbm, .pam), OpenEXR(.exr), Radiance HDR(.hdr), Farbfeld(.ff) 포맷의 읽기와 쓰기를 모두 지원한다.
3. WHEN 입력 파일의 확장자가 지원되지 않는 포맷이면, THE CLI SHALL 지원 포맷 목록과 함께 오류 메시지를 출력한다.
4. WHEN `--to` 옵션에 지원되지 않는 포맷이 지정되면, THE CLI SHALL 지원 포맷 목록과 함께 오류 메시지를 출력한다.

### 요구사항 2: WebP 포맷 변환

**사용자 스토리:** 개발자로서, WebP 포맷으로의 변환 및 WebP로부터의 변환을 수행하고 싶다. 이를 통해 웹 최적화된 이미지를 생성하거나 WebP 이미지를 다른 포맷으로 변환할 수 있다.

#### 인수 조건

1. WHEN 사용자가 이미지 파일과 `--to webp`를 지정하면, THE Converter SHALL zenwebp 크레이트를 사용하여 WebP 포맷으로 변환한다.
2. WHEN 사용자가 WebP 파일과 `--to` 옵션으로 다른 포맷을 지정하면, THE Converter SHALL zenwebp 크레이트를 사용하여 WebP 파일을 디코딩한 후 대상 포맷으로 변환한다.
3. WHEN `--lossy` 옵션이 지정되면, THE Converter SHALL WebP lossy 인코딩을 사용한다.
4. WHEN `--lossless` 옵션이 지정되면, THE Converter SHALL WebP lossless 인코딩을 사용한다.
5. WHEN WebP 변환 시 `--lossy`와 `--lossless` 옵션이 모두 지정되지 않으면, THE Converter SHALL 기본값으로 lossy 인코딩(quality 75)을 사용한다.

### 요구사항 3: SVG 변환

**사용자 스토리:** 디자이너/개발자로서, SVG와 래스터 이미지 간 양방향 변환을 수행하고 싶다. 이를 통해 벡터 그래픽을 다양한 해상도의 래스터 이미지로 내보내거나, 래스터 이미지를 벡터화할 수 있다.

#### 인수 조건

1. WHEN 사용자가 SVG 파일과 `--to` 옵션으로 래스터 포맷을 지정하면, THE Rasterizer SHALL resvg와 tiny-skia를 사용하여 SVG를 래스터 이미지로 변환한다.
2. WHEN SVG 래스터화 시 `--dpi` 옵션이 지정되면, THE Rasterizer SHALL 해당 DPI 값을 사용하여 래스터화한다.
3. WHEN SVG 래스터화 시 `--dpi` 옵션이 지정되지 않으면, THE Rasterizer SHALL 기본값 96 DPI를 사용한다.
4. WHEN SVG 래스터화 시 `--width` 옵션이 지정되면, THE Rasterizer SHALL 해당 너비로 래스터화한다.
5. WHEN 사용자가 래스터 이미지 파일과 `--to svg`를 지정하면, THE Tracer SHALL vtracer를 사용하여 래스터 이미지를 SVG로 트레이싱한다.
6. WHEN SVG 트레이싱 시 `--svg-preset` 옵션이 지정되면, THE Tracer SHALL 해당 프리셋(bw, poster, photo)을 적용한다.
7. WHEN SVG 트레이싱 시 `--svg-preset` 옵션이 지정되지 않으면, THE Tracer SHALL 기본 프리셋을 적용한다.

### 요구사항 4: AVIF 포맷 변환 (선택적 Feature Flag)

**사용자 스토리:** 개발자로서, AVIF 포맷 변환을 선택적으로 활성화하고 싶다. 이를 통해 필요한 경우에만 AVIF 관련 의존성을 포함하여 빌드 시간과 바이너리 크기를 최적화할 수 있다.

#### 인수 조건

1. WHERE Cargo feature flag `avif`가 활성화된 경우, THE Converter SHALL 래스터 이미지를 AVIF 포맷으로 인코딩하는 기능을 제공한다.
2. WHERE Cargo feature flag `avif`가 활성화된 경우, THE Converter SHALL AVIF 파일을 래스터 이미지로 디코딩하는 기능을 제공한다.
3. WHERE Cargo feature flag `avif`가 비활성화된 경우, THE CLI SHALL AVIF 관련 의존성을 빌드에 포함하지 않는다.
4. WHERE Cargo feature flag `avif`가 비활성화된 상태에서 WHEN 사용자가 AVIF 변환을 요청하면, THE CLI SHALL `avif` feature flag 활성화 방법을 안내하는 오류 메시지를 출력한다.
5. WHERE Cargo feature flag `avif`가 활성화된 경우, THE Converter SHALL AVIF 인코딩에 ravif(rav1e 기반)를, AVIF 디코딩에 avif-parse와 rav1d를 사용한다.

### 요구사항 5: 단일 파일 변환

**사용자 스토리:** 사용자로서, 단일 이미지 파일을 원하는 포맷으로 변환하고 싶다. 이를 통해 개별 파일을 빠르게 변환할 수 있다.

#### 인수 조건

1. WHEN 사용자가 단일 파일 경로와 `--to` 옵션을 지정하면, THE CLI SHALL 해당 파일을 대상 포맷으로 변환한다.
2. WHEN 출력 디렉토리가 `-o` 옵션으로 지정되면, THE CLI SHALL 변환된 파일을 해당 디렉토리에 저장한다.
3. WHEN 출력 디렉토리가 지정되지 않으면, THE CLI SHALL 변환된 파일을 입력 파일과 동일한 디렉토리에 저장한다.
4. THE CLI SHALL 변환된 파일의 이름을 원본 파일명에 대상 포맷 확장자를 적용하여 생성한다.

### 요구사항 6: 디렉토리 일괄(Batch) 변환

**사용자 스토리:** 사용자로서, 디렉토리 내 모든 이미지 파일을 한 번에 변환하고 싶다. 이를 통해 대량의 이미지를 효율적으로 처리할 수 있다.

#### 인수 조건

1. WHEN 사용자가 디렉토리 경로와 `--to` 옵션을 지정하면, THE Batch_Processor SHALL 해당 디렉토리 내 지원되는 모든 이미지 파일을 대상 포맷으로 변환한다.
2. THE Batch_Processor SHALL rayon을 사용하여 여러 파일을 병렬로 변환한다.
3. WHEN 일괄 변환 중 일부 파일의 변환이 실패하면, THE Batch_Processor SHALL 실패한 파일을 건너뛰고 나머지 파일의 변환을 계속 진행한다.
4. WHEN 일괄 변환이 완료되면, THE CLI SHALL 성공 파일 수, 실패 파일 수, 실패한 파일 목록을 포함한 최종 요약을 출력한다.
5. WHEN 디렉토리 내에 지원되는 이미지 파일이 없으면, THE CLI SHALL 변환 가능한 파일이 없음을 알리는 메시지를 출력한다.

### 요구사항 7: 다중 출력 포맷 동시 변환

**사용자 스토리:** 사용자로서, 하나의 입력 파일에서 여러 포맷의 출력을 동시에 생성하고 싶다. 이를 통해 다양한 용도에 맞는 이미지를 한 번에 생성할 수 있다.

#### 인수 조건

1. WHEN 사용자가 `--to` 옵션에 쉼표로 구분된 여러 포맷을 지정하면, THE Converter SHALL 입력 파일을 지정된 모든 포맷으로 변환한다.
2. WHEN 다중 포맷 변환 시 일부 포맷 변환이 실패하면, THE Converter SHALL 실패한 포맷을 건너뛰고 나머지 포맷의 변환을 계속 진행한다.
3. WHEN 다중 포맷 변환이 완료되면, THE CLI SHALL 각 포맷별 변환 결과를 포함한 요약을 출력한다.

### 요구사항 8: 이미지 리사이즈

**사용자 스토리:** 사용자로서, 변환 시 이미지 크기를 조정하고 싶다. 이를 통해 웹 최적화나 특정 해상도 요구사항에 맞는 이미지를 생성할 수 있다.

#### 인수 조건

1. WHEN `--width` 옵션이 지정되면, THE Resizer SHALL 이미지의 너비를 해당 값(픽셀)으로 조정한다.
2. WHEN `--height` 옵션이 지정되면, THE Resizer SHALL 이미지의 높이를 해당 값(픽셀)으로 조정한다.
3. WHEN `--width`와 `--height`가 모두 지정되고 `--keep-aspect` 옵션이 지정되면, THE Resizer SHALL 지정된 너비와 높이 내에서 원본 종횡비를 유지하며 리사이즈한다.
4. WHEN `--width`만 지정되고 `--height`가 지정되지 않으면, THE Resizer SHALL 원본 종횡비를 유지하며 너비에 맞춰 높이를 자동 계산한다.
5. WHEN `--height`만 지정되고 `--width`가 지정되지 않으면, THE Resizer SHALL 원본 종횡비를 유지하며 높이에 맞춰 너비를 자동 계산한다.
6. WHEN `--width`와 `--height`가 모두 지정되고 `--keep-aspect` 옵션이 지정되지 않으면, THE Resizer SHALL 지정된 너비와 높이로 이미지를 강제 리사이즈한다.

### 요구사항 9: 품질(Quality) 설정

**사용자 스토리:** 사용자로서, 손실 압축 포맷의 품질을 조정하고 싶다. 이를 통해 파일 크기와 이미지 품질 간 균형을 맞출 수 있다.

#### 인수 조건

1. WHEN `--quality` 옵션이 1에서 100 사이의 정수로 지정되면, THE Quality_Controller SHALL 해당 값을 손실 압축 포맷의 품질 파라미터로 적용한다.
2. WHEN `--quality` 옵션이 지정되지 않으면, THE Quality_Controller SHALL 포맷별 기본 품질 값을 적용한다 (JPEG: 85, WebP lossy: 75, AVIF: 70).
3. WHEN `--quality` 옵션이 1 미만이거나 100 초과인 값으로 지정되면, THE CLI SHALL 유효 범위(1-100)를 안내하는 오류 메시지를 출력한다.
4. WHEN `--quality` 옵션이 무손실 포맷(PNG, BMP 등)에 대해 지정되면, THE CLI SHALL 해당 포맷은 품질 설정을 지원하지 않음을 알리는 경고 메시지를 출력하고 변환을 계속 진행한다.

### 요구사항 10: 원본 파일 보호

**사용자 스토리:** 사용자로서, 변환 시 원본 파일이 실수로 덮어쓰기되지 않도록 보호하고 싶다. 이를 통해 원본 데이터의 안전성을 보장할 수 있다.

#### 인수 조건

1. WHEN 변환 결과 파일이 이미 존재하고 `--overwrite` 옵션이 지정되지 않으면, THE CLI SHALL 해당 파일을 건너뛰고 덮어쓰기 방지 메시지를 출력한다.
2. WHEN `--overwrite` 옵션이 지정되면, THE CLI SHALL 기존 파일을 덮어쓰기한다.
3. WHEN 입력 파일과 출력 파일의 경로가 동일하면, THE CLI SHALL 원본 파일 보호를 위해 변환을 거부하고 오류 메시지를 출력한다.

### 요구사항 11: 변환 진행률 표시

**사용자 스토리:** 사용자로서, 일괄 변환 시 진행 상황을 실시간으로 확인하고 싶다. 이를 통해 변환 완료까지 남은 시간을 예측할 수 있다.

#### 인수 조건

1. WHILE 일괄 변환이 진행 중인 동안, THE Progress_Reporter SHALL indicatif를 사용하여 진행률 바를 터미널에 표시한다.
2. THE Progress_Reporter SHALL 현재 처리 중인 파일 수, 전체 파일 수, 경과 시간을 진행률 바에 포함한다.
3. WHEN 단일 파일 변환 시, THE Progress_Reporter SHALL 진행률 바 대신 변환 완료 메시지만 출력한다.

### 요구사항 12: Dry-Run 모드

**사용자 스토리:** 사용자로서, 실제 변환 전에 변환 계획을 미리 확인하고 싶다. 이를 통해 의도하지 않은 변환을 방지할 수 있다.

#### 인수 조건

1. WHEN `--dry-run` 옵션이 지정되면, THE CLI SHALL 실제 변환을 수행하지 않고 변환 계획(입력 파일, 출력 파일, 적용될 옵션)을 출력한다.
2. WHEN `--dry-run` 옵션이 지정되면, THE CLI SHALL 파일 시스템에 어떠한 변경도 수행하지 않는다.

### 요구사항 13: 상세 로그 출력

**사용자 스토리:** 사용자로서, 변환 과정의 상세 정보를 확인하고 싶다. 이를 통해 문제 발생 시 원인을 파악할 수 있다.

#### 인수 조건

1. WHEN `--verbose` 옵션이 지정되면, THE CLI SHALL 각 파일의 변환 과정(입력 포맷, 출력 포맷, 파일 크기, 소요 시간)을 상세히 출력한다.
2. WHEN `--verbose` 옵션이 지정되지 않으면, THE CLI SHALL 최종 요약만 출력한다.

### 요구사항 14: 종료 코드

**사용자 스토리:** 자동화 스크립트 작성자로서, imgconv의 종료 코드를 통해 변환 결과를 프로그래밍적으로 판단하고 싶다. 이를 통해 CI/CD 파이프라인에서 변환 결과에 따른 분기 처리를 할 수 있다.

#### 인수 조건

1. WHEN 모든 파일의 변환이 성공하면, THE CLI SHALL 종료 코드 0을 반환한다.
2. WHEN 일부 파일의 변환이 실패하면, THE CLI SHALL 종료 코드 1을 반환한다.
3. WHEN 모든 파일의 변환이 실패하거나 인자 오류가 발생하면, THE CLI SHALL 종료 코드 2를 반환한다.

### 요구사항 15: Pure Rust 빌드

**사용자 스토리:** 개발자로서, C 컴파일러 없이 `cargo build --release`만으로 빌드를 완료하고 싶다. 이를 통해 빌드 환경 설정을 단순화하고 크로스 플랫폼 빌드를 용이하게 할 수 있다.

#### 인수 조건

1. THE CLI SHALL 외부 C 라이브러리 바인딩 없이 pure Rust 의존성만으로 빌드된다.
2. THE CLI SHALL `cargo build --release` 명령만으로 Linux, macOS, Windows에서 빌드 가능하다.
3. THE CLI SHALL 싱글 바이너리로 배포 가능하다.
4. THE CLI SHALL build.rs에서 C 컴파일러를 호출하지 않는다.

### 요구사항 16: 라이브러리와 바이너리 분리

**사용자 스토리:** 개발자로서, imgconv의 변환 기능을 다른 Rust 프로젝트에서 라이브러리로 재사용하고 싶다. 이를 통해 CLI 없이도 이미지 변환 기능을 프로그래밍적으로 활용할 수 있다.

#### 인수 조건

1. THE Converter SHALL 라이브러리 크레이트(lib.rs)로 구현되어 외부 프로젝트에서 의존성으로 사용 가능하다.
2. THE CLI SHALL 바이너리 크레이트(main.rs)로 구현되어 라이브러리 크레이트를 호출한다.
3. THE Converter SHALL CLI 의존성(clap, indicatif) 없이 독립적으로 사용 가능하다.

### 요구사항 17: CLI 인자 파싱

**사용자 스토리:** 사용자로서, 직관적인 CLI 인터페이스를 통해 변환 옵션을 지정하고 싶다. 이를 통해 복잡한 변환 작업을 간단한 명령어로 수행할 수 있다.

#### 인수 조건

1. THE CLI SHALL clap(derive 방식)을 사용하여 커맨드라인 인자를 파싱한다.
2. THE CLI SHALL 다음 인자를 지원한다: 입력 경로(위치 인자), `--to`(대상 포맷), `--quality`, `--width`, `--height`, `--keep-aspect`, `--lossy`, `--lossless`, `--dpi`, `--svg-preset`, `--overwrite`, `--dry-run`, `--verbose`, `-o`(출력 디렉토리), `--crop`, `--rotate`, `--flip`, `--grayscale`, `--invert`, `--sepia`, `--blur`, `--sharpen`, `--watermark`, `--watermark-position`, `--watermark-opacity`, `--watermark-font`, `--overlay`, `--overlay-position`, `--overlay-opacity`, `--auto-orient`, `--no-auto-orient`, `--brightness`, `--contrast`, `--gamma`, `--preserve-exif`, `--info`, `--compare`, `--skip-identical`, `--preset`.
3. WHEN 필수 인자(`--to`)가 누락되면, THE CLI SHALL 사용법 안내와 함께 오류 메시지를 출력한다.
4. WHEN 잘못된 인자 조합이 지정되면, THE CLI SHALL 구체적인 오류 원인을 설명하는 메시지를 출력한다.

### 요구사항 18: JPEG XL 디코딩 (선택적 Feature Flag)

**사용자 스토리:** 개발자로서, JPEG XL 포맷의 이미지를 다른 포맷으로 변환하고 싶다. 이를 통해 JPEG XL로 저장된 이미지를 범용 포맷으로 내보낼 수 있다.

#### 인수 조건

1. WHERE Cargo feature flag `jxl`이 활성화된 경우, THE JXL_Decoder SHALL jxl-oxide 크레이트를 사용하여 JPEG XL(.jxl) 파일을 디코딩한다.
2. WHERE Cargo feature flag `jxl`이 활성화된 경우, WHEN 사용자가 JXL 파일과 `--to` 옵션으로 대상 포맷을 지정하면, THE Converter SHALL JXL 파일을 디코딩한 후 대상 포맷으로 변환한다.
3. WHERE Cargo feature flag `jxl`이 비활성화된 상태에서 WHEN 사용자가 JXL 변환을 요청하면, THE CLI SHALL `jxl` feature flag 활성화 방법을 안내하는 오류 메시지를 출력한다.
4. THE JXL_Decoder SHALL 읽기 전용으로 동작하며, JXL 인코딩 기능은 제공하지 않는다.
5. THE JXL_Decoder SHALL pure Rust 크레이트(jxl-oxide)만 사용하여 C 의존성 없이 빌드된다.

### 요구사항 19: DDS 포맷 읽기 (선택적 Feature Flag)

**사용자 스토리:** 게임 개발자로서, DDS(DirectDraw Surface) 텍스처 파일을 범용 이미지 포맷으로 변환하고 싶다. 이를 통해 3D 텍스처를 일반 이미지 편집 도구에서 활용할 수 있다.

#### 인수 조건

1. WHERE Cargo feature flag `dds`가 활성화된 경우, THE DDS_Decoder SHALL image 크레이트의 dds feature를 사용하여 DDS(.dds) 파일을 디코딩한다.
2. WHERE Cargo feature flag `dds`가 활성화된 경우, WHEN 사용자가 DDS 파일과 `--to` 옵션으로 대상 포맷을 지정하면, THE Converter SHALL DDS 파일을 디코딩한 후 대상 포맷으로 변환한다.
3. WHERE Cargo feature flag `dds`가 비활성화된 상태에서 WHEN 사용자가 DDS 변환을 요청하면, THE CLI SHALL `dds` feature flag 활성화 방법을 안내하는 오류 메시지를 출력한다.
4. THE DDS_Decoder SHALL 읽기 전용으로 동작하며, DDS 인코딩 기능은 제공하지 않는다.

### 요구사항 20: PCX 포맷 변환 (선택적 Feature Flag)

**사용자 스토리:** 사용자로서, 레거시 PCX 포맷 이미지를 현대적인 포맷으로 변환하거나, 현대 포맷을 PCX로 변환하고 싶다. 이를 통해 레거시 시스템과의 호환성을 유지할 수 있다.

#### 인수 조건

1. WHERE Cargo feature flag `pcx`가 활성화된 경우, THE PCX_Codec SHALL pcx 크레이트를 사용하여 PCX(.pcx) 파일의 읽기와 쓰기를 모두 지원한다.
2. WHERE Cargo feature flag `pcx`가 활성화된 경우, WHEN 사용자가 PCX 파일과 `--to` 옵션으로 대상 포맷을 지정하면, THE Converter SHALL PCX 파일을 디코딩한 후 대상 포맷으로 변환한다.
3. WHERE Cargo feature flag `pcx`가 활성화된 경우, WHEN 사용자가 이미지 파일과 `--to pcx`를 지정하면, THE Converter SHALL 입력 이미지를 PCX 포맷으로 인코딩한다.
4. WHERE Cargo feature flag `pcx`가 비활성화된 상태에서 WHEN 사용자가 PCX 변환을 요청하면, THE CLI SHALL `pcx` feature flag 활성화 방법을 안내하는 오류 메시지를 출력한다.
5. THE PCX_Codec SHALL pure Rust 크레이트(pcx)만 사용하여 C 의존성 없이 빌드된다.

### 요구사항 21: Ultra HDR JPEG 변환 (선택적 Feature Flag)

**사용자 스토리:** 개발자로서, Ultra HDR JPEG(Gain Map) 이미지를 처리하고 싶다. 이를 통해 HDR 디스플레이 대응 이미지를 생성하거나 SDR 호환 HDR 이미지를 변환할 수 있다.

#### 인수 조건

1. WHERE Cargo feature flag `ultrahdr`가 활성화된 경우, THE UltraHDR_Codec SHALL ultrahdr-core 크레이트를 사용하여 Ultra HDR JPEG 파일의 읽기와 쓰기를 지원한다.
2. WHERE Cargo feature flag `ultrahdr`가 활성화된 경우, WHEN 사용자가 Ultra HDR JPEG 파일과 `--to` 옵션으로 대상 포맷을 지정하면, THE Converter SHALL Ultra HDR JPEG 파일을 디코딩한 후 대상 포맷으로 변환한다.
3. WHERE Cargo feature flag `ultrahdr`가 비활성화된 상태에서 WHEN 사용자가 Ultra HDR 변환을 요청하면, THE CLI SHALL `ultrahdr` feature flag 활성화 방법을 안내하는 오류 메시지를 출력한다.
4. THE UltraHDR_Codec SHALL pure Rust 크레이트(ultrahdr-core)만 사용하여 C 의존성 없이 빌드된다.

### 요구사항 22: APNG 지원 (선택적 Feature Flag)

**사용자 스토리:** 개발자로서, APNG(Animated PNG) 파일을 읽고 쓰고 싶다. 이를 통해 애니메이션 이미지를 다른 포맷으로 변환하거나 APNG로 생성할 수 있다.

#### 인수 조건

1. WHERE Cargo feature flag `apng`가 활성화된 경우, THE APNG_Codec SHALL image 크레이트의 PNG 디코더를 사용하여 APNG 파일의 개별 프레임을 읽는다.
2. WHERE Cargo feature flag `apng`가 활성화된 경우, THE APNG_Codec SHALL apng 크레이트를 사용하여 여러 프레임을 APNG 파일로 인코딩한다.
3. WHERE Cargo feature flag `apng`가 활성화된 경우, WHEN 사용자가 APNG 파일과 `--to` 옵션으로 대상 포맷을 지정하면, THE Converter SHALL APNG의 첫 번째 프레임을 대상 포맷으로 변환한다.
4. WHERE Cargo feature flag `apng`가 비활성화된 상태에서 WHEN 사용자가 APNG 변환을 요청하면, THE CLI SHALL `apng` feature flag 활성화 방법을 안내하는 오류 메시지를 출력한다.
5. THE APNG_Codec SHALL pure Rust 크레이트(image + apng)만 사용하여 C 의존성 없이 빌드된다.

### 요구사항 23: 이미지 크롭

**사용자 스토리:** 사용자로서, 이미지의 특정 영역만 잘라내고 싶다. 이를 통해 불필요한 부분을 제거하고 원하는 영역만 추출할 수 있다.

#### 인수 조건

1. WHEN `--crop x,y,w,h` 옵션이 지정되면, THE Cropper SHALL (x,y) 좌표에서 시작하여 너비 w, 높이 h 크기의 영역을 잘라낸다.
2. WHEN `--crop` 옵션과 `--width`/`--height` 옵션이 함께 지정되면, THE Converter SHALL 크롭을 먼저 수행한 후 리사이즈를 적용한다.
3. WHEN `--crop` 옵션의 좌표가 이미지 범위를 초과하면, THE CLI SHALL 유효 범위를 안내하는 오류 메시지를 출력한다.
4. WHEN `--crop` 옵션의 값이 `x,y,w,h` 형식이 아니면, THE CLI SHALL 올바른 형식을 안내하는 오류 메시지를 출력한다.

### 요구사항 24: 이미지 회전 및 뒤집기

**사용자 스토리:** 사용자로서, 이미지를 회전하거나 뒤집고 싶다. 이를 통해 이미지 방향을 원하는 대로 조정할 수 있다.

#### 인수 조건

1. WHEN `--rotate 90` 옵션이 지정되면, THE Transformer SHALL 이미지를 시계 방향으로 90도 회전한다.
2. WHEN `--rotate 180` 옵션이 지정되면, THE Transformer SHALL 이미지를 180도 회전한다.
3. WHEN `--rotate 270` 옵션이 지정되면, THE Transformer SHALL 이미지를 시계 방향으로 270도 회전한다.
4. WHEN `--flip horizontal` 옵션이 지정되면, THE Transformer SHALL 이미지를 수평으로 뒤집는다.
5. WHEN `--flip vertical` 옵션이 지정되면, THE Transformer SHALL 이미지를 수직으로 뒤집는다.
6. WHEN `--rotate` 옵션에 90, 180, 270 이외의 값이 지정되면, THE CLI SHALL 유효한 회전 각도(90, 180, 270)를 안내하는 오류 메시지를 출력한다.
7. WHEN `--flip` 옵션에 horizontal, vertical 이외의 값이 지정되면, THE CLI SHALL 유효한 뒤집기 방향(horizontal, vertical)을 안내하는 오류 메시지를 출력한다.

### 요구사항 25: 색상 필터

**사용자 스토리:** 사용자로서, 이미지에 색상 필터를 적용하고 싶다. 이를 통해 흑백, 반전, 세피아 등의 효과를 간편하게 적용할 수 있다.

#### 인수 조건

1. WHEN `--grayscale` 옵션이 지정되면, THE Color_Filter SHALL 이미지를 흑백(그레이스케일)으로 변환한다.
2. WHEN `--invert` 옵션이 지정되면, THE Color_Filter SHALL 이미지의 색상을 반전한다.
3. WHEN `--sepia` 옵션이 지정되면, THE Color_Filter SHALL 이미지에 세피아 톤 효과를 적용한다.
4. WHEN 여러 색상 필터 옵션이 동시에 지정되면, THE Color_Filter SHALL `--grayscale` → `--sepia` → `--invert` 순서로 필터를 적용한다.

### 요구사항 26: 블러 및 샤프닝

**사용자 스토리:** 사용자로서, 이미지에 블러 또는 샤프닝 효과를 적용하고 싶다. 이를 통해 이미지의 선명도를 조정할 수 있다.

#### 인수 조건

1. WHEN `--blur` 옵션이 양수 실수 값(sigma)으로 지정되면, THE Blur_Sharpen SHALL 해당 sigma 값으로 가우시안 블러를 적용한다.
2. WHEN `--sharpen` 옵션이 양수 실수 값으로 지정되면, THE Blur_Sharpen SHALL 해당 값으로 언샤프 마스크를 적용한다.
3. WHEN `--blur` 옵션의 값이 0 이하이면, THE CLI SHALL 양수 값을 입력하도록 안내하는 오류 메시지를 출력한다.
4. WHEN `--sharpen` 옵션의 값이 0 이하이면, THE CLI SHALL 양수 값을 입력하도록 안내하는 오류 메시지를 출력한다.
5. WHEN `--blur`와 `--sharpen` 옵션이 동시에 지정되면, THE CLI SHALL 두 옵션을 동시에 사용할 수 없음을 안내하는 오류 메시지를 출력한다.

### 요구사항 27: 텍스트 워터마크

**사용자 스토리:** 사용자로서, 이미지에 텍스트 워터마크를 삽입하고 싶다. 이를 통해 저작권 표시나 브랜딩을 이미지에 추가할 수 있다.

#### 인수 조건

1. WHEN `--watermark` 옵션이 텍스트 문자열과 함께 지정되면, THE Text_Watermarker SHALL 해당 텍스트를 이미지에 워터마크로 삽입한다.
2. WHEN `--watermark-position` 옵션이 지정되면, THE Text_Watermarker SHALL 해당 위치(top-left, top-right, bottom-left, bottom-right, center)에 워터마크를 배치한다.
3. WHEN `--watermark-position` 옵션이 지정되지 않으면, THE Text_Watermarker SHALL 기본 위치(bottom-right)에 워터마크를 배치한다.
4. WHEN `--watermark-opacity` 옵션이 0.0에서 1.0 사이의 실수로 지정되면, THE Text_Watermarker SHALL 해당 투명도로 워터마크를 렌더링한다.
5. WHEN `--watermark-opacity` 옵션이 지정되지 않으면, THE Text_Watermarker SHALL 기본 투명도 0.5로 워터마크를 렌더링한다.
6. WHEN `--watermark-font` 옵션이 TrueType 폰트 파일 경로와 함께 지정되면, THE Text_Watermarker SHALL 해당 폰트를 사용하여 워터마크를 렌더링한다.
7. WHEN `--watermark-font` 옵션이 지정되지 않으면, THE Text_Watermarker SHALL 내장 기본 폰트를 사용한다.
8. THE Text_Watermarker SHALL pure Rust 크레이트(imageproc + ab_glyph)만 사용하여 C 의존성 없이 빌드된다.

### 요구사항 28: 이미지 오버레이

**사용자 스토리:** 사용자로서, 이미지 위에 로고나 다른 이미지를 합성하고 싶다. 이를 통해 로고 워터마크나 이미지 합성을 수행할 수 있다.

#### 인수 조건

1. WHEN `--overlay` 옵션이 이미지 파일 경로와 함께 지정되면, THE Overlay_Composer SHALL 해당 이미지를 대상 이미지 위에 합성한다.
2. WHEN `--overlay-position` 옵션이 지정되면, THE Overlay_Composer SHALL 해당 위치(top-left, top-right, bottom-left, bottom-right, center)에 오버레이 이미지를 배치한다.
3. WHEN `--overlay-position` 옵션이 지정되지 않으면, THE Overlay_Composer SHALL 기본 위치(bottom-right)에 오버레이 이미지를 배치한다.
4. WHEN `--overlay-opacity` 옵션이 0.0에서 1.0 사이의 실수로 지정되면, THE Overlay_Composer SHALL 해당 투명도로 오버레이 이미지를 합성한다.
5. WHEN `--overlay-opacity` 옵션이 지정되지 않으면, THE Overlay_Composer SHALL 기본 투명도 1.0(불투명)으로 오버레이 이미지를 합성한다.
6. WHEN `--overlay` 옵션에 지정된 파일이 존재하지 않으면, THE CLI SHALL 파일을 찾을 수 없음을 안내하는 오류 메시지를 출력한다.
7. WHEN `--overlay` 옵션에 지정된 파일이 지원되지 않는 이미지 포맷이면, THE CLI SHALL 지원 포맷 목록과 함께 오류 메시지를 출력한다.

### 요구사항 29: 자동 EXIF 방향 보정

**사용자 스토리:** 사용자로서, 스마트폰 등에서 촬영한 이미지의 방향을 EXIF 정보에 따라 자동으로 보정하고 싶다. 이를 통해 이미지가 올바른 방향으로 표시되도록 할 수 있다.

#### 인수 조건

1. WHEN `--auto-orient` 옵션이 지정되면, THE EXIF_Orienter SHALL kamadak-exif 크레이트를 사용하여 EXIF Orientation 태그를 읽고 이미지를 올바른 방향으로 회전/뒤집기한다.
2. WHEN `--no-auto-orient` 옵션이 지정되면, THE EXIF_Orienter SHALL EXIF 방향 보정을 수행하지 않는다.
3. WHEN `--auto-orient`와 `--no-auto-orient` 옵션이 모두 지정되지 않으면, THE EXIF_Orienter SHALL EXIF 방향 보정을 수행하지 않는다.
4. WHEN 입력 파일에 EXIF Orientation 태그가 없으면, THE EXIF_Orienter SHALL 보정 없이 원본 이미지를 그대로 전달한다.
5. THE EXIF_Orienter SHALL pure Rust 크레이트(kamadak-exif)만 사용하여 C 의존성 없이 빌드된다.

### 요구사항 30: 밝기, 대비, 감마 조정

**사용자 스토리:** 사용자로서, 이미지의 밝기, 대비, 감마를 조정하고 싶다. 이를 통해 이미지의 노출과 톤을 세밀하게 제어할 수 있다.

#### 인수 조건

1. WHEN `--brightness` 옵션이 정수 값으로 지정되면, THE Brightness_Contrast_Controller SHALL 해당 값만큼 이미지의 밝기를 조정한다 (양수: 밝게, 음수: 어둡게).
2. WHEN `--contrast` 옵션이 실수 값으로 지정되면, THE Brightness_Contrast_Controller SHALL 해당 값으로 이미지의 대비를 조정한다.
3. WHEN `--gamma` 옵션이 양수 실수 값으로 지정되면, THE Brightness_Contrast_Controller SHALL 해당 값으로 감마 보정을 적용한다.
4. WHEN `--gamma` 옵션의 값이 0 이하이면, THE CLI SHALL 양수 값을 입력하도록 안내하는 오류 메시지를 출력한다.
5. WHEN `--brightness`, `--contrast`, `--gamma` 옵션이 동시에 지정되면, THE Brightness_Contrast_Controller SHALL `--brightness` → `--contrast` → `--gamma` 순서로 조정을 적용한다.

### 요구사항 31: EXIF 메타데이터 보존

**사용자 스토리:** 사용자로서, 이미지 변환 시 원본의 EXIF 메타데이터를 보존하고 싶다. 이를 통해 촬영 정보, GPS 데이터 등 중요한 메타데이터를 유지할 수 있다.

#### 인수 조건

1. WHEN `--preserve-exif` 옵션이 지정되면, THE EXIF_Preserver SHALL kamadak-exif 크레이트로 원본 파일의 EXIF 메타데이터를 읽고, img-parts 크레이트로 변환된 파일에 EXIF 메타데이터를 기록한다.
2. WHEN `--preserve-exif` 옵션이 지정되고 JPEG에서 JPEG로 변환하는 경우, THE EXIF_Preserver SHALL 원본의 EXIF 데이터를 변환 결과에 복사한다.

3. WHEN `--preserve-exif` 옵션이 지정되고 크로스 포맷 변환(JPEG → PNG 등)인 경우, THE CLI SHALL EXIF 보존이 제한적임을 알리는 경고 메시지를 출력한다.
4. WHEN `--preserve-exif` 옵션이 지정되지 않으면, THE Converter SHALL EXIF 메타데이터를 보존하지 않는다.
5. THE EXIF_Preserver SHALL pure Rust 크레이트(kamadak-exif + img-parts)만 사용하여 C 의존성 없이 빌드된다.

### 요구사항 32: 이미지 정보 출력

**사용자 스토리:** 사용자로서, 이미지 파일의 메타데이터를 변환 없이 확인하고 싶다. 이를 통해 이미지의 크기, 포맷, 색상 정보 등을 빠르게 파악할 수 있다.

#### 인수 조건

1. WHEN `--info` 옵션이 지정되면, THE Info_Reporter SHALL 변환을 수행하지 않고 이미지의 메타데이터(너비, 높이, 포맷, 색상 공간, 비트 깊이)를 출력한다.
2. WHEN `--info` 옵션이 지정되면, THE Info_Reporter SHALL 파일 시스템에 어떠한 변경도 수행하지 않는다.
3. WHEN `--info` 옵션이 지정되고 입력 파일에 EXIF 데이터가 존재하면, THE Info_Reporter SHALL EXIF 요약 정보(카메라 모델, 촬영 일시, ISO, 셔터 속도 등)를 함께 출력한다.
4. WHEN `--info` 옵션이 지정되고 입력이 디렉토리이면, THE Info_Reporter SHALL 디렉토리 내 모든 지원 이미지 파일의 정보를 출력한다.

### 요구사항 33: 이미지 품질 비교

**사용자 스토리:** 사용자로서, 변환 전후 이미지의 품질 차이를 객관적으로 비교하고 싶다. 이를 통해 최적의 품질 설정을 찾을 수 있다.

#### 인수 조건

1. WHEN `--compare` 옵션이 두 개의 이미지 파일 경로와 함께 지정되면, THE Image_Comparator SHALL image-compare 크레이트를 사용하여 두 이미지 간 SSIM 및 PSNR 메트릭을 계산하여 출력한다.
2. WHEN `--compare` 옵션이 지정되면, THE Image_Comparator SHALL 파일 시스템에 어떠한 변경도 수행하지 않는다.
3. WHEN `--compare` 옵션에 지정된 두 이미지의 크기가 다르면, THE CLI SHALL 이미지 크기가 동일해야 함을 안내하는 오류 메시지를 출력한다.
4. THE Image_Comparator SHALL pure Rust 크레이트(image-compare)만 사용하여 C 의존성 없이 빌드된다.

### 요구사항 34: 중복 파일 건너뛰기

**사용자 스토리:** 사용자로서, 이미 변환된 파일을 다시 변환하지 않고 건너뛰고 싶다. 이를 통해 반복 실행 시 불필요한 재변환을 방지하고 시간을 절약할 수 있다.

#### 인수 조건

1. WHEN `--skip-identical` 옵션이 지정되고 출력 파일이 이미 존재하면, THE Dedup_Checker SHALL sha2 크레이트를 사용하여 입력 파일과 출력 파일의 SHA-256 해시를 비교한다.
2. WHEN `--skip-identical` 옵션이 지정되고 입력 파일과 출력 파일의 해시가 동일하면, THE Dedup_Checker SHALL 해당 파일의 변환을 건너뛰고 건너뛴 사유를 출력한다.
3. WHEN `--skip-identical` 옵션이 지정되고 출력 파일이 존재하지 않으면, THE Converter SHALL 정상적으로 변환을 수행한다.
4. THE Dedup_Checker SHALL pure Rust 크레이트(sha2)만 사용하여 C 의존성 없이 빌드된다.

### 요구사항 35: 변환 프리셋

**사용자 스토리:** 사용자로서, 용도별 사전 정의된 변환 설정을 간편하게 적용하고 싶다. 이를 통해 복잡한 옵션 조합을 매번 입력하지 않고 한 번에 적용할 수 있다.

#### 인수 조건

1. WHEN `--preset web` 옵션이 지정되면, THE Preset_Manager SHALL WebP lossy 포맷, quality 80, 최대 너비 1920px 설정을 적용한다.
2. WHEN `--preset thumbnail` 옵션이 지정되면, THE Preset_Manager SHALL JPEG 포맷, quality 70, 너비 200px, 높이 200px, 종횡비 유지 설정을 적용한다.
3. WHEN `--preset print` 옵션이 지정되면, THE Preset_Manager SHALL TIFF 포맷, 300 DPI 설정을 적용한다.
4. WHEN `--preset social` 옵션이 지정되면, THE Preset_Manager SHALL JPEG 포맷, quality 85, 너비 1200px, 높이 630px 설정을 적용한다.
5. WHEN `--preset` 옵션과 개별 옵션(`--quality`, `--width` 등)이 동시에 지정되면, THE Preset_Manager SHALL 개별 옵션이 프리셋 설정을 덮어쓰도록 한다.
6. WHEN `--preset` 옵션에 지원되지 않는 프리셋 이름이 지정되면, THE CLI SHALL 지원되는 프리셋 목록(web, thumbnail, print, social)과 함께 오류 메시지를 출력한다.
