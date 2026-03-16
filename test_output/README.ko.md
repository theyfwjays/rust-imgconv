# imgconv CLI 통합 테스트 결과

모든 기능을 실제 CLI로 실행한 테스트 결과물입니다.
각 폴더에 결과 파일과 사용한 명령어가 README.md로 포함되어 있습니다.

입력 이미지: `TestImg.jpg` (1024x768, ffmpeg testsrc 패턴, 40KB)

## 테스트 목록

| # | 폴더 | 기능 | 상태 |
|---|------|------|------|
| 01 | 01_format_convert | 포맷 변환 (9개 포맷) | ✅ |
| 02 | 02_multi_format | 멀티 포맷 동시 변환 | ✅ |
| 03 | 03_quality | 품질 설정 (JPEG/WebP) | ✅ |
| 04 | 04_resize | 리사이즈 (너비/높이/스트레치/fit) | ✅ |
| 05 | 05_crop | 크롭 | ✅ |
| 06 | 06_rotate | 회전 (90/180/270) | ✅ |
| 07 | 07_flip | 뒤집기 (수평/수직) | ✅ |
| 08 | 08_grayscale | 흑백 변환 | ✅ |
| 09 | 09_invert | 색상 반전 | ✅ |
| 10 | 10_sepia | 세피아 톤 | ✅ |
| 11 | 11_blur | 가우시안 블러 | ✅ |
| 12 | 12_sharpen | 샤프닝 | ✅ |
| 13 | 13_brightness | 밝기 조정 | ✅ |
| 14 | 14_contrast | 대비 조정 | ✅ |
| 15 | 15_gamma | 감마 보정 | ✅ |
| 16 | 16_combined_filters | 복합 필터 | ✅ |
| 17 | 17_watermark | 텍스트 워터마크 | ✅ |
| 18 | 18_overlay | 이미지 오버레이 | ✅ |
| 19 | 19_preset_web | 프리셋 - Web | ✅ |
| 20 | 20_preset_thumbnail | 프리셋 - Thumbnail | ✅ |
| 21 | 21_preset_print | 프리셋 - Print | ✅ |
| 22 | 22_preset_social | 프리셋 - Social | ✅ |
| 23 | 23_exif | EXIF 메타데이터 | ✅ |
| 24 | 24_info | 이미지 정보 출력 | ✅ |
| 25 | 25_batch | 배치 변환 | ✅ |
| 26 | 26_dry_run | 드라이런 | ✅ |
| 27 | 27_overwrite | 덮어쓰기 | ✅ |
| 28 | 28_compare | 이미지 비교 (SSIM/PSNR) | ✅ |
| 29 | 29_dedup | 중복 건너뛰기 | ✅ |
| 30 | 30_svg_trace | SVG 트레이싱 | ✅ |
| 31 | 31_lossless_webp | WebP Lossy/Lossless | ✅ |
| 32 | 32_pipeline_combo | 파이프라인 콤보 | ✅ |
| 33 | 33_animation | 애니메이션 (프레임 추출 / GIF 조립) | ✅ |

## 빌드 명령어

```bash
# 전체 기능 빌드
cargo build --features "pcx,compare,dedup"
```

## 참고

- feature-gated 기능 (compare, dedup, pcx)은 `--features` 플래그로 빌드 필요
- avif, jxl 등 추가 코덱은 별도 feature 플래그로 활성화
