# 31. WebP Lossy / Lossless

Compare WebP lossy and lossless encoding modes.

## Commands

```bash
# Lossless WebP
imgconv TestImg.jpg --to webp --lossless -o test_output/31_lossless_webp

# Lossy WebP (quality 80)
imgconv TestImg.jpg --to webp --lossy --quality 80 -o test_output/31_lossless_webp
```

## Output Files

| File | Size | Description |
|------|------|-------------|
| TestImg_lossless.webp | 28KB | Lossless encoding |
| TestImg_lossy_q80.webp | 10KB | Lossy encoding Q80 |
