# imgconv CLI Integration Test Results

End-to-end test results for every feature, executed via the actual CLI binary.
Each folder contains output files and a README.md with the exact CLI commands used.

Input image: `TestImg.jpg` (1024x768, ffmpeg testsrc pattern, 40KB)

## Test List

| # | Folder | Feature | Status |
|---|--------|---------|--------|
| 01 | 01_format_convert | Format conversion (9 formats) | ✅ |
| 02 | 02_multi_format | Multi-format simultaneous conversion | ✅ |
| 03 | 03_quality | Quality control (JPEG/WebP) | ✅ |
| 04 | 04_resize | Resize (width/height/stretch/fit) | ✅ |
| 05 | 05_crop | Crop | ✅ |
| 06 | 06_rotate | Rotation (90/180/270) | ✅ |
| 07 | 07_flip | Flip (horizontal/vertical) | ✅ |
| 08 | 08_grayscale | Grayscale conversion | ✅ |
| 09 | 09_invert | Color inversion | ✅ |
| 10 | 10_sepia | Sepia tone | ✅ |
| 11 | 11_blur | Gaussian blur | ✅ |
| 12 | 12_sharpen | Sharpening | ✅ |
| 13 | 13_brightness | Brightness adjustment | ✅ |
| 14 | 14_contrast | Contrast adjustment | ✅ |
| 15 | 15_gamma | Gamma correction | ✅ |
| 16 | 16_combined_filters | Combined filters | ✅ |
| 17 | 17_watermark | Text watermark | ✅ |
| 18 | 18_overlay | Image overlay | ✅ |
| 19 | 19_preset_web | Preset — Web | ✅ |
| 20 | 20_preset_thumbnail | Preset — Thumbnail | ✅ |
| 21 | 21_preset_print | Preset — Print | ✅ |
| 22 | 22_preset_social | Preset — Social | ✅ |
| 23 | 23_exif | EXIF metadata | ✅ |
| 24 | 24_info | Image info output | ✅ |
| 25 | 25_batch | Batch conversion | ✅ |
| 26 | 26_dry_run | Dry run | ✅ |
| 27 | 27_overwrite | Overwrite | ✅ |
| 28 | 28_compare | Image comparison (SSIM/PSNR) | ✅ |
| 29 | 29_dedup | Deduplication (skip identical) | ✅ |
| 30 | 30_svg_trace | SVG tracing | ✅ |
| 31 | 31_lossless_webp | WebP lossy/lossless | ✅ |
| 32 | 32_pipeline_combo | Full pipeline combo | ✅ |
| 33 | 33_animation | Animation (frame extract / GIF assemble) | ✅ |

## Build Command

```bash
# Build with all features
cargo build --features "pcx,compare,dedup"
```

## Notes

- Feature-gated features (compare, dedup, pcx) require `--features` flag at build time
- Additional codecs (avif, jxl) can be enabled with their respective feature flags
