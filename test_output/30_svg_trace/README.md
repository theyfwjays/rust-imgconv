# 30. SVG Tracing

Convert raster images to SVG vector format using various tracing presets.

## Commands

```bash
# Default preset (photo)
imgconv TestImg.jpg --to svg -o test_output/30_svg_trace

# Poster preset
imgconv TestImg.jpg --to svg --svg-preset poster -o test_output/30_svg_trace

# Black & white preset
imgconv TestImg.jpg --to svg --svg-preset bw -o test_output/30_svg_trace
```

## Output Files

| File | Description |
|------|-------------|
| TestImg_default.svg | Default (photo) preset |
| TestImg_poster.svg | Poster preset |
| TestImg_bw.svg | Black & white preset |
