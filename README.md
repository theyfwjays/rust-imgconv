# 🦀 rust-imgconv

A high-performance image format batch conversion CLI tool written in pure Rust. Convert between 20+ image formats with parallel processing, resizing, cropping, filters, watermarks, EXIF handling, quality comparison, presets, and more — all from a single binary with zero C dependencies.

## Features

- **20+ Image Formats** — JPEG, PNG, GIF, BMP, TIFF, TGA, ICO, QOI, PNM, OpenEXR, HDR, Farbfeld, WebP, SVG, AVIF, JPEG XL, DDS, PCX, Ultra HDR, APNG
- **Batch Conversion** — Convert all images in a directory at once with parallel processing (rayon)
- **Multi-Format Output** — Convert a single input to multiple formats simultaneously (`--to png,webp,jpeg`)
- **Image Resizing** — Resize during conversion with flexible aspect ratio controls
- **Image Cropping** — Crop specific regions with `--crop x,y,w,h`
- **Rotation & Flip** — Rotate 90/180/270° and flip horizontal/vertical
- **Color Filters** — Grayscale, invert, sepia tone effects
- **Blur & Sharpen** — Gaussian blur and unsharp mask
- **Brightness/Contrast/Gamma** — Fine-tune image exposure and tone
- **Text Watermark** — Add text watermarks with position, opacity, and custom font support
- **Image Overlay** — Composite logo or image overlays with position and opacity control
- **EXIF Auto-Orient** — Automatically correct image orientation from EXIF data
- **EXIF Preservation** — Copy EXIF metadata from source to output (JPEG→JPEG)
- **Image Info** — Display image metadata without conversion (`--info`)
- **Quality Comparison** — SSIM/PSNR metrics between two images (`--compare`)
- **Deduplication** — Skip identical files via SHA-256 hash comparison (`--skip-identical`)
- **Conversion Presets** — Built-in presets for web, thumbnail, print, social (`--preset`)
- **Quality Control** — Fine-tune lossy compression quality for JPEG, WebP, and AVIF
- **SVG ↔ Raster** — Rasterize SVG (resvg) or trace raster images to SVG (vtracer)
- **Animation** — Extract frames from animated GIF, or assemble images into animated GIF
- **Dry-Run Mode** — Preview conversion plans without writing any files
- **Progress Bar** — Real-time progress display for batch operations (indicatif)
- **Pure Rust** — No C compiler required. Builds on Linux, macOS, and Windows
- **Library + Binary** — Use as a CLI tool or integrate the library crate into your project

## Supported Formats

| Format | Extensions | Read | Write | Quality | Feature | Crate |
| --- | --- | --- | --- | --- | --- | --- |
| JPEG | `.jpg`, `.jpeg` | ✓ | ✓ | ✓ (default: 85) | — | image |
| PNG | `.png` | ✓ | ✓ | — | — | image |
| GIF | `.gif` | ✓ | ✓ | — | — | image |
| BMP | `.bmp` | ✓ | ✓ | — | — | image |
| TIFF | `.tif`, `.tiff` | ✓ | ✓ | — | — | image |
| TGA | `.tga` | ✓ | ✓ | — | — | image |
| ICO | `.ico` | ✓ | ✓ | — | — | image |
| QOI | `.qoi` | ✓ | ✓ | — | — | image |
| PNM | `.ppm`, `.pgm`, `.pbm`, `.pam` | ✓ | ✓ | — | — | image |
| OpenEXR | `.exr` | ✓ | ✓ | — | — | image |
| HDR | `.hdr` | ✓ | ✓ | — | — | image |
| Farbfeld | `.ff` | ✓ | ✓ | — | — | image |
| WebP | `.webp` | ✓ | ✓ | ✓ (lossy default: 75) | — | zenwebp |
| SVG | `.svg` | ✓ | ✓ | — | — | resvg / vtracer |
| AVIF | `.avif` | ✓ | ✓ | ✓ (default: 70) | `avif` | ravif / rav1d |
| JPEG XL | `.jxl` | ✓ | ✗ | — | `jxl` | jxl-oxide |
| DDS | `.dds` | ✓ | ✗ | — | `dds` | image (dds) |
| PCX | `.pcx` | ✓ | ✓ | — | `pcx` | pcx |
| Ultra HDR | `.uhdr` | stub | stub | — | `ultrahdr` | — |
| APNG | `.apng` | ✓ | stub | — | `apng` | image (png) |

> Feature-gated formats require the corresponding feature flag at build time (e.g., `cargo build --features avif,jxl,pcx`).

## Installation

### From Source

```bash
# Standard build
cargo build --release

# With all optional formats
cargo build --release --features avif,jxl,dds,pcx,compare,dedup

# With specific features
cargo build --release --features "avif,compare"
```

The compiled binary will be at `target/release/imgconv` (or `imgconv.exe` on Windows).

### Requirements

- Rust 1.70+ (2021 edition)
- No C compiler needed — all dependencies are pure Rust

## Usage

```text
imgconv [OPTIONS] --to <FORMAT> <INPUT>
```

### Basic Examples

```bash
# Single file conversion
imgconv photo.png --to jpeg
imgconv photo.jpg --to webp
imgconv photo.jpg --to webp --lossless

# Multiple formats at once
imgconv photo.bmp --to jpeg,webp,png

# Batch convert a directory
imgconv ./photos --to webp -o ./output

# Resize
imgconv photo.png --to jpeg --width 800
imgconv photo.png --to jpeg --width 800 --height 600 --keep-aspect

# Quality control
imgconv photo.png --to jpeg --quality 95
imgconv photo.png --to webp --lossy --quality 60

# Crop
imgconv photo.jpg --to png --crop 100,100,800,600

# Rotate & Flip
imgconv photo.jpg --to png --rotate 90
imgconv photo.jpg --to png --flip horizontal

# Color filters
imgconv photo.jpg --to png --grayscale
imgconv photo.jpg --to png --sepia
imgconv photo.jpg --to png --invert

# Blur & Sharpen
imgconv photo.jpg --to png --blur 2.0
imgconv photo.jpg --to png --sharpen 1.5

# Brightness / Contrast / Gamma
imgconv photo.jpg --to png --brightness 20 --contrast 1.5
imgconv photo.jpg --to png --gamma 2.2

# Text watermark
imgconv photo.jpg --to png --watermark "© 2026" --watermark-position bottom-right --watermark-opacity 0.5

# Image overlay
imgconv photo.jpg --to png --overlay logo.png --overlay-position center --overlay-opacity 0.7

# EXIF
imgconv photo.jpg --to jpeg --auto-orient
imgconv photo.jpg --to jpeg --preserve-exif

# Image info (no conversion)
imgconv photo.jpg --info

# Quality comparison (requires --features compare)
imgconv --compare original.jpg converted.webp

# Skip identical files (requires --features dedup)
imgconv ./photos --to webp -o ./output --skip-identical

# Presets
imgconv photo.jpg --preset web
imgconv photo.jpg --preset thumbnail
imgconv photo.jpg --preset print
imgconv photo.jpg --preset social

# SVG
imgconv icon.svg --to png --dpi 300
imgconv photo.png --to svg --svg-preset poster

# Animation
imgconv animated.gif --extract-frames --to png -o ./frames
imgconv ./frames --assemble-gif --frame-delay 50 -o ./output

# Dry-run & Verbose
imgconv ./photos --to webp --dry-run
imgconv ./photos --to webp -o ./output --verbose
```

### CLI Options Reference

| Option | Short | Description | Default |
| --- | --- | --- | --- |
| `<INPUT>` | | Input file or directory path (required) | |
| `--to <FORMAT>` | | Target format(s), comma-separated (required) | |
| `--output <DIR>` | `-o` | Output directory | Same as input |
| `--quality <1-100>` | | Compression quality for lossy formats | Format default |
| `--width <PX>` | | Resize width in pixels | |
| `--height <PX>` | | Resize height in pixels | |
| `--keep-aspect` | | Preserve aspect ratio when both width and height set | `false` |
| `--crop <x,y,w,h>` | | Crop region (x, y, width, height) | |
| `--rotate <DEG>` | | Rotate: 90, 180, 270 | |
| `--flip <DIR>` | | Flip: horizontal, vertical | |
| `--grayscale` | | Convert to grayscale | `false` |
| `--invert` | | Invert colors | `false` |
| `--sepia` | | Apply sepia tone | `false` |
| `--blur <SIGMA>` | | Gaussian blur (sigma value) | |
| `--sharpen <VALUE>` | | Unsharp mask | |
| `--brightness <INT>` | | Brightness adjustment (+/-) | |
| `--contrast <FLOAT>` | | Contrast adjustment | |
| `--gamma <FLOAT>` | | Gamma correction | |
| `--watermark <TEXT>` | | Text watermark | |
| `--watermark-position` | | Watermark position | `bottom-right` |
| `--watermark-opacity` | | Watermark opacity (0.0-1.0) | `0.5` |
| `--watermark-font` | | TrueType font file path | Built-in |
| `--overlay <FILE>` | | Overlay image file path | |
| `--overlay-position` | | Overlay position | `bottom-right` |
| `--overlay-opacity` | | Overlay opacity (0.0-1.0) | `1.0` |
| `--auto-orient` | | Auto-correct EXIF orientation | `false` |
| `--no-auto-orient` | | Disable EXIF orientation correction | |
| `--preserve-exif` | | Preserve EXIF metadata | `false` |
| `--info` | | Show image info (no conversion) | `false` |
| `--compare <A> <B>` | | Compare two images (SSIM/PSNR) | |
| `--skip-identical` | | Skip files with identical hash | `false` |
| `--preset <NAME>` | | Conversion preset: web, thumbnail, print, social | |
| `--lossy` | | WebP lossy encoding | Default for WebP |
| `--lossless` | | WebP lossless encoding | |
| `--dpi <DPI>` | | SVG rasterization DPI | `96` |
| `--svg-preset <PRESET>` | | SVG tracing preset: bw, poster, photo | `default` |
| `--overwrite` | | Overwrite existing output files | `false` |
| `--dry-run` | | Print conversion plan without executing | `false` |
| `--verbose` | | Print detailed per-file conversion info | `false` |
| `--extract-frames` | | Extract frames from animated GIF | `false` |
| `--assemble-gif` | | Assemble images into animated GIF | `false` |
| `--frame-delay <MS>` | | Frame delay in milliseconds | `100` |
| `--help` | `-h` | Show help message | |

### Presets

| Preset | Format | Quality | Width | Height | Keep Aspect | Notes |
| --- | --- | --- | --- | --- | --- | --- |
| `web` | WebP (lossy) | 80 | 1920 | — | ✓ | Max width 1920px |
| `thumbnail` | JPEG | 70 | 200 | 200 | ✓ | Square fit |
| `print` | TIFF | — | — | — | — | 300 DPI |
| `social` | JPEG | 85 | 1200 | 630 | ✗ | OG image size |

### Exit Codes

| Code | Meaning |
| --- | --- |
| `0` | All conversions succeeded |
| `1` | Some conversions failed (partial success) |
| `2` | All conversions failed, or argument error |

## Using as a Library

```toml
[dependencies]
imgconv = { path = ".", default-features = false }
```

```rust
use std::path::Path;
use imgconv::convert::{convert_file, ConvertOptions};
use imgconv::format::ImageFormat;

fn main() -> Result<(), imgconv::error::ConvertError> {
    let options = ConvertOptions {
        target_formats: vec![ImageFormat::WebP, ImageFormat::Jpeg],
        quality: Some(80),
        output_dir: Some("./output".into()),
        overwrite: false,
        dry_run: false,
        verbose: false,
        ..Default::default()
    };

    let results = convert_file(Path::new("photo.png"), &options)?;
    for r in &results {
        println!("{} -> {} ({} bytes)",
            r.input_path.display(),
            r.output_path.display(),
            r.output_size,
        );
    }
    Ok(())
}
```

## Architecture

```text
imgconv/
├── src/
│   ├── lib.rs        # Library entry point, public API re-exports
│   ├── main.rs       # CLI binary (clap + indicatif)
│   ├── format.rs     # ImageFormat enum, extension detection
│   ├── convert.rs    # Conversion orchestrator (single file pipeline)
│   ├── batch.rs      # Batch processor (directory, parallel via rayon)
│   ├── raster.rs     # Raster codec (12 formats via image crate)
│   ├── webp.rs       # WebP codec (zenwebp)
│   ├── svg.rs        # SVG rasterizer (resvg) + tracer (vtracer)
│   ├── avif.rs       # AVIF codec (ravif + rav1d, feature-gated)
│   ├── jxl.rs        # JPEG XL decoder (jxl-oxide, feature-gated)
│   ├── dds.rs        # DDS decoder (image dds, feature-gated)
│   ├── pcx.rs        # PCX codec (pcx crate, feature-gated)
│   ├── ultrahdr.rs   # Ultra HDR stub (feature-gated)
│   ├── apng.rs       # APNG codec (feature-gated)
│   ├── resize.rs     # Image resizing with aspect ratio controls
│   ├── crop.rs       # Image cropping
│   ├── transform.rs  # Rotation and flip
│   ├── filter.rs     # Color filters, blur, sharpen, brightness/contrast/gamma
│   ├── watermark.rs  # Text watermark (imageproc + ab_glyph)
│   ├── overlay.rs    # Image overlay compositing
│   ├── exif.rs       # EXIF auto-orient and preservation
│   ├── info.rs       # Image metadata info output
│   ├── compare.rs    # SSIM/PSNR comparison (feature-gated)
│   ├── dedup.rs      # SHA-256 deduplication (feature-gated)
│   ├── preset.rs     # Conversion presets
│   ├── quality.rs    # Quality defaults and validation
│   ├── animation.rs  # GIF frame extract / assemble
│   └── error.rs      # ConvertError enum (thiserror)
```

### Conversion Pipeline

```text
Input → Format Detection → Decode → [Auto-Orient] → [Crop] → [Resize]
  → [Rotate/Flip] → [Filters] → [Brightness/Contrast/Gamma]
  → [Blur/Sharpen] → [Watermark] → [Overlay] → Encode → [Preserve EXIF] → Output
```

## Building & Testing

```bash
# Build
cargo build --release

# Build with all features
cargo build --release --features avif,jxl,dds,pcx,compare,dedup

# Run tests
cargo test

# Run tests with all features
cargo test --features "pcx,compare,dedup"
```

## Test Results

- Base tests: 173 passed
- Full tests (with pcx, compare, dedup): 188 passed
- Property-based tests: 28 included
- Failures: 0

## License

MIT
