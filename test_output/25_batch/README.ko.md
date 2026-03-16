# 25. 배치 변환 (Batch Conversion)

디렉토리 내 모든 이미지를 일괄 변환하는 테스트.

## 테스트 명령어

```bash
# 입력 디렉토리의 모든 JPG를 PNG로 일괄 변환
imgconv test_output/25_batch/input --to png -o test_output/25_batch/output
```

## 입력 파일

| 파일 | 설명 |
|------|------|
| input/img1.jpg | 테스트 패턴 1024x768 |
| input/img2.jpg | 테스트 패턴 640x480 |
| input/img3.jpg | SMPTE 컬러바 320x240 |

## 결과 파일

| 파일 | 설명 |
|------|------|
| output/img1.png | img1 PNG 변환 |
| output/img2.png | img2 PNG 변환 |
| output/img3.png | img3 PNG 변환 |
