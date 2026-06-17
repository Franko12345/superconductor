
.PHONY: main

main:
	cd ui/main-window \
		&& npm run build
	GDK_BACKEND=x11 WEBKIT_DISABLE_COMPOSITING_MODE=1 WEBKIT_DISABLE_DMABUF_RENDERER=1 RUST_LOG=info cargo run

all:
	cd ui/main-window \
		&& npm run build
	cd ui/stdout \
		&& npm run build
	GDK_BACKEND=x11 WEBKIT_DISABLE_COMPOSITING_MODE=1 WEBKIT_DISABLE_DMABUF_RENDERER=1 RUST_LOG=info cargo run

setup:
	cd ui/main-window \
		&& npm install
	cd ui/stdout \
		&& npm install

frontend:
	cd ui/main-window \
		&& npm run build
	cd ui/stdout \
		&& npm run build

release:
	cd ui/main-window \
		&& npm run build
	cd ui/stdout \
		&& npm run build
	cargo build --release
