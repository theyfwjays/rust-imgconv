# 33. Animation (Frame Extract / GIF Assemble)

Extract individual frames from an animated GIF, and reassemble frames into an animated GIF.

## Commands

```bash
# Extract frames from animated GIF → individual PNGs
imgconv test_anim.gif --extract-frames --to png -o frames/

# Assemble frames back into animated GIF (200ms delay per frame)
imgconv frames/ --assemble-gif -o reassembled.gif --frame-delay 200
```

## Output Files

| File | Description |
|------|-------------|
| test_anim.gif | Source animated GIF (5 frames, 200x150) |
| frames/test_anim_0000.png | Frame 0 |
| frames/test_anim_0001.png | Frame 1 |
| frames/test_anim_0002.png | Frame 2 |
| frames/test_anim_0003.png | Frame 3 |
| frames/test_anim_0004.png | Frame 4 |
| reassembled.gif | Reassembled animated GIF from extracted frames |
