# 12. 샤프닝 (Unsharpen Mask)

언샤프 마스크 필터 테스트. 값이 클수록 더 선명해짐.

## 테스트 명령어

```bash
# 약한 샤프닝 (5.0)
imgconv TestImg.jpg --to png --sharpen 5.0 -o test_output/12_sharpen

# 강한 샤프닝 (20.0)
imgconv TestImg.jpg --to png --sharpen 20.0 -o test_output/12_sharpen
```

## 결과 파일

| 파일 | 설명 |
|------|------|
| TestImg_sharpen5.png | sharpen=5.0 |
| TestImg_sharpen20.png | sharpen=20.0 |
