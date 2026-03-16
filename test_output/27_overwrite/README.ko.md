# 27. 덮어쓰기 (Overwrite)

기존 파일이 있을 때 --overwrite 옵션 동작 테스트.

## 테스트 명령어

```bash
# 첫 번째 변환
imgconv TestImg.jpg --to png -o test_output/27_overwrite

# 덮어쓰기 없이 재실행 (에러 발생 가능)
imgconv TestImg.jpg --to png -o test_output/27_overwrite

# --overwrite 옵션으로 강제 덮어쓰기
imgconv TestImg.jpg --to png -o test_output/27_overwrite --overwrite
```

## 결과 파일

| 파일 | 설명 |
|------|------|
| TestImg.png | 덮어쓰기 테스트 결과 |
