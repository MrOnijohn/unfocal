# Maintainer: John Kinell <johnkinell@gmail.com>

pkgname=unfocol
pkgver=0.1.0
pkgrel=1
pkgdesc="Unfocused Focus TUI app using colors"
arch=('x86_64')
url="https://github.com/MrOnijohn/unfocol"
license=('MIT')
depends=('glibc')  # runtime deps, add 'alacritty' or 'ghostty' if you want to enforce a terminal
makedepends=('cargo')
source=("$pkgname-$pkgver.tar.gz::$url/archive/refs/tags/v$pkgver.tar.gz"
        "unfocol.desktop"
        "unfocol.png")
sha256sums=('SKIP' 'SKIP' 'SKIP')

build() {
  cd "$srcdir/$pkgname-$pkgver"
  cargo build --release --locked
}

package() {
  cd "$srcdir/$pkgname-$pkgver"

  # binary
  install -Dm755 "target/release/unfocol" "$pkgdir/usr/bin/unfocol"

  # .desktop entry
  install -Dm644 "$srcdir/unfocol.desktop" \
    "$pkgdir/usr/share/applications/unfocol.desktop"

  # icon
  install -Dm644 "$srcdir/unfocol.png" \
    "$pkgdir/usr/share/icons/hicolor/64x64/apps/unfocol.png"
}
