# Maintainer: Tanav Malhotra <tanavm2009@gmail.com>
pkgname=blocklet
pkgver=0.1.0
pkgrel=1
pkgdesc="A cross-platform CLI tool that generates ASCII art using Unicode block characters"
arch=('x86_64' 'aarch64')
url="https://github.com/tanav-malhotra/blocklet"
license=('GPL3')
depends=()
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/tanav-malhotra/blocklet/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')  # Update with actual SHA256 after release

build() {
    cd "$pkgname-$pkgver"
    cargo build --release --locked --all-features
}

check() {
    cd "$pkgname-$pkgver"
    cargo test --release --locked
}

package() {
    cd "$pkgname-$pkgver"
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
    install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
    install -Dm644 CHANGELOG.md "$pkgdir/usr/share/doc/$pkgname/CHANGELOG.md"
}

