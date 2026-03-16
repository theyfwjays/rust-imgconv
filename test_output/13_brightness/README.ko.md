# 13. 밝기 조정 (Brightness)

밝기를 높이거나 낮추는 테스트. 양수=밝게, 음수=어둡게.

## 테스트 명령어

```bash
# 밝게 (+50)
imgconv TestImg.jpg --to png --brightness 50 -o test_output/13_brightness

# 어둡게 (-50)
imgconv TestImg.jpg --to png --brightness -50 -o test_output/13_brightness
```

## 결과 파일

| 파일 | 설명 |
|------|------|
| TestImg_bright50.png | 밝기 +50 |
| TestImg_dark50.png | 밝기 -50 |
