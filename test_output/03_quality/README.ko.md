# 03. 품질 설정 (Quality Control)

JPEG/WebP 품질 값에 따른 파일 크기 및 화질 차이 테스트.

## 테스트 명령어

```bash
# JPEG 품질 10 (저화질)
imgconv TestImg.jpg --to jpg --quality 10 -o test_output/03_quality

# JPEG 품질 50 (중간)
imgconv TestImg.jpg --to jpg --quality 50 -o test_output/03_quality

# JPEG 품질 95 (고화질)
imgconv TestImg.jpg --to jpg --quality 95 -o test_output/03_quality

# WebP 품질 30 (저화질)
imgconv TestImg.jpg --to webp --quality 30 -o test_output/03_quality

# WebP 품질 90 (고화질)
imgconv TestImg.jpg --to webp --quality 90 -o test_output/03_quality
```

## 결과 파일

| 파일 | 품질 | 크기 |
|------|------|------|
| TestImg_q10.jpg | JPEG Q10 | 27KB |
| TestImg_q50.jpg | JPEG Q50 | 36KB |
| TestImg_q95.jpg | JPEG Q95 | 63KB |
| TestImg_q30.webp | WebP Q30 | 7.9KB |
| TestImg_q90.webp | WebP Q90 | 12KB |
