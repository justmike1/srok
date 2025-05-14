ensure-cross = \
	command -v cross >/dev/null 2>&1 || { \
		echo "'cross' not found. Installing with 'cargo install cross'..."; \
		cargo install cross; \
	}

compile:
	cargo build --release --features ssr --bin srok

compile-amd64:
	$(ensure-cross)
	cross build --release --target x86_64-unknown-linux-gnu --features ssr --bin srok

compile-arm64:
	$(ensure-cross)
	cross build --release --target aarch64-unknown-linux-gnu --features ssr --bin srok

run:
	docker compose -f docker-compose.yml up --remove-orphans -d

clean:
	docker compose -f docker-compose.yml down --remove-orphans
