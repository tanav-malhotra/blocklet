# Publishing to AUR (Arch User Repository)

Quick guide for publishing Blocklet to AUR for Arch Linux users.

## Prerequisites

1. **AUR Account**: Register at https://aur.archlinux.org/register
2. **SSH Key**: Add your SSH public key at https://aur.archlinux.org/account/
3. **Arch Linux**: Need Arch/Manjaro/EndeavourOS to test (or use Docker)

## First Time Publication

### Step 1: Test PKGBUILD Locally

```bash
# Clone your project
cd /path/to/blocklet

# Test the PKGBUILD
makepkg -si

# This will:
# - Download source
# - Build the package
# - Install it
# - Test if it works: blocklet "Hello AUR"
```

### Step 2: Calculate SHA256

```bash
# Download release tarball
wget https://github.com/tanav-malhotra/blocklet/archive/v0.1.0.tar.gz

# Calculate SHA256
sha256sum v0.1.0.tar.gz

# Copy the hash and update PKGBUILD:
# sha256sums=('YOUR_HASH_HERE')
```

### Step 3: Generate .SRCINFO

```bash
# Must be regenerated every time PKGBUILD changes
makepkg --printsrcinfo > .SRCINFO

# Verify it looks correct
cat .SRCINFO
```

### Step 4: Push to AUR

```bash
# Clone AUR repository (creates empty repo)
git clone ssh://aur@aur.archlinux.org/blocklet.git blocklet-aur
cd blocklet-aur

# Copy your files
cp ../PKGBUILD .
cp ../.SRCINFO .

# Commit and push
git add PKGBUILD .SRCINFO
git commit -m "Initial upload: blocklet 0.1.0"
git push origin master
```

### Step 5: Verify

Visit https://aur.archlinux.org/packages/blocklet

Users can now install:
```bash
# Using yay
yay -S blocklet

# Using paru
paru -S blocklet

# Manual
git clone https://aur.archlinux.org/blocklet.git
cd blocklet
makepkg -si
```

## Updating the Package

When you release a new version:

```bash
cd blocklet-aur

# Method 1: New version number
# Edit PKGBUILD:
# - Update pkgver=0.1.1
# - Reset pkgrel=1
# - Update sha256sums

# Method 2: Same version (if fixing PKGBUILD)
# Edit PKGBUILD:
# - Keep pkgver same
# - Increment pkgrel=2

# Regenerate .SRCINFO
makepkg --printsrcinfo > .SRCINFO

# Test locally
makepkg -si

# Commit and push
git add PKGBUILD .SRCINFO
git commit -m "Update to version 0.1.1"
git push
```

## Common Issues

### Issue: "invalid signature"
**Solution**: Update your SSH key at https://aur.archlinux.org/account/

### Issue: "PKGBUILD already exists"
**Solution**: 
```bash
# Check if name is taken
curl https://aur.archlinux.org/rpc/?v=5&type=info&arg[]=blocklet

# If taken, pick a different name like blocklet-bin or blocklet-git
```

### Issue: "sha256sum mismatch"
**Solution**: 
```bash
# Download exact tarball
wget https://github.com/tanav-malhotra/blocklet/archive/v0.1.0.tar.gz

# Calculate correct hash
sha256sum v0.1.0.tar.gz

# Update PKGBUILD
```

## AUR Best Practices

1. **Always test locally** before pushing
2. **Respond to comments** on AUR page
3. **Keep package updated** with new releases
4. **Follow AUR guidelines**: https://wiki.archlinux.org/title/AUR_submission_guidelines
5. **Add maintainer info** in PKGBUILD comments

## Package Variants

You can create multiple AUR packages:

- `blocklet` - Builds from source (current)
- `blocklet-bin` - Pre-built binary from GitHub releases
- `blocklet-git` - Latest git version

Example `blocklet-bin` PKGBUILD:
```bash
pkgname=blocklet-bin
pkgver=0.1.0
source=("https://github.com/tanav-malhotra/blocklet/releases/download/v$pkgver/blocklet-linux-amd64")
# No build() function needed, just package() to install binary
```

## Adoption/Orphaning

If you want to:
- **Orphan package**: Click "Disown Package" on AUR page
- **Adopt orphaned package**: Click "Adopt Package" (must be orphaned first)
- **Transfer to co-maintainer**: Add them, then disown

## Monitoring

- **Out-of-date notifications**: Users can flag package
- **Comments**: Get email notifications
- **Votes**: Indicates popularity
- **Check regularly**: Ensure builds still work

## Quick Commands Reference

```bash
# Clone AUR package
git clone ssh://aur@aur.archlinux.org/blocklet.git

# Test build
makepkg -si

# Check for errors
namcap PKGBUILD
namcap *.pkg.tar.zst

# Update .SRCINFO
makepkg --printsrcinfo > .SRCINFO

# Push update
git add PKGBUILD .SRCINFO
git commit -m "Update message"
git push
```

## Resources

- AUR Guidelines: https://wiki.archlinux.org/title/AUR_submission_guidelines
- PKGBUILD Guide: https://wiki.archlinux.org/title/PKGBUILD
- AUR Helpers: https://wiki.archlinux.org/title/AUR_helpers
- Arch Forums: https://bbs.archlinux.org/

Good luck! 🎉

