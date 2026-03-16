# 15. Gamma Correction

Adjust gamma. Values <1.0 darken, values >1.0 brighten.

## Commands

```bash
imgconv TestImg.jpg --to png --gamma 0.5 -o test_output/15_gamma   # Darker
imgconv TestImg.jpg --to png --gamma 2.0 -o test_output/15_gamma   # Brighter
```

## Output Files

| File | Description |
|------|-------------|
| TestImg_gamma05.png | Gamma 0.5 |
| TestImg_gamma20.png | Gamma 2.0 |
