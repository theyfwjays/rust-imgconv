# 30. SVG 트레이싱 (SVG Trace)

래스터 이미지를 SVG 벡터로 변환하는 테스트. 다양한 프리셋 지원.

## 테스트 명령어

```bash
# 기본 프리셋 (photo)
imgconv TestImg.jpg --to svg -o test_output/30_svg_trace

# poster 프리셋
imgconv TestImg.jpg --to svg --svg-preset poster -o test_output/30_svg_trace

# bw (흑백) 프리셋
imgconv TestImg.jpg --to svg --svg-preset bw -o test_output/30_svg_trace
```

## 결과 파일

| 파일 | 설명 |
|------|------|
| TestImg_default.svg | 기본(photo) 프리셋 |
| TestImg_poster.svg | poster 프리셋 |
| TestImg_bw.svg | bw(흑백) 프리셋 |
