# Maintainer: John Kinell <johnkinell@gmail.com>

pkgname=unfocol
pkgver=0.1.0
pkgrel=1
pkgdesc="Unfocused Focus TUI timer that uses colors from your terminal theme"
arch=('x86_64')
url="https://github.com/MrOnijohn/unfocol"
license=('MIT')
depends=()            # no runtime deps beyond glibc, which is implicit
makedepends=('cargo')
source=("$pkgname-$pkgver.tar.gz::$url/archive/refs/tags/v$pkgver.tar.gz"
        "unfocol.desktop")
sha256sums=('6c46245612d6e8a48db2b7c583c412983c1ec389891ef2525c9896366cdfe67b'
            '30c0fa6dace499918159b534510488cf1ec1b22d5502071c16fe92ddb2ff8f85')

build() {
  cd "$srcdir/$pkgname-$pkgver"
  cargo build --release --locked
}

package() {
  cd "$srcdir/$pkgname-$pkgver"

  # binary
  install -Dm755 "target/release/unfocol" "$pkgdir/usr/bin/unfocol"

  # license
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"

  # .desktop entry
  install -Dm644 "$srcdir/unfocol.desktop" \
    "$pkgdir/usr/share/applications/unfocol.desktop"

  # icons (from assets/icons/)
  for icon in assets/icons/icon-*.png; do
    size=$(basename "$icon" | cut -d'-' -f2 | cut -dx -f1)
    install -Dm644 "$icon" \
      "$pkgdir/usr/share/icons/hicolor/${size}x${size}/apps/unfocol.png"
  done
}
