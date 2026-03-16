# 27. Overwrite

Test the --overwrite flag behavior when output files already exist.

## Commands

```bash
# First conversion
imgconv TestImg.jpg --to png -o test_output/27_overwrite

# Re-run without --overwrite (may error if file exists)
imgconv TestImg.jpg --to png -o test_output/27_overwrite

# Force overwrite with --overwrite flag
imgconv TestImg.jpg --to png -o test_output/27_overwrite --overwrite
```

## Output Files

| File | Description |
|------|-------------|
| TestImg.png | Overwrite test result |
