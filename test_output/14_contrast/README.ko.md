# 14. 대비 조정 (Contrast)

대비를 높이거나 낮추는 테스트.

## 테스트 명령어

```bash
# 대비 증가 (+30)
imgconv TestImg.jpg --to png --contrast 30.0 -o test_output/14_contrast

# 대비 감소 (-20)
imgconv TestImg.jpg --to png --contrast -20.0 -o test_output/14_contrast
```

## 결과 파일

| 파일 | 설명 |
|------|------|
| TestImg_contrast30.png | 대비 +30 |
| TestImg_contrast_neg20.png | 대비 -20 |
