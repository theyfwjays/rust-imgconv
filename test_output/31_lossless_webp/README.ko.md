# 31. WebP Lossy/Lossless

WebP의 손실/무손실 인코딩 비교 테스트.

## 테스트 명령어

```bash
# 무손실 WebP
imgconv TestImg.jpg --to webp --lossless -o test_output/31_lossless_webp

# 손실 WebP (품질 80)
imgconv TestImg.jpg --to webp --lossy --quality 80 -o test_output/31_lossless_webp
```

## 결과 파일

| 파일 | 크기 | 설명 |
|------|------|------|
| TestImg_lossless.webp | 28KB | 무손실 인코딩 |
| TestImg_lossy_q80.webp | 10KB | 손실 인코딩 Q80 |
