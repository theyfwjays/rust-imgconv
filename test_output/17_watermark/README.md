# 17. Text Watermark

Add text watermarks with various positions and opacity levels.

## Commands

```bash
# Default watermark (bottom-right, opacity 0.5)
imgconv TestImg.jpg --to png --watermark "imgconv test" -o test_output/17_watermark

# Top-left, opacity 0.8
imgconv TestImg.jpg --to png --watermark "TOP LEFT" --watermark-position top-left --watermark-opacity 0.8 -o test_output/17_watermark

# Center, opacity 1.0
imgconv TestImg.jpg --to png --watermark "CENTER" --watermark-position center --watermark-opacity 1.0 -o test_output/17_watermark
```

## Output Files

| File | Description |
|------|-------------|
| TestImg_default.png | Bottom-right watermark (default) |
| TestImg_topleft.png | Top-left watermark |
| TestImg_center.png | Center watermark |
