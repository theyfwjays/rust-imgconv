# 28. Image Comparison (SSIM/PSNR)

Compare two images using SSIM and PSNR quality metrics. Requires `--features compare`.

## Commands

```bash
# Generate original PNG
imgconv TestImg.jpg --to png -o test_output/28_compare

# Generate blurred PNG
imgconv TestImg.jpg --to png --blur 5.0 -o test_output/28_compare

# Compare original vs blurred
imgconv TestImg.jpg --compare TestImg_original.png TestImg_blurred.png
```

## Comparison Result (compare_output.txt)

```
SSIM: 0.705429
PSNR: 20.61 dB
```

SSIM 1.0 = identical. Lower = more different. Higher PSNR = more similar.

## Output Files

| File | Description |
|------|-------------|
| TestImg_original.png | Original PNG |
| TestImg_blurred.png | Blurred PNG |
| compare_output.txt | SSIM/PSNR comparison result |
