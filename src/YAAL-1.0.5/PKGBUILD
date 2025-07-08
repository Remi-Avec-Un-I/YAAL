# Maintainer: Remi-Avec-Un-I <ri@jsp13.com>
pkgname=YAAL
pkgver=1.0.5
pkgrel=1
pkgdesc="Yet Another Awesome Launcher, even tho it can do more"
arch=('x86_64')
url="https://github.com/Remi-Avec-Un-I/YAAL"
license=('MIT')
depends=('gcc-libs')
makedepends=('rust' 'cargo')
provides=('yaal')
source=("$pkgname-$pkgver.tar.gz::https://github.com/Remi-Avec-Un-I/YAAL/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')


build() {
  cd "$pkgname-$pkgver"

  cargo build --release --locked
}


package() {
  cd "$pkgname-$pkgver"
  install -Dm755 "target/release/yaal" "$pkgdir/usr/bin/yaal"
  
  install -Dm644 "yaal.desktop" "$pkgdir/usr/share/applications/yaal.desktop"
  
  install -Dm644 "README.md" "$pkgdir/usr/share/doc/yaal/README.md"
  install -Dm644 "LICENSE" "$pkgdir/usr/share/licenses/yaal/LICENSE"
} 