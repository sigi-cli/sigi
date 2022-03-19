# Maintainer: J.R. Hill <hiljusti@so.dang.cool>
pkgname="sigi"
pkgver="3.0.3"
pkgrel=1
pkgdesc="Organization CLI for people who hate organization"
arch=('x86_64' 'aarch54')
url="https://github.com/hiljusti/sigi"
license=('GPL-2.0-only')
makedepends=('cargo')
source=("$pkgname-$pkgver.tar.gz::https://crates.io/api/v1/crates/sigi/$pkgver/download")
sha256sums=('6bb5e0f633d1209cb89e4d4a6c820e9519b5b6ed4d1ed05c989e423b241cf621')

build() {
	cd "$pkgname-$pkgver"

	cargo build --release --locked
}

check() {
	cd "$pkgname-$pkgver"

	SKIP_BATS_TESTS=1 cargo test --release --locked
}

package() {
	cd "$pkgname-$pkgver"

	install -Dm755 "target/release/sigi" "$pkgdir/usr/bin/sigi"

	install -Dm644 "sigi.1" "$pkgdir/usr/share/man/man1/sigi.1"
}