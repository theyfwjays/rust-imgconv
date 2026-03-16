# 11. 블러 (Gaussian Blur)

가우시안 블러 필터 테스트. sigma 값이 클수록 더 흐려짐.

## 테스트 명령어

```bash
# 약한 블러 (sigma=3.0)
imgconv TestImg.jpg --to png --blur 3.0 -o test_output/11_blur

# 강한 블러 (sigma=10.0)
imgconv TestImg.jpg --to png --blur 10.0 -o test_output/11_blur
```

## 결과 파일

| 파일 | 설명 |
|------|------|
| TestImg_blur3.png | sigma=3.0 블러 |
| TestImg_blur10.png | sigma=10.0 블러 |
