compile:
	cargo build --release --features ssr --bin srok

run:
	docker compose -f docker-compose.yml up --remove-orphans -d

clean:
	docker compose -f docker-compose.yml down --remove-orphans
