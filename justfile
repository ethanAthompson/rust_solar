dev: 
	cargo run  

build:
	RUSTFLAGS="-C link-arg=-fuse-ld=lld" cargo build --release


