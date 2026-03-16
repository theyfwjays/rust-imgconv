# 07. 뒤집기 (Flip)

수평/수직 뒤집기 테스트.

## 테스트 명령어

```bash
# 수평 뒤집기 (좌우 반전)
imgconv TestImg.jpg --to png --flip horizontal -o test_output/07_flip

# 수직 뒤집기 (상하 반전)
imgconv TestImg.jpg --to png --flip vertical -o test_output/07_flip
```

## 결과 파일

| 파일 | 설명 |
|------|------|
| TestImg_horizontal.png | 좌우 반전 |
| TestImg_vertical.png | 상하 반전 |
