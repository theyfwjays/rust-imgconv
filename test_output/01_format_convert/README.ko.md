# 01. 포맷 변환 (Format Conversion)

JPG 이미지를 다양한 포맷으로 변환하는 테스트.

## 테스트 명령어

```bash
# JPG → PNG
imgconv TestImg.jpg --to png -o test_output/01_format_convert

# JPG → BMP
imgconv TestImg.jpg --to bmp -o test_output/01_format_convert

# JPG → GIF
imgconv TestImg.jpg --to gif -o test_output/01_format_convert

# JPG → TIFF
imgconv TestImg.jpg --to tiff -o test_output/01_format_convert

# JPG → TGA
imgconv TestImg.jpg --to tga -o test_output/01_format_convert

# JPG → QOI
imgconv TestImg.jpg --to qoi -o test_output/01_format_convert

# JPG → PPM
imgconv TestImg.jpg --to ppm -o test_output/01_format_convert

# JPG → WebP
imgconv TestImg.jpg --to webp -o test_output/01_format_convert

# JPG → Farbfeld
imgconv TestImg.jpg --to ff -o test_output/01_format_convert
```

## 결과 파일

| 파일 | 포맷 | 크기 |
|------|------|------|
| TestImg.png | PNG | 43KB |
| TestImg.bmp | BMP | 2.3MB |
| TestImg.gif | GIF | 59KB |
| TestImg.tiff | TIFF | 2.3MB |
| TestImg.tga | TGA | 468KB |
| TestImg.qoi | QOI | 180KB |
| TestImg.ppm | PPM | 2.3MB |
| TestImg.webp | WebP | 9.6KB |
| TestImg.ff | Farbfeld | 6.3MB |
