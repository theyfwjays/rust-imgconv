# 18. Image Overlay

Composite another image as an overlay with position and opacity control.

## Commands

```bash
# Generate a red 100x100 overlay image (ffmpeg)
ffmpeg -y -f lavfi -i "color=c=red:s=100x100:d=1" -frames:v 1 overlay_red.png

# Default overlay (bottom-right, opacity 1.0)
imgconv TestImg.jpg --to png --overlay overlay_red.png -o test_output/18_overlay

# Center, opacity 0.5
imgconv TestImg.jpg --to png --overlay overlay_red.png --overlay-position center --overlay-opacity 0.5 -o test_output/18_overlay
```

## Output Files

| File | Description |
|------|-------------|
| overlay_red.png | Overlay source (red 100x100) |
| TestImg_overlay_br.png | Bottom-right overlay |
| TestImg_overlay_center_50.png | Center overlay (50% opacity) |
