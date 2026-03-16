# 23. EXIF Metadata

Test EXIF preservation and auto-orientation correction.

## Commands

```bash
# Preserve EXIF metadata during conversion
imgconv TestImg.jpg --to png --preserve-exif -o test_output/23_exif

# Auto-correct orientation based on EXIF orientation tag
imgconv TestImg.jpg --to jpg --auto-orient -o test_output/23_exif
```

## Output Files

| File | Description |
|------|-------------|
| TestImg_preserve_exif.png | EXIF metadata preserved |
| TestImg_auto_orient.jpg | Auto-orientation corrected |
