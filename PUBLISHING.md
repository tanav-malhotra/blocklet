# Publishing Guide for Blocklet

This guide covers how to publish Blocklet to various package managers.

## Prerequisites

Before publishing, ensure:
1. All tests pass: `cargo test`
2. Code is properly formatted: `cargo fmt`
3. No clippy warnings: `cargo clippy`
4. Version is updated in `Cargo.toml`
5. `CHANGELOG.md` is updated
6. `README.md` reflects latest features
7. Git repository is clean and pushed

## 1. Publishing to crates.io

### First Time Setup
```bash
# Login to crates.io (requires account)
cargo login YOUR_API_TOKEN
```

### Publishing
```bash
# Build and test
cargo build --release
cargo test

# Publish
cargo publish --dry-run  # Test first
cargo publish
```

### Verification
```bash
# After a few minutes, verify at:
# https://crates.io/crates/blocklet

# Test installation
cargo install blocklet
```

## 2. Publishing to WinGet (Windows Package Manager)

### First Time Setup
1. Fork the `microsoft/winget-pkgs` repository
2. Clone your fork locally

### Publishing Process
```bash
# 1. Create a new release on GitHub with tag v0.1.0
# 2. Upload the Windows binary to the release

# 3. Calculate SHA256 of the release file
certutil -hashfile blocklet-windows-amd64.exe SHA256

# 4. Update winget/blocklet.installer.yaml with the SHA256

# 5. Copy manifests to winget-pkgs
cd path/to/winget-pkgs
mkdir -p manifests/t/TanavMalhotra/Blocklet/0.1.0
cp path/to/blocklet/winget/* manifests/t/TanavMalhotra/Blocklet/0.1.0/

# 6. Commit and create PR
git checkout -b add-blocklet-0.1.0
git add manifests/t/TanavMalhotra/Blocklet/0.1.0/
git commit -m "Add Blocklet version 0.1.0"
git push origin add-blocklet-0.1.0

# 7. Create PR to microsoft/winget-pkgs
```

### Verification
```bash
# After PR is merged
winget search blocklet
winget install TanavMalhotra.Blocklet
```

## 3. Publishing to Chocolatey (Windows)

### First Time Setup
```bash
# Install chocolatey (if not installed)
# Then get API key from https://community.chocolatey.org/account

# Set API key
choco apikey --key YOUR_API_KEY --source https://push.chocolatey.org/
```

### Publishing Process
```bash
# 1. Build the package
cd choco
choco pack

# 2. Test locally
choco install blocklet -source .

# 3. Publish to Chocolatey
choco push blocklet.0.1.0.nupkg --source https://push.chocolatey.org/
```

### Verification
```bash
# After approval (takes 1-3 days)
choco search blocklet
choco install blocklet
```

## 4. Publishing to APT (Debian/Ubuntu)

### Option A: Using PPA (Personal Package Archive)

```bash
# 1. Create a PPA on Launchpad
# Visit: https://launchpad.net/

# 2. Build source package
cd debian
debuild -S -sa

# 3. Upload to PPA
dput ppa:your-username/blocklet ../blocklet_0.1.0_source.changes
```

### Option B: Creating .deb package

```bash
# 1. Build the binary
cargo build --release

# 2. Create package structure
mkdir -p blocklet_0.1.0-1_amd64/DEBIAN
mkdir -p blocklet_0.1.0-1_amd64/usr/bin

# 3. Copy files
cp target/release/blocklet blocklet_0.1.0-1_amd64/usr/bin/
cp debian/control blocklet_0.1.0-1_amd64/DEBIAN/

# 4. Build .deb
dpkg-deb --build blocklet_0.1.0-1_amd64

# 5. Test installation
sudo dpkg -i blocklet_0.1.0-1_amd64.deb
```

### Hosting the repository

You can host your own APT repository:

```bash
# 1. Create repository structure
mkdir -p repo/dists/stable/main/binary-amd64
mv blocklet_0.1.0-1_amd64.deb repo/

# 2. Generate Packages file
cd repo
dpkg-scanpackages . /dev/null | gzip -9c > Packages.gz

# 3. Host on GitHub Pages or your own server
```

Users can then add your repository:
```bash
echo "deb [trusted=yes] https://your-domain.com/repo ./" | sudo tee /etc/apt/sources.list.d/blocklet.list
sudo apt update
sudo apt install blocklet
```

## 5. Publishing to Homebrew (macOS/Linux)

### Creating a Homebrew Formula

```bash
# 1. Fork homebrew-core (for official) or create your tap
# For your own tap:
brew tap-new your-username/blocklet

# 2. Create formula
cd $(brew --repository your-username/blocklet)
brew create --set-name blocklet https://github.com/tanav-malhotra/blocklet/archive/v0.1.0.tar.gz

# 3. Edit formula (Formula/blocklet.rb)
# The formula should look like:
```

```ruby
class Blocklet < Formula
  desc "ASCII art generator using Unicode block characters"
  homepage "https://github.com/tanav-malhotra/blocklet"
  url "https://github.com/tanav-malhotra/blocklet/archive/v0.1.0.tar.gz"
  sha256 "CALCULATED_SHA"
  license "GPL-3.0"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "Blocklet", shell_output("#{bin}/blocklet --version")
  end
end
```

```bash
# 4. Test formula
brew install --build-from-source Formula/blocklet.rb
brew test blocklet

# 5. Commit and push
git add Formula/blocklet.rb
git commit -m "Add Blocklet 0.1.0"
git push
```

Users can install:
```bash
brew tap your-username/blocklet
brew install blocklet
```

## 6. Publishing to AUR (Arch Linux/Pacman)

### Initial Setup
1. Create an AUR account at https://aur.archlinux.org/register
2. Add your SSH key at https://aur.archlinux.org/account/

### Publishing Process

```bash
# 1. Test PKGBUILD locally
makepkg -si  # Build and install

# 2. Generate .SRCINFO
makepkg --printsrcinfo > .SRCINFO

# 3. Clone AUR repository (first time)
git clone ssh://aur@aur.archlinux.org/blocklet.git blocklet-aur
cd blocklet-aur

# 4. Copy files
cp ../PKGBUILD .
cp ../.SRCINFO .

# 5. Update SHA256 in PKGBUILD
# Download release tarball and calculate hash:
wget https://github.com/tanav-malhotra/blocklet/archive/v0.1.0.tar.gz
sha256sum v0.1.0.tar.gz
# Update sha256sums=('...') in PKGBUILD

# 6. Regenerate .SRCINFO
makepkg --printsrcinfo > .SRCINFO

# 7. Commit and push
git add PKGBUILD .SRCINFO
git commit -m "Initial upload: blocklet 0.1.0"
git push
```

### Verification
```bash
# Users can install via:
yay -S blocklet
# or
paru -S blocklet
# or
git clone https://aur.archlinux.org/blocklet.git
cd blocklet
makepkg -si
```

### Updating the Package
```bash
# Update version in PKGBUILD
# Increment pkgrel if same version, or update pkgver for new version
makepkg --printsrcinfo > .SRCINFO
git add PKGBUILD .SRCINFO
git commit -m "Update to version 0.1.1"
git push
```

## 7. Publishing to Fedora/RHEL (DNF/RPM)

### Option A: COPR (Recommended for testing)

COPR is Fedora's equivalent to Ubuntu's PPA.

```bash
# 1. Create COPR account at https://copr.fedorainfracloud.org/

# 2. Create new project
# Name: blocklet
# Description: ASCII art generator using Unicode blocks
# Instructions: Automatic builds from Git

# 3. Add package from git
# Clone URL: https://github.com/tanav-malhotra/blocklet
# Spec file location: blocklet.spec

# 4. Enable auto-builds on new tags
```

Users can install:
```bash
sudo dnf copr enable your-username/blocklet
sudo dnf install blocklet
```

### Option B: Building RPM locally

```bash
# 1. Install dependencies
sudo dnf install rpm-build rpmdevtools rust cargo

# 2. Setup RPM build tree
rpmdev-setuptree

# 3. Download source
wget https://github.com/tanav-malhotra/blocklet/archive/v0.1.0.tar.gz -O ~/rpmbuild/SOURCES/v0.1.0.tar.gz

# 4. Copy spec file
cp blocklet.spec ~/rpmbuild/SPECS/

# 5. Build RPM
cd ~/rpmbuild/SPECS
rpmbuild -ba blocklet.spec

# 6. Install
sudo dnf install ~/rpmbuild/RPMS/x86_64/blocklet-0.1.0-1.*.x86_64.rpm
```

### Option C: Submit to official Fedora repos

This is a longer process but makes it available in official repos:

```bash
# 1. Create Fedora Account System (FAS) account
# Visit: https://accounts.fedoraproject.org/

# 2. Join the packagers group
# Follow: https://fedoraproject.org/wiki/Join_the_package_collection_maintainers

# 3. Request a package review
# Create bug at: https://bugzilla.redhat.com/
# Component: Package Review
# Summary: Review Request: blocklet - ASCII art generator

# 4. After approval, import package
fedpkg clone blocklet
cd blocklet
cp ../blocklet.spec .
git add blocklet.spec
git commit -m "Initial import"
fedpkg new-sources v0.1.0.tar.gz
git push

# 5. Build for different Fedora versions
fedpkg build --target f39
fedpkg build --target f40
```

### Creating your own RPM repository

```bash
# 1. Build RPMs as shown above

# 2. Create repository structure
mkdir -p repo/fedora/39/x86_64
cp ~/rpmbuild/RPMS/x86_64/*.rpm repo/fedora/39/x86_64/

# 3. Create repo metadata
cd repo/fedora/39
createrepo x86_64

# 4. Host on web server or GitHub Pages
```

Users add your repo:
```bash
# Create /etc/yum.repos.d/blocklet.repo
[blocklet]
name=Blocklet Repository
baseurl=https://your-domain.com/repo/fedora/$releasever/$basearch
enabled=1
gpgcheck=0
```

Then install:
```bash
sudo dnf install blocklet
```

## 8. GitHub Release

```bash
# 1. Create and push tag
git tag -a v0.1.0 -m "Release version 0.1.0"
git push origin v0.1.0

# 2. Build for multiple platforms
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-msvc
cargo build --release --target x86_64-apple-darwin

# 3. Create release on GitHub
# Upload binaries for each platform

# 4. Use GitHub Actions (see .github/workflows/release.yml)
# This will automatically build and upload binaries
```

## Release Checklist

- [ ] Update version in `Cargo.toml`
- [ ] Update `CHANGELOG.md`
- [ ] Update `README.md` if needed
- [ ] Run all tests: `cargo test`
- [ ] Run benchmarks: `cargo bench`
- [ ] Format code: `cargo fmt`
- [ ] Check with clippy: `cargo clippy`
- [ ] Build release binaries for all platforms
- [ ] Create Git tag
- [ ] Publish to crates.io
- [ ] Create GitHub Release
- [ ] Update WinGet manifests
- [ ] Update Chocolatey package
- [ ] Update Homebrew formula (if applicable)
- [ ] Update Debian package (if applicable)
- [ ] Announce on social media/relevant forums

## Post-Release

1. Monitor for installation issues
2. Respond to bug reports
3. Plan next release features
4. Update documentation if needed

## Support

For questions or issues with publishing:
- Create an issue at https://github.com/tanav-malhotra/blocklet/issues
- Email: tanavm2009@gmail.com

