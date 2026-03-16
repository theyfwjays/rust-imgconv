# 18. 이미지 오버레이 (Image Overlay)

다른 이미지를 오버레이로 합성하는 테스트.

## 테스트 명령어

```bash
# 오버레이용 빨간 100x100 이미지 생성 (ffmpeg)
ffmpeg -y -f lavfi -i "color=c=red:s=100x100:d=1" -frames:v 1 overlay_red.png

# 기본 오버레이 (우하단, 투명도 1.0)
imgconv TestImg.jpg --to png --overlay overlay_red.png -o test_output/18_overlay

# 중앙, 투명도 0.5
imgconv TestImg.jpg --to png --overlay overlay_red.png --overlay-position center --overlay-opacity 0.5 -o test_output/18_overlay
```

## 결과 파일

| 파일 | 설명 |
|------|------|
| overlay_red.png | 오버레이 소스 (빨간 100x100) |
| TestImg_overlay_br.png | 우하단 오버레이 |
| TestImg_overlay_center_50.png | 중앙 오버레이 (투명도 50%) |
