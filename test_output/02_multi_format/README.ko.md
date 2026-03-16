# 02. 멀티 포맷 변환 (Multi-Format Conversion)

하나의 입력 파일을 여러 포맷으로 동시에 변환하는 테스트.

## 테스트 명령어

```bash
# JPG → PNG, BMP, GIF 동시 변환
imgconv TestImg.jpg --to png,bmp,gif -o test_output/02_multi_format
```

## 결과 파일

| 파일 | 포맷 |
|------|------|
| TestImg.png | PNG |
| TestImg.bmp | BMP |
| TestImg.gif | GIF |
