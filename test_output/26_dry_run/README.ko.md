# 26. 드라이런 (Dry Run)

실제 변환 없이 변환 계획만 출력하는 테스트.

## 테스트 명령어

```bash
imgconv TestImg.jpg --to png,webp,bmp --dry-run
```

## 출력 결과 (dry_run_output.txt)

```
[dry-run] TestImg.jpg (jpg) → TestImg.png (png)
[dry-run] TestImg.jpg (jpg) → TestImg.webp (webp)
  품질: 75
[dry-run] TestImg.jpg (jpg) → TestImg.bmp (bmp)
```
