# 12. Sharpening (Unsharpen Mask)

Test unsharp mask at different values. Higher value = sharper.

## Commands

```bash
imgconv TestImg.jpg --to png --sharpen 5.0 -o test_output/12_sharpen
imgconv TestImg.jpg --to png --sharpen 20.0 -o test_output/12_sharpen
```

## Output Files

| File | Description |
|------|-------------|
| TestImg_sharpen5.png | sharpen=5.0 |
| TestImg_sharpen20.png | sharpen=20.0 |
