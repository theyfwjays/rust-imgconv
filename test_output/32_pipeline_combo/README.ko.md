# 32. 파이프라인 콤보 (Full Pipeline Combo)

크롭 + 리사이즈 + 흑백 + 회전 + 품질 + 포맷변환을 한 번에 적용하는 테스트.

## 테스트 명령어

```bash
imgconv TestImg.jpg --to webp \
  --crop 50,50,500,400 \
  --width 300 \
  --grayscale \
  --rotate 90 \
  --quality 85 \
  -o test_output/32_pipeline_combo
```

## 파이프라인 순서

1. 크롭: (50,50)에서 500x400 영역 추출
2. 리사이즈: 너비 300px로 축소
3. 흑백 변환
4. 90° 회전
5. WebP Q85로 인코딩

## 결과 파일

| 파일 | 크기 | 설명 |
|------|------|------|
| TestImg.webp | 1.2KB | 모든 옵션 복합 적용 |
