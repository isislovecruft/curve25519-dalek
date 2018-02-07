FEATURES := nightly yolocrypto
TEST_EXECUTABLE := $(shell find target/debug/ -executable -regex "target/debug/curve25519_dalek-[0-9a-f]*")

# See http://sunjay.ca/2016/07/25/rust-code-coverage for installing up a new enough kcov
coverage: clean
	cargo test --features "$(FEATURES)" --no-run
	kcov --exclude-pattern=/.cargo,/usr/lib --verify target/cov "$(TEST_EXECUTABLE)"

clean:
	cargo clean

doc:
	cargo rustdoc --features "$(FEATURES)" -- --html-in-header rustdoc-include-katex-header.html

doc-internal:
	cargo rustdoc --features "$(FEATURES)" -- --html-in-header rustdoc-include-katex-header.html --document-private-items

