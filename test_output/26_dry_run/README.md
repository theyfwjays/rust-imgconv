# 26. Dry Run

Preview the conversion plan without writing any files.

## Commands

```bash
imgconv TestImg.jpg --to png,webp,bmp --dry-run
```

## Output (dry_run_output.txt)

```
[dry-run] TestImg.jpg (jpg) → TestImg.png (png)
[dry-run] TestImg.jpg (jpg) → TestImg.webp (webp)
  Quality: 75
[dry-run] TestImg.jpg (jpg) → TestImg.bmp (bmp)
```
