# 16. 복합 필터 (Combined Filters)

여러 필터와 옵션을 동시에 적용하는 파이프라인 테스트.

## 테스트 명령어

```bash
# 흑백 + 밝기 +30 + 대비 +10 + 너비 500px
imgconv TestImg.jpg --to png --grayscale --brightness 30 --contrast 10.0 --width 500 -o test_output/16_combined_filters
```

## 결과 파일

| 파일 | 설명 |
|------|------|
| TestImg.png | 흑백 + 밝기 + 대비 + 리사이즈 복합 적용 |
