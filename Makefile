PORTNAME=	sigi
DISTVERSIONPREFIX=	v
DISTVERSION=	3.0.3
CATEGORIES=	misc

MAINTAINER=	hiljusti@so.dang.cool
COMMENT=	Organizing tool for terminal lovers who hate organizing

LICENSE=	GPLv2

USES=		cargo
USE_GITHUB=	yes
GH_ACCOUNT=	hiljusti
CARGO_BUILD_ARGS=	--all

CARGO_CRATES=	atty-0.2.14 \
		autocfg-1.0.1 \
		bitflags-1.2.1 \
		chrono-0.4.19 \
		clap-3.1.6 \
		clap_derive-3.1.4 \
		hashbrown-0.11.2 \
		heck-0.4.0 \
		hermit-abi-0.1.18 \
		indexmap-1.8.0 \
		itoa-0.4.7 \
		json-0.12.4 \
		lazy_static-1.4.0 \
		libc-0.2.87 \
		memchr-2.4.1 \
		num-integer-0.1.44 \
		num-traits-0.2.14 \
		os_str_bytes-6.0.0 \
		proc-macro-error-1.0.4 \
		proc-macro-error-attr-1.0.4 \
		proc-macro2-1.0.36 \
		pure-rust-locales-0.5.6 \
		quote-1.0.9 \
		ryu-1.0.5 \
		serde-1.0.123 \
		serde_derive-1.0.123 \
		serde_json-1.0.64 \
		strsim-0.10.0 \
		syn-1.0.86 \
		termcolor-1.1.2 \
		textwrap-0.15.0 \
		time-0.1.44 \
		unicode-xid-0.2.1 \
		version_check-0.9.4 \
		wasi-0.10.0+wasi-snapshot-preview1 \
		winapi-0.3.9 \
		winapi-i686-pc-windows-gnu-0.4.0 \
		winapi-util-0.1.5 \
		winapi-x86_64-pc-windows-gnu-0.4.0

post-install:
	${INSTALL_MAN} ${WRKSRC}/${PORTNAME}.1 ${STAGEDIR}${PREFIX}/share/man/man1/

.include <bsd.port.mk>
