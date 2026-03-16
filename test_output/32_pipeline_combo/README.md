# 32. Full Pipeline Combo

Apply crop + resize + grayscale + rotation + quality + format conversion all at once.

## Commands

```bash
imgconv TestImg.jpg --to webp \
  --crop 50,50,500,400 \
  --width 300 \
  --grayscale \
  --rotate 90 \
  --quality 85 \
  -o test_output/32_pipeline_combo
```

## Pipeline Order

1. Crop: extract 500x400 region from (50,50)
2. Resize: scale to 300px width
3. Grayscale conversion
4. 90° rotation
5. Encode as WebP Q85

## Output Files

| File | Size | Description |
|------|------|-------------|
| TestImg.webp | 1.2KB | All options applied |
