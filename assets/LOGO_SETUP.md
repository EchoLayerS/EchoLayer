# EchoLayer Logo Setup Guide

## Quick Setup Instructions

### Step 1: Prepare Your Logo
- **Format**: PNG (recommended)
- **Size**: 200x200 pixels (optimal)
- **Background**: Transparent or solid color
- **Quality**: High resolution for sharp display

### Step 2: Add Logo File
1. Save your logo file as `echolayer-logo.png`
2. Place it in the `assets/images/` directory
3. The path should be: `assets/images/echolayer-logo.png`

### Step 3: Verify Display
- The logo will automatically appear at the top of README.md
- Check the display by viewing the README file in GitHub or locally

## Current Setup
The README.md file already contains the logo reference:
```html
<div align="center">
  <img src="assets/images/echolayer-logo.png" alt="EchoLayer Logo" width="200" height="200">
</div>
```

## Directory Structure
```
EchoLayer/
├── assets/
│   └── images/
│       ├── README.md
│       └── echolayer-logo.png  # <-- Add your logo here
└── README.md
```

## Troubleshooting
- If logo doesn't appear, check the file name is exactly `echolayer-logo.png`
- Ensure the file is in the correct directory: `assets/images/`
- Verify the image format is PNG and not corrupted

## Alternative Formats
If you need to use a different format:
1. Update the filename in README.md
2. Supported formats: PNG, JPG, JPEG, SVG, GIF 