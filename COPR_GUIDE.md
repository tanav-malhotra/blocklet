# Publishing to COPR (Fedora/RHEL)

Quick guide for publishing Blocklet to COPR for Fedora, RHEL, and CentOS users.

## What is COPR?

COPR (Cool Other Package Repo) is Fedora's equivalent to Ubuntu's PPA. It allows you to host RPM packages for Fedora, RHEL, CentOS, and other RPM-based distributions.

## Prerequisites

1. **Fedora Account**: Register at https://accounts.fedoraproject.org/
2. **COPR Account**: Login at https://copr.fedorainfracloud.org/ with Fedora account

## Publishing via Web Interface (Easiest)

### Step 1: Create New Project

1. Go to https://copr.fedorainfracloud.org/coprs/add/
2. Fill in:
   - **Project name**: `blocklet`
   - **Description**: `ASCII art generator using Unicode block characters`
   - **Instructions**: 
     ```
     A cross-platform CLI tool that generates ASCII art using Unicode 
     block characters, similar to figlet but with beautiful shadows.
     ```
   - **Chroots**: Select Fedora versions and RHEL versions you want to support
     - Fedora 39
     - Fedora 40
     - Fedora Rawhide
     - EPEL 8
     - EPEL 9
   - **Build dependencies**: `rust cargo`
   - **Enable auto-rebuilds**: Yes

### Step 2: Add Package from Git

1. In your COPR project, click "Packages" → "Add Package" → "SCM"
2. Fill in:
   - **Package name**: `blocklet`
   - **Clone URL**: `https://github.com/tanav-malhotra/blocklet`
   - **Committish**: `v0.1.0` (or branch name)
   - **Spec File**: `blocklet.spec`
   - **Type**: `rpkg`
   - **Auto-rebuild**: Yes (rebuilds on new commits/tags)

3. Click "Build"

### Step 3: Verify Build

1. Watch build progress on build page
2. Typical build time: 5-15 minutes
3. Check build logs if it fails

### Step 4: Test Installation

```bash
# Enable your COPR repo
sudo dnf copr enable YOUR_USERNAME/blocklet

# Install
sudo dnf install blocklet

# Test
blocklet "Hello COPR"
```

## Publishing via CLI (Advanced)

### Step 1: Install COPR CLI

```bash
sudo dnf install copr-cli

# Get API token from https://copr.fedorainfracloud.org/api/
# Save to ~/.config/copr
```

### Step 2: Create Project

```bash
copr-cli create blocklet \
  --chroot fedora-39-x86_64 \
  --chroot fedora-40-x86_64 \
  --description "ASCII art generator using Unicode block characters" \
  --instructions "sudo dnf copr enable YOUR_USERNAME/blocklet && sudo dnf install blocklet"
```

### Step 3: Build from Git

```bash
# Build from URL
copr-cli buildscm blocklet \
  --clone-url https://github.com/tanav-malhotra/blocklet \
  --committish v0.1.0 \
  --spec blocklet.spec \
  --type rpkg
```

## Building Locally First (Recommended)

Before pushing to COPR, test locally:

### On Fedora

```bash
# Install dependencies
sudo dnf install rpm-build rpmdevtools rust cargo

# Setup build tree
rpmdev-setuptree

# Download source
wget https://github.com/tanav-malhotra/blocklet/archive/v0.1.0.tar.gz \
  -O ~/rpmbuild/SOURCES/v0.1.0.tar.gz

# Copy spec file
cp blocklet.spec ~/rpmbuild/SPECS/

# Build
cd ~/rpmbuild/SPECS
rpmbuild -ba blocklet.spec

# Test install
sudo dnf install ~/rpmbuild/RPMS/x86_64/blocklet-0.1.0-1.fc*.x86_64.rpm

# Test run
blocklet "Hello RPM"
```

### Using Mock (any Linux)

```bash
# Install mock
sudo dnf install mock
sudo usermod -a -G mock $USER

# Build for Fedora 40
mock -r fedora-40-x86_64 --rebuild blocklet-0.1.0-1.fc40.src.rpm

# Results in /var/lib/mock/fedora-40-x86_64/result/
```

## Updating Your Package

### Via Web Interface
1. Go to your COPR project
2. Click "Packages" → Select package
3. Click "Rebuild"
4. Or enable auto-rebuild to build on new commits

### Via CLI
```bash
# Rebuild with new version
copr-cli buildscm blocklet \
  --clone-url https://github.com/tanav-malhotra/blocklet \
  --committish v0.1.1 \
  --spec blocklet.spec \
  --type rpkg
```

## Common Issues

### Issue: "Rust not found"
**Solution**: Add to spec file:
```spec
BuildRequires: rust >= 1.70
BuildRequires: cargo
```

### Issue: "Build takes too long / timeout"
**Solution**: 
- COPR has 4-hour build limit
- Optimize cargo build with: `cargo build --release --locked`
- Consider using `--target` for specific arch

### Issue: "Dependencies not available"
**Solution**: 
- All cargo dependencies should be fetched during build
- No need to package Rust dependencies separately
- Use `cargo vendor` if offline build needed

## Package Variants

You can create multiple packages:

- `blocklet` - Builds from source (current)
- `blocklet-nightly` - Latest git version
- `blocklet-bin` - Pre-built binary

## Monitoring

- **Build notifications**: Enabled by default (email)
- **Usage statistics**: View on project page
- **Issues**: Users can report via COPR or GitHub

## Best Practices

1. **Test locally** before COPR
2. **Enable auto-rebuild** for automatic updates
3. **Support multiple Fedora versions**
4. **Keep spec file clean** and commented
5. **Respond to build failures** quickly
6. **Document installation** in project README

## User Installation

After publishing, users can install with:

```bash
# Enable repository
sudo dnf copr enable YOUR_USERNAME/blocklet

# Install package
sudo dnf install blocklet

# Update package
sudo dnf update blocklet

# Remove repository
sudo dnf copr disable YOUR_USERNAME/blocklet
```

## Moving to Official Fedora Repos

If your package becomes popular, you can submit to official Fedora repositories:

1. **Join Fedora Packagers**: https://fedoraproject.org/wiki/Join_the_package_collection_maintainers
2. **Package Review**: Submit to Bugzilla
3. **Import to Fedora**: After approval
4. **Maintain**: Keep package updated

Benefits:
- Available by default on all Fedora systems
- Higher visibility
- Community support

Process takes 1-2 months typically.

## Quick Reference

```bash
# Install COPR CLI
sudo dnf install copr-cli

# Create project
copr-cli create PROJECT_NAME

# Build from git
copr-cli buildscm PROJECT_NAME --clone-url URL --spec SPEC

# List your projects
copr-cli list

# Monitor builds
copr-cli monitor

# Delete project
copr-cli delete PROJECT_NAME
```

## Resources

- COPR Documentation: https://docs.pagure.org/copr.copr/
- Fedora Packaging: https://docs.fedoraproject.org/en-US/packaging-guidelines/
- RPM Packaging: https://rpm-packaging-guide.github.io/
- Mock: https://github.com/rpm-software-management/mock/wiki

Good luck! 🎉

