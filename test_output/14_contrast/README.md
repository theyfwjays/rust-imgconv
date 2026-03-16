# 14. Contrast Adjustment

Adjust image contrast. Positive = more contrast, negative = less.

## Commands

```bash
imgconv TestImg.jpg --to png --contrast 30.0 -o test_output/14_contrast    # More contrast
imgconv TestImg.jpg --to png --contrast -20.0 -o test_output/14_contrast   # Less contrast
```

## Output Files

| File | Description |
|------|-------------|
| TestImg_contrast30.png | Contrast +30 |
| TestImg_contrast_neg20.png | Contrast -20 |
