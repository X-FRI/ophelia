build:
	@cargo build --release
	@cp target/release/ophelia .

fmt:
	@cargo fmt --all -- --check

push: fmt
	git push