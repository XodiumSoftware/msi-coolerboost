# Maintainer: Illyrius <28700752+illyrius666@users.noreply.github.com>
pkgname=msi-coolerboost
pkgver=1.0.0
pkgrel=1
pkgdesc="System tray utility for toggling MSI CoolerBoost on Linux"
arch=('x86_64')
url="https://github.com/XodiumSoftware/msi-coolerboost"
license=('MIT')
depends=('gtk3' 'libnotify')
makedepends=('cargo')
source=("$pkgname-$pkgver.tar.gz::$url/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
  cd "$pkgname-$pkgver"
  cargo build --release --locked
}

package() {
  cd "$pkgname-$pkgver"

  install -Dm755 target/release/tray "$pkgdir/usr/bin/$pkgname"
  install -Dm755 target/release/toggle "$pkgdir/usr/bin/$pkgname-toggle"

  install -Dm644 "$pkgname.desktop" "$pkgdir/usr/share/applications/$pkgname.desktop"

  install -Dm644 LICENSE.md "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
