# 04. 리사이즈 (Resize)

다양한 리사이즈 옵션 테스트: 너비만, 높이만, 강제 스트레치, 종횡비 유지.

## 테스트 명령어

```bash
# 너비 200px (높이 자동 계산, 종횡비 유지)
imgconv TestImg.jpg --to png --width 200 -o test_output/04_resize

# 높이 100px (너비 자동 계산, 종횡비 유지)
imgconv TestImg.jpg --to png --height 100 -o test_output/04_resize

# 300x300 강제 스트레치 (종횡비 무시)
imgconv TestImg.jpg --to png --width 300 --height 300 -o test_output/04_resize

# 300x300 종횡비 유지 (fit)
imgconv TestImg.jpg --to png --width 300 --height 300 --keep-aspect -o test_output/04_resize
```

## 결과 파일

| 파일 | 설명 |
|------|------|
| TestImg_w200.png | 너비 200px, 종횡비 유지 |
| TestImg_h100.png | 높이 100px, 종횡비 유지 |
| TestImg_300x300_stretch.png | 300x300 강제 스트레치 |
| TestImg_300x300_fit.png | 300x300 박스 안에 종횡비 유지 |
