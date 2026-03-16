# 17. 텍스트 워터마크 (Text Watermark)

텍스트 워터마크를 다양한 위치와 투명도로 적용하는 테스트.

## 테스트 명령어

```bash
# 기본 워터마크 (우하단, 투명도 0.5)
imgconv TestImg.jpg --to png --watermark "imgconv test" -o test_output/17_watermark

# 좌상단, 투명도 0.8
imgconv TestImg.jpg --to png --watermark "TOP LEFT" --watermark-position top-left --watermark-opacity 0.8 -o test_output/17_watermark

# 중앙, 투명도 1.0
imgconv TestImg.jpg --to png --watermark "CENTER" --watermark-position center --watermark-opacity 1.0 -o test_output/17_watermark
```

## 결과 파일

| 파일 | 설명 |
|------|------|
| TestImg_default.png | 우하단 워터마크 (기본) |
| TestImg_topleft.png | 좌상단 워터마크 |
| TestImg_center.png | 중앙 워터마크 |
