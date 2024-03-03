test:
	cargo test --no-fail-fast
test-cover:
	cargo tarpaulin --frozen --no-fail-fast --out Xml --skip-clean
test-watch:
	cargo watch -x "test -- --nocapture"
