# Quick Start: Publishing Blocklet 🚀

## Immediate Actions (in order)

### 1. Publish to crates.io (5 minutes)
```bash
# Login (one time)
cargo login YOUR_API_TOKEN  # Get from https://crates.io/settings/tokens

# Publish
cargo publish --dry-run  # Test first
cargo publish            # Actually publish
```

**Verification**: Visit https://crates.io/crates/blocklet in 2-3 minutes

### 2. Create GitHub Release (10 minutes)
```bash
# Tag and push
git add -A
git commit -m "Prepare for v0.1.0 release"
git push
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0
```

Then on GitHub:
1. Go to https://github.com/tanav-malhotra/blocklet/releases
2. Click "Create a new release"
3. Select tag `v0.1.0`
4. Title: `Blocklet v0.1.0`
5. Description: Copy from CHANGELOG.md
6. The GitHub Action will automatically build binaries and attach them

### 3. WinGet (15 minutes + review time)
After GitHub release is complete and binaries are uploaded:

```bash
# 1. Fork microsoft/winget-pkgs on GitHub

# 2. Clone your fork
git clone https://github.com/YOUR_USERNAME/winget-pkgs
cd winget-pkgs

# 3. Get SHA256 of your Windows release
# Download from GitHub release, then:
certutil -hashfile blocklet-windows-amd64.exe SHA256

# 4. Update winget/blocklet.installer.yaml with SHA256
# Replace <TO_BE_FILLED_AFTER_RELEASE> with actual hash

# 5. Copy manifests
mkdir -p manifests/t/TanavMalhotra/Blocklet/0.1.0
cp ../blocklet/winget/* manifests/t/TanavMalhotra/Blocklet/0.1.0/

# 6. Create PR
git checkout -b add-blocklet-0.1.0
git add manifests/t/TanavMalhotra/Blocklet/
git commit -m "New package: TanavMalhotra.Blocklet version 0.1.0"
git push origin add-blocklet-0.1.0
```

Then create a PR to microsoft/winget-pkgs

**Timeline**: PR review takes 1-3 days

### 4. Chocolatey (20 minutes + approval time)
```bash
# 1. Get API key from https://community.chocolatey.org/account
choco apikey --key YOUR_API_KEY --source https://push.chocolatey.org/

# 2. Get SHA256 from GitHub release
certutil -hashfile blocklet-windows-amd64.exe SHA256

# 3. Update choco/tools/chocolateyinstall.ps1 with SHA256

# 4. Build package
cd choco
choco pack

# 5. Test locally (optional)
choco install blocklet -source . -y

# 6. Push to Chocolatey
choco push blocklet.0.1.0.nupkg --source https://push.chocolatey.org/
```

**Timeline**: Approval takes 1-3 days

### 5. APT/Debian (Optional - 30 minutes)

**Quick option**: Host a simple repository on GitHub Pages

```bash
# Build .deb
cargo build --release
mkdir -p blocklet_0.1.0-1_amd64/DEBIAN
mkdir -p blocklet_0.1.0-1_amd64/usr/bin
cp target/release/blocklet blocklet_0.1.0-1_amd64/usr/bin/
cp debian/control blocklet_0.1.0-1_amd64/DEBIAN/
dpkg-deb --build blocklet_0.1.0-1_amd64

# Upload .deb to GitHub Releases
# Users can download and install with:
# wget https://github.com/tanav-malhotra/blocklet/releases/download/v0.1.0/blocklet_0.1.0-1_amd64.deb
# sudo dpkg -i blocklet_0.1.0-1_amd64.deb
```

### 6. AUR/Pacman for Arch Linux (20 minutes)

```bash
# 1. Create AUR account at https://aur.archlinux.org/register
# 2. Add SSH key

# 3. Test PKGBUILD
makepkg -si

# 4. Get SHA256 of release tarball
wget https://github.com/tanav-malhotra/blocklet/archive/v0.1.0.tar.gz
sha256sum v0.1.0.tar.gz

# 5. Update PKGBUILD with SHA256

# 6. Clone AUR repo
git clone ssh://aur@aur.archlinux.org/blocklet.git blocklet-aur
cd blocklet-aur

# 7. Copy files and generate .SRCINFO
cp ../PKGBUILD .
makepkg --printsrcinfo > .SRCINFO

# 8. Push to AUR
git add PKGBUILD .SRCINFO
git commit -m "Initial upload: blocklet 0.1.0"
git push
```

**Timeline**: Immediate availability after push

Users install with:
```bash
yay -S blocklet
# or
paru -S blocklet
```

### 7. COPR for Fedora/DNF (15 minutes)

```bash
# 1. Create COPR account at https://copr.fedorainfracloud.org/
# 2. Create new project "blocklet"
# 3. Add package from git:
#    - Clone URL: https://github.com/tanav-malhotra/blocklet
#    - Spec file: blocklet.spec
# 4. Enable automatic builds
```

Users install with:
```bash
sudo dnf copr enable your-username/blocklet
sudo dnf install blocklet
```

**Timeline**: Builds complete in 10-20 minutes

### 8. Homebrew (Optional - 20 minutes)

Create your own tap:
```bash
brew tap-new tanav-malhotra/blocklet
cd $(brew --repository tanav-malhotra/blocklet)

# Create Formula/blocklet.rb (see PUBLISHING.md for template)

# Test
brew install --build-from-source Formula/blocklet.rb
brew test blocklet

# Push
git add Formula/blocklet.rb
git commit -m "Add Blocklet 0.1.0"
git push
```

## Priority Order

If you want to publish gradually:

1. **Day 1**: crates.io + GitHub Release (Required, takes 15 mins)
2. **Day 2**: WinGet PR (Windows users, takes 15 mins + wait for approval)
3. **Day 3**: AUR + COPR (Arch & Fedora users, takes 35 mins total)
4. **Day 4**: Chocolatey (Alternative for Windows, takes 20 mins + wait)
5. **Later**: APT, Homebrew (if you want Debian/Ubuntu/Mac package managers)

## Verification Commands

After publishing, users can install via:

```bash
# Rust/Cargo (all platforms)
cargo install blocklet

# Windows (WinGet)
winget install TanavMalhotra.Blocklet

# Windows (Chocolatey)
choco install blocklet

# Arch Linux (AUR)
yay -S blocklet
# or
paru -S blocklet

# Fedora/RHEL (COPR)
sudo dnf copr enable your-username/blocklet
sudo dnf install blocklet

# Debian/Ubuntu
sudo dpkg -i blocklet_0.1.0-1_amd64.deb

# macOS (Homebrew)
brew tap tanav-malhotra/blocklet
brew install blocklet
```

## Pre-Publication Checklist

- [x] README.md updated with latest features
- [x] CHANGELOG.md created
- [x] Cargo.toml ready
- [x] LICENSE file present (GPL-3.0)
- [x] All tests passing
- [x] Code formatted and linted
- [ ] Git repository clean and pushed
- [ ] GitHub Actions workflow ready
- [ ] Package manager configs created

## Need Help?

See `PUBLISHING.md` for detailed guides for each platform.

Good luck! 🎉

