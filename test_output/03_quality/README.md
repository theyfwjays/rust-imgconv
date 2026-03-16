# 03. Quality Control

Test file size and quality differences across JPEG/WebP quality levels.

## Commands

```bash
imgconv TestImg.jpg --to jpg --quality 10 -o test_output/03_quality   # Low quality
imgconv TestImg.jpg --to jpg --quality 50 -o test_output/03_quality   # Medium
imgconv TestImg.jpg --to jpg --quality 95 -o test_output/03_quality   # High quality
imgconv TestImg.jpg --to webp --quality 30 -o test_output/03_quality  # WebP low
imgconv TestImg.jpg --to webp --quality 90 -o test_output/03_quality  # WebP high
```

## Output Files

| File | Quality | Size |
|------|---------|------|
| TestImg_q10.jpg | JPEG Q10 | 27KB |
| TestImg_q50.jpg | JPEG Q50 | 36KB |
| TestImg_q95.jpg | JPEG Q95 | 63KB |
| TestImg_q30.webp | WebP Q30 | 7.9KB |
| TestImg_q90.webp | WebP Q90 | 12KB |
