# imgconv TODO

구현 완료 및 향후 추가 기능 체크리스트.

---

## 1. 지원 포맷

- [x] 래스터 포맷 (JPEG, PNG, GIF, BMP, TIFF, TGA, ICO, QOI, PNM, OpenEXR, HDR, Farbfeld)
- [x] WebP (lossy/lossless, zenwebp)
- [x] SVG (래스터화: resvg, 트레이싱: vtracer)
- [x] AVIF (feature: `avif`, ravif + rav1d)
- [x] JPEG XL 디코딩 (feature: `jxl`, jxl-oxide, 읽기 전용)
- [x] DDS 읽기 (feature: `dds`, image 크레이트 dds feature, 읽기 전용)
- [x] PCX (feature: `pcx`, pcx 크레이트, 읽기/쓰기)
- [x] Ultra HDR JPEG (feature: `ultrahdr`, 스텁 구현 — ultrahdr-core 크레이트 미존재)
- [x] APNG (feature: `apng`, 디코딩: image PNG 디코더, 인코딩: 스텁)
- [x] 애니메이션 GIF 프레임 추출/조립 (`--extract-frames`, `--assemble-gif`)

## 2. 이미지 처리 기능

- [x] 이미지 크롭 (`--crop x,y,w,h`)
- [x] 이미지 회전 (`--rotate 90/180/270`)
- [x] 이미지 뒤집기 (`--flip horizontal/vertical`)
- [x] 색상 필터 (`--grayscale`, `--invert`, `--sepia`)
- [x] 블러/샤프닝 (`--blur`, `--sharpen`)
- [x] 밝기/대비/감마 조정 (`--brightness`, `--contrast`, `--gamma`)
- [x] 텍스트 워터마크 (`--watermark`, imageproc + ab_glyph)
- [x] 이미지 오버레이 (`--overlay`, image 크레이트 overlay)
- [x] 자동 EXIF 방향 보정 (`--auto-orient`, kamadak-exif)

## 3. 메타데이터/분석

- [x] EXIF 메타데이터 보존 (`--preserve-exif`, kamadak-exif + img-parts)
- [x] 이미지 정보 출력 (`--info`)
- [x] 이미지 품질 비교 (`--compare`, feature: `compare`, image-compare)
- [x] 중복 파일 건너뛰기 (`--skip-identical`, feature: `dedup`, sha2)

## 4. 변환 프리셋

- [x] 용도별 프리셋 (`--preset web/thumbnail/print/social`)

## 5. 핵심 기능

- [x] 단일 파일 변환
- [x] 디렉토리 일괄(배치) 변환 (rayon 병렬 처리)
- [x] 다중 포맷 동시 변환 (`--to png,webp,jpeg`)
- [x] 이미지 리사이즈 (`--width`, `--height`, `--keep-aspect`)
- [x] 품질 설정 (`--quality 1-100`)
- [x] Dry-run 모드 (`--dry-run`)
- [x] 상세 로그 (`--verbose`)
- [x] 덮어쓰기 보호 / `--overwrite`
- [x] 진행률 바 (indicatif)
- [x] 종료 코드 (0/1/2)
- [x] 라이브러리 + 바이너리 분리

---

## 6. 향후 추가 가능 기능 (미구현)

- [ ] HEIC/HEIF 지원 (pure Rust 구현 없음, C 바인딩 검토 필요)

---

## 테스트 현황

- 기본 테스트: 173 통과
- 전체 테스트 (pcx, compare, dedup feature 포함): 188 통과
- Property-Based 테스트: 28개 포함
- 실패: 0
