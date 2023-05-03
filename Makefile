dev:
	docker-compose up -d

dev-down:
	docker-compose down

start-server:
	cargo watch -q -c -w src/ -x run

install-watch:
	cargo install cargo-watch
