# 29. 중복 건너뛰기 (Dedup / Skip Identical)

동일한 변환 결과가 이미 존재할 때 건너뛰는 테스트. (--features dedup 필요)

## 테스트 명령어

```bash
# 첫 번째 변환
imgconv TestImg.jpg --to png -o test_output/29_dedup

# 동일 변환 재실행 (--skip-identical로 중복 건너뛰기)
imgconv TestImg.jpg --to png -o test_output/29_dedup --overwrite --skip-identical --verbose
```

## 결과 파일

| 파일 | 설명 |
|------|------|
| TestImg.png | 변환 결과 |
| dedup_output.txt | 중복 건너뛰기 로그 |
