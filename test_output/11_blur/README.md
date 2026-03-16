# 11. Gaussian Blur

Test Gaussian blur at different sigma values. Higher sigma = more blur.

## Commands

```bash
imgconv TestImg.jpg --to png --blur 3.0 -o test_output/11_blur    # Light blur
imgconv TestImg.jpg --to png --blur 10.0 -o test_output/11_blur   # Heavy blur
```

## Output Files

| File | Description |
|------|-------------|
| TestImg_blur3.png | sigma=3.0 blur |
| TestImg_blur10.png | sigma=10.0 blur |
