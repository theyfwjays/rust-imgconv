# 25. Batch Conversion

Convert all images in a directory at once with parallel processing.

## Commands

```bash
imgconv test_output/25_batch/input --to png -o test_output/25_batch/output
```

## Input Files

| File | Description |
|------|-------------|
| input/img1.jpg | Test pattern 1024x768 |
| input/img2.jpg | Test pattern 640x480 |
| input/img3.jpg | SMPTE color bars 320x240 |

## Output Files

| File | Description |
|------|-------------|
| output/img1.png | img1 converted to PNG |
| output/img2.png | img2 converted to PNG |
| output/img3.png | img3 converted to PNG |
