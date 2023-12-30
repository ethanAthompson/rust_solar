dev: 
	cargo run  

build:
	RUSTFLAGS="-C link-arg=-fuse-ld=lld" cargo build --release

up:
	update_repo.sh

load:
	cargo publish
