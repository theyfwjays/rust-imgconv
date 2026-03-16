# 23. EXIF 메타데이터 (EXIF Metadata)

EXIF 보존 및 자동 방향 보정 테스트.

## 테스트 명령어

```bash
# EXIF 메타데이터 보존하며 변환
imgconv TestImg.jpg --to png --preserve-exif -o test_output/23_exif

# EXIF 방향 태그 기반 자동 회전 보정
imgconv TestImg.jpg --to jpg --auto-orient -o test_output/23_exif
```

## 결과 파일

| 파일 | 설명 |
|------|------|
| TestImg_preserve_exif.png | EXIF 보존 변환 |
| TestImg_auto_orient.jpg | 자동 방향 보정 |
