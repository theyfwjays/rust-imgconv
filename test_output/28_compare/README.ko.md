# 28. 이미지 비교 (Image Compare)

두 이미지의 SSIM/PSNR 품질 비교 테스트. (--features compare 필요)

## 테스트 명령어

```bash
# 원본 PNG 생성
imgconv TestImg.jpg --to png -o test_output/28_compare

# 블러 적용 PNG 생성
imgconv TestImg.jpg --to png --blur 5.0 -o test_output/28_compare

# 두 이미지 비교
imgconv TestImg.jpg --compare TestImg_original.png TestImg_blurred.png
```

## 비교 결과 (compare_output.txt)

```
SSIM: 0.705429
PSNR: 20.61 dB
```

SSIM 1.0 = 동일, 낮을수록 차이가 큼. PSNR이 높을수록 유사.

## 결과 파일

| 파일 | 설명 |
|------|------|
| TestImg_original.png | 원본 PNG |
| TestImg_blurred.png | 블러 적용 PNG |
| compare_output.txt | SSIM/PSNR 비교 결과 |
