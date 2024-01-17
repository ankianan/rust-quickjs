build-embed:
	(cd web/ && pnpm rollup:embed)

run-go:
	cargo run --release
