# 🦀 rust-imgconv

Pure Rust로 작성된 고성능 이미지 포맷 일괄 변환 CLI 도구. 20개 이상의 이미지 포맷 간 변환을 지원하며, 병렬 처리, 리사이즈, 크롭, 필터, 워터마크, EXIF 처리, 품질 비교, 프리셋 등 다양한 기능을 C 의존성 없는 단일 바이너리로 제공합니다.

## 주요 기능

- **20개 이상의 이미지 포맷** — JPEG, PNG, GIF, BMP, TIFF, TGA, ICO, QOI, PNM, OpenEXR, HDR, Farbfeld, WebP, SVG, AVIF, JPEG XL, DDS, PCX, Ultra HDR, APNG
- **일괄 변환** — 디렉토리 내 모든 이미지를 rayon 기반 병렬 처리로 한 번에 변환
- **다중 포맷 출력** — 하나의 입력 파일에서 여러 포맷을 동시에 생성 (`--to png,webp,jpeg`)
- **이미지 리사이즈** — 변환 시 유연한 종횡비 제어로 크기 조정
- **이미지 크롭** — `--crop x,y,w,h`로 특정 영역 잘라내기
- **회전 및 뒤집기** — 90/180/270° 회전, 수평/수직 뒤집기
- **색상 필터** — 흑백, 반전, 세피아 톤 효과
- **블러 및 샤프닝** — 가우시안 블러, 언샤프 마스크
- **밝기/대비/감마** — 이미지 노출 및 톤 세밀 조정
- **텍스트 워터마크** — 위치, 투명도, 커스텀 폰트 지원 텍스트 워터마크
- **이미지 오버레이** — 로고 등 이미지 합성 (위치, 투명도 제어)
- **EXIF 자동 방향 보정** — EXIF 데이터 기반 이미지 방향 자동 보정
- **EXIF 메타데이터 보존** — 원본 EXIF 데이터를 변환 결과에 복사 (JPEG→JPEG)
- **이미지 정보** — 변환 없이 이미지 메타데이터 출력 (`--info`)
- **품질 비교** — 두 이미지 간 SSIM/PSNR 메트릭 비교 (`--compare`)
- **중복 건너뛰기** — SHA-256 해시 비교로 동일 파일 건너뛰기 (`--skip-identical`)
- **변환 프리셋** — web, thumbnail, print, social 용도별 프리셋 (`--preset`)
- **품질 제어** — JPEG, WebP, AVIF 등 손실 압축 포맷의 품질 세밀 조정
- **SVG ↔ 래스터** — SVG 래스터화(resvg) 또는 래스터 이미지 SVG 트레이싱(vtracer)
- **애니메이션** — 애니메이션 GIF에서 프레임 추출, 또는 이미지들을 애니메이션 GIF로 조립
- **Dry-Run 모드** — 파일을 생성하지 않고 변환 계획만 미리 확인
- **진행률 표시** — indicatif 기반 실시간 진행률 바
- **Pure Rust** — C 컴파일러 불필요. Linux, macOS, Windows에서 빌드
- **라이브러리 + 바이너리** — CLI 도구 또는 라이브러리 크레이트로 프로젝트에 통합 가능

## 지원 포맷

| 포맷 | 확장자 | 읽기 | 쓰기 | 품질 설정 | Feature | 크레이트 |
| --- | --- | --- | --- | --- | --- | --- |
| JPEG | `.jpg`, `.jpeg` | ✓ | ✓ | ✓ (기본: 85) | — | image |
| PNG | `.png` | ✓ | ✓ | — | — | image |
| GIF | `.gif` | ✓ | ✓ | — | — | image |
| BMP | `.bmp` | ✓ | ✓ | — | — | image |
| TIFF | `.tif`, `.tiff` | ✓ | ✓ | — | — | image |
| TGA | `.tga` | ✓ | ✓ | — | — | image |
| ICO | `.ico` | ✓ | ✓ | — | — | image |
| QOI | `.qoi` | ✓ | ✓ | — | — | image |
| PNM | `.ppm`, `.pgm`, `.pbm`, `.pam` | ✓ | ✓ | — | — | image |
| OpenEXR | `.exr` | ✓ | ✓ | — | — | image |
| HDR | `.hdr` | ✓ | ✓ | — | — | image |
| Farbfeld | `.ff` | ✓ | ✓ | — | — | image |
| WebP | `.webp` | ✓ | ✓ | ✓ (lossy 기본: 75) | — | zenwebp |
| SVG | `.svg` | ✓ | ✓ | — | — | resvg / vtracer |
| AVIF | `.avif` | ✓ | ✓ | ✓ (기본: 70) | `avif` | ravif / rav1d |
| JPEG XL | `.jxl` | ✓ | ✗ | — | `jxl` | jxl-oxide |
| DDS | `.dds` | ✓ | ✗ | — | `dds` | image (dds) |
| PCX | `.pcx` | ✓ | ✓ | — | `pcx` | pcx |
| Ultra HDR | `.uhdr` | 스텁 | 스텁 | — | `ultrahdr` | — |
| APNG | `.apng` | ✓ | 스텁 | — | `apng` | image (png) |

> Feature-gated 포맷은 빌드 시 해당 feature flag를 활성화해야 합니다 (예: `cargo build --features avif,jxl,pcx`).

## 설치

### 소스에서 빌드

```bash
# 기본 빌드
cargo build --release

# 모든 선택적 포맷 포함 빌드
cargo build --release --features avif,jxl,dds,pcx,compare,dedup

# 특정 feature만 포함
cargo build --release --features "avif,compare"
```

컴파일된 바이너리는 `target/release/imgconv` (Windows: `imgconv.exe`)에 생성됩니다.

### 요구사항

- Rust 1.70+ (2021 에디션)
- C 컴파일러 불필요 — 모든 의존성이 pure Rust

## 사용법

```text
imgconv [옵션] --to <포맷> <입력경로>
```

### 기본 사용 예시

```bash
# 단일 파일 변환
imgconv photo.png --to jpeg
imgconv photo.jpg --to webp
imgconv photo.jpg --to webp --lossless

# 다중 포맷 동시 변환
imgconv photo.bmp --to jpeg,webp,png

# 디렉토리 일괄 변환
imgconv ./photos --to webp -o ./output

# 리사이즈
imgconv photo.png --to jpeg --width 800
imgconv photo.png --to jpeg --width 800 --height 600 --keep-aspect

# 품질 설정
imgconv photo.png --to jpeg --quality 95
imgconv photo.png --to webp --lossy --quality 60

# 크롭
imgconv photo.jpg --to png --crop 100,100,800,600

# 회전 및 뒤집기
imgconv photo.jpg --to png --rotate 90
imgconv photo.jpg --to png --flip horizontal

# 색상 필터
imgconv photo.jpg --to png --grayscale
imgconv photo.jpg --to png --sepia
imgconv photo.jpg --to png --invert

# 블러 및 샤프닝
imgconv photo.jpg --to png --blur 2.0
imgconv photo.jpg --to png --sharpen 1.5

# 밝기 / 대비 / 감마
imgconv photo.jpg --to png --brightness 20 --contrast 1.5
imgconv photo.jpg --to png --gamma 2.2

# 텍스트 워터마크
imgconv photo.jpg --to png --watermark "© 2026" --watermark-position bottom-right --watermark-opacity 0.5

# 이미지 오버레이
imgconv photo.jpg --to png --overlay logo.png --overlay-position center --overlay-opacity 0.7

# EXIF
imgconv photo.jpg --to jpeg --auto-orient
imgconv photo.jpg --to jpeg --preserve-exif

# 이미지 정보 (변환 없음)
imgconv photo.jpg --info

# 품질 비교 (--features compare 필요)
imgconv --compare original.jpg converted.webp

# 중복 파일 건너뛰기 (--features dedup 필요)
imgconv ./photos --to webp -o ./output --skip-identical

# 프리셋
imgconv photo.jpg --preset web
imgconv photo.jpg --preset thumbnail
imgconv photo.jpg --preset print
imgconv photo.jpg --preset social

# SVG
imgconv icon.svg --to png --dpi 300
imgconv photo.png --to svg --svg-preset poster

# 애니메이션
imgconv animated.gif --extract-frames --to png -o ./frames
imgconv ./frames --assemble-gif --frame-delay 50 -o ./output

# Dry-run 및 상세 출력
imgconv ./photos --to webp --dry-run
imgconv ./photos --to webp -o ./output --verbose
```

### CLI 옵션 레퍼런스

| 옵션 | 단축 | 설명 | 기본값 |
| --- | --- | --- | --- |
| `<INPUT>` | | 입력 파일 또는 디렉토리 경로 (필수) | |
| `--to <포맷>` | | 대상 포맷, 쉼표로 구분 (필수) | |
| `--output <디렉토리>` | `-o` | 출력 디렉토리 | 입력과 동일 |
| `--quality <1-100>` | | 손실 압축 포맷의 품질 | 포맷별 기본값 |
| `--width <PX>` | | 리사이즈 너비 (픽셀) | |
| `--height <PX>` | | 리사이즈 높이 (픽셀) | |
| `--keep-aspect` | | width + height 지정 시 종횡비 유지 | `false` |
| `--crop <x,y,w,h>` | | 크롭 영역 (x, y, 너비, 높이) | |
| `--rotate <각도>` | | 회전: 90, 180, 270 | |
| `--flip <방향>` | | 뒤집기: horizontal, vertical | |
| `--grayscale` | | 흑백 변환 | `false` |
| `--invert` | | 색상 반전 | `false` |
| `--sepia` | | 세피아 톤 | `false` |
| `--blur <SIGMA>` | | 가우시안 블러 (sigma 값) | |
| `--sharpen <값>` | | 언샤프 마스크 | |
| `--brightness <정수>` | | 밝기 조정 (+/-) | |
| `--contrast <실수>` | | 대비 조정 | |
| `--gamma <실수>` | | 감마 보정 | |
| `--watermark <텍스트>` | | 텍스트 워터마크 | |
| `--watermark-position` | | 워터마크 위치 | `bottom-right` |
| `--watermark-opacity` | | 워터마크 투명도 (0.0-1.0) | `0.5` |
| `--watermark-font` | | TrueType 폰트 파일 경로 | 내장 폰트 |
| `--overlay <파일>` | | 오버레이 이미지 파일 경로 | |
| `--overlay-position` | | 오버레이 위치 | `bottom-right` |
| `--overlay-opacity` | | 오버레이 투명도 (0.0-1.0) | `1.0` |
| `--auto-orient` | | EXIF 방향 자동 보정 | `false` |
| `--no-auto-orient` | | EXIF 방향 보정 비활성화 | |
| `--preserve-exif` | | EXIF 메타데이터 보존 | `false` |
| `--info` | | 이미지 정보 출력 (변환 없음) | `false` |
| `--compare <A> <B>` | | 두 이미지 비교 (SSIM/PSNR) | |
| `--skip-identical` | | 해시 동일 파일 건너뛰기 | `false` |
| `--preset <이름>` | | 변환 프리셋: web, thumbnail, print, social | |
| `--lossy` | | WebP lossy 인코딩 | WebP 기본값 |
| `--lossless` | | WebP lossless 인코딩 | |
| `--dpi <DPI>` | | SVG 래스터화 DPI | `96` |
| `--svg-preset <프리셋>` | | SVG 트레이싱 프리셋: bw, poster, photo | `default` |
| `--overwrite` | | 기존 출력 파일 덮어쓰기 | `false` |
| `--dry-run` | | 변환 계획만 출력 (실제 변환 안 함) | `false` |
| `--verbose` | | 파일별 상세 변환 정보 출력 | `false` |
| `--extract-frames` | | 애니메이션 GIF에서 프레임 추출 | `false` |
| `--assemble-gif` | | 이미지들을 애니메이션 GIF로 조립 | `false` |
| `--frame-delay <밀리초>` | | 프레임 딜레이 (밀리초) | `100` |
| `--help` | `-h` | 도움말 표시 | |

### 프리셋

| 프리셋 | 포맷 | 품질 | 너비 | 높이 | 종횡비 유지 | 비고 |
| --- | --- | --- | --- | --- | --- | --- |
| `web` | WebP (lossy) | 80 | 1920 | — | ✓ | 최대 너비 1920px |
| `thumbnail` | JPEG | 70 | 200 | 200 | ✓ | 정사각형 fit |
| `print` | TIFF | — | — | — | — | 300 DPI |
| `social` | JPEG | 85 | 1200 | 630 | ✗ | OG 이미지 크기 |

### 종료 코드

| 코드 | 의미 |
| --- | --- |
| `0` | 모든 변환 성공 |
| `1` | 일부 변환 실패 (부분 성공) |
| `2` | 모든 변환 실패 또는 인자 오류 |

## 라이브러리로 사용하기

```toml
[dependencies]
imgconv = { path = ".", default-features = false }
```

```rust
use std::path::Path;
use imgconv::convert::{convert_file, ConvertOptions};
use imgconv::format::ImageFormat;

fn main() -> Result<(), imgconv::error::ConvertError> {
    let options = ConvertOptions {
        target_formats: vec![ImageFormat::WebP, ImageFormat::Jpeg],
        quality: Some(80),
        output_dir: Some("./output".into()),
        overwrite: false,
        dry_run: false,
        verbose: false,
        ..Default::default()
    };

    let results = convert_file(Path::new("photo.png"), &options)?;
    for r in &results {
        println!("{} -> {} ({} bytes)",
            r.input_path.display(),
            r.output_path.display(),
            r.output_size,
        );
    }
    Ok(())
}
```

## 아키텍처

```text
imgconv/
├── src/
│   ├── lib.rs        # 라이브러리 진입점, 공개 API re-export
│   ├── main.rs       # CLI 바이너리 (clap + indicatif)
│   ├── format.rs     # ImageFormat 열거형, 확장자 감지
│   ├── convert.rs    # 변환 오케스트레이터 (단일 파일 파이프라인)
│   ├── batch.rs      # 배치 프로세서 (디렉토리, rayon 병렬 처리)
│   ├── raster.rs     # 래스터 코덱 (image 크레이트, 12개 포맷)
│   ├── webp.rs       # WebP 코덱 (zenwebp)
│   ├── svg.rs        # SVG 래스터라이저 (resvg) + 트레이서 (vtracer)
│   ├── avif.rs       # AVIF 코덱 (ravif + rav1d, feature-gated)
│   ├── jxl.rs        # JPEG XL 디코더 (jxl-oxide, feature-gated)
│   ├── dds.rs        # DDS 디코더 (image dds, feature-gated)
│   ├── pcx.rs        # PCX 코덱 (pcx 크레이트, feature-gated)
│   ├── ultrahdr.rs   # Ultra HDR 스텁 (feature-gated)
│   ├── apng.rs       # APNG 코덱 (feature-gated)
│   ├── resize.rs     # 이미지 리사이즈 (종횡비 제어)
│   ├── crop.rs       # 이미지 크롭
│   ├── transform.rs  # 회전 및 뒤집기
│   ├── filter.rs     # 색상 필터, 블러, 샤프닝, 밝기/대비/감마
│   ├── watermark.rs  # 텍스트 워터마크 (imageproc + ab_glyph)
│   ├── overlay.rs    # 이미지 오버레이 합성
│   ├── exif.rs       # EXIF 자동 방향 보정 및 보존
│   ├── info.rs       # 이미지 메타데이터 정보 출력
│   ├── compare.rs    # SSIM/PSNR 비교 (feature-gated)
│   ├── dedup.rs      # SHA-256 중복 검사 (feature-gated)
│   ├── preset.rs     # 변환 프리셋
│   ├── quality.rs    # 품질 기본값 및 유효성 검증
│   ├── animation.rs  # GIF 프레임 추출 / 조립
│   └── error.rs      # ConvertError 열거형 (thiserror)
```

### 변환 파이프라인

```text
입력 → 포맷 감지 → 디코딩 → [자동 방향 보정] → [크롭] → [리사이즈]
  → [회전/뒤집기] → [필터] → [밝기/대비/감마]
  → [블러/샤프닝] → [워터마크] → [오버레이] → 인코딩 → [EXIF 보존] → 출력
```

## 빌드 및 테스트

```bash
# 빌드
cargo build --release

# 모든 feature 포함 빌드
cargo build --release --features avif,jxl,dds,pcx,compare,dedup

# 테스트 실행
cargo test

# 모든 feature 포함 테스트
cargo test --features "pcx,compare,dedup"
```

## 테스트 결과

- 기본 테스트: 173 통과
- 전체 테스트 (pcx, compare, dedup feature 포함): 188 통과
- Property-Based 테스트: 28개 포함
- 실패: 0

## 라이선스

MIT
