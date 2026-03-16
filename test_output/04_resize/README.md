# 04. Resize

Test various resize options: width-only, height-only, forced stretch, and aspect-ratio fit.

## Commands

```bash
imgconv TestImg.jpg --to png --width 200 -o test_output/04_resize
imgconv TestImg.jpg --to png --height 100 -o test_output/04_resize
imgconv TestImg.jpg --to png --width 300 --height 300 -o test_output/04_resize
imgconv TestImg.jpg --to png --width 300 --height 300 --keep-aspect -o test_output/04_resize
```

## Output Files

| File | Description |
|------|-------------|
| TestImg_w200.png | Width 200px, aspect ratio preserved |
| TestImg_h100.png | Height 100px, aspect ratio preserved |
| TestImg_300x300_stretch.png | 300x300 forced stretch |
| TestImg_300x300_fit.png | 300x300 box fit, aspect ratio preserved |
