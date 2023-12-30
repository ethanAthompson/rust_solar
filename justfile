dev: 
	cargo run  

build:
	RUSTFLAGS="-C link-arg=-fuse-ld=lld" cargo build --release

up:
	update_repo.sh

load:
	cargo publish


# In order to show documentation
#
# 1. Rename main.rs to lib.rs
# 2. Comment out main.rs
# 3. cargo publish again
# 4. it'll say its not a library, if so wait and do it again

