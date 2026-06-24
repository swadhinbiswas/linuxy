# Copyright 2026 Swadhin Biswas
# Distributed under the terms of the MIT License

EAPI=8

inherit cargo desktop xdg

DESCRIPTION="Multi-platform Desktop Application Manager with Firejail sandboxing"
HOMEPAGE="https://github.com/swadhinbiswas/linuxy"
SRC_URI="https://github.com/swadhinbiswas/${PN}/archive/refs/tags/v${PV}.tar.gz -> ${P}.tar.gz"

LICENSE="MIT"
SLOT="0"
KEYWORDS="~amd64 ~arm64"
IUSE=""

DEPEND="
	dev-libs/openssl
	dev-libs/glib:2
	net-libs/webkit-gtk:4.1
	x11-libs/gtk+:3
	x11-libs/libX11
	x11-libs/libxdo
	app-admin/firejail
	x11-misc/xdg-utils
	gnome-base/librsvg
	dev-libs/libappindicator:3
"
RDEPEND="${DEPEND}"
BDEPEND="
	>=virtual/rust-1.80
	net-libs/nodejs
	sys-devel/clang
"

QA_FLAGS_IGNORED="usr/bin/${PN}"

src_prepare() {
	default
	npm install
}

src_configure() {
	:
}

src_compile() {
	npm run build
	cd src-tauri
	cargo build --release
}

src_install() {
	dobin src-tauri/target/release/linuxy
	domenu src-tauri/debian/desktop-template.desktop
	doicon src-tauri/icons/icon.png

	for size in 32 128 256 512; do
		icon="src-tauri/icons/${size}x${size}.png"
		[ -f "$icon" ] && newicon -s ${size} "$icon" linuxy.png
	done

	dodoc README.md
	dolicense LICENSE
}

pkg_postinst() {
	xdg_desktop_database_update
	xdg_icon_cache_update
}

pkg_postrm() {
	xdg_desktop_database_update
	xdg_icon_cache_update
}
