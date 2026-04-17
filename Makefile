check:
	cargo fmt --all
	cargo clippy --all-features --tests -- -Dwarnings
	cargo test --all-features
	npm run format
	npm run lint
	npm run test
