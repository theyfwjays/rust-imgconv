# 05. 크롭 (Crop)

이미지의 특정 영역을 잘라내는 테스트.

## 테스트 명령어

```bash
# (100,100) 위치에서 400x300 영역 크롭
imgconv TestImg.jpg --to png --crop 100,100,400,300 -o test_output/05_crop
```

## 결과 파일

| 파일 | 설명 |
|------|------|
| TestImg.png | 400x300 크롭 결과 |
