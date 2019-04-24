# Maintainer: jingjie <atmouse.cc g-mail>

pkgname='otp-uinput'
pkgdesc="otp-uinput daemon"
pkgver=0.1
pkgrel=1
arch=('x86_64')
license=('custom')
makedepends=('rustup')
provides=("otp-uinput=$pkgver")
source=("$pkgname::git+https://gitlab.com/jingjie/otp-uinput.git")
sha256sums=('SKIP')

build() {
  cd "$srcdir/$pkgname"
  cargo build --release
}

package() {
  cd "$srcdir/$pkgname"

  install -m755 "${srcdir}/$pkgname/target/release" "${pkgdir}/usr/bin/otp-uinput"
}
