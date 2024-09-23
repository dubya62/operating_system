.PHONY: user

user: user/hello

user/%: src/bin/%.rs Makefile
	cargo rustc --release --bin $* -- \
		-C linker-flavor=ld \
		-C link-args="-Ttext-segment=5000000 -Trodata-segment=5100000" \
		-C relocation-model=static
	mkdir -p user
	cp target/x86_64-operating_system/release/$* user/

run: user
	cargo run
