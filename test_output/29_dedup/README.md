# 29. Deduplication (Skip Identical)

Skip conversion when the output file is identical. Requires `--features dedup`.

## Commands

```bash
# First conversion
imgconv TestImg.jpg --to png -o test_output/29_dedup

# Re-run with --skip-identical (skips if output hash matches)
imgconv TestImg.jpg --to png -o test_output/29_dedup --overwrite --skip-identical --verbose
```

## Output Files

| File | Description |
|------|-------------|
| TestImg.png | Conversion result |
| dedup_output.txt | Dedup verbose log |
