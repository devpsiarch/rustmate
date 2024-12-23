debug:
	cargo rustc -- -Awarnings
check:
	cargo check
run: 
	cargo build --release
	cargo run
