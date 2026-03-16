# 15. 감마 보정 (Gamma Correction)

감마 값 조정 테스트. <1.0=어둡게, >1.0=밝게.

## 테스트 명령어

```bash
# 감마 0.5 (어둡게)
imgconv TestImg.jpg --to png --gamma 0.5 -o test_output/15_gamma

# 감마 2.0 (밝게)
imgconv TestImg.jpg --to png --gamma 2.0 -o test_output/15_gamma
```

## 결과 파일

| 파일 | 설명 |
|------|------|
| TestImg_gamma05.png | 감마 0.5 |
| TestImg_gamma20.png | 감마 2.0 |
