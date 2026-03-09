.PHONY: up down build run shell logs restart clean

# Start all services in detached mode
up:
	docker compose up -d --build

# Stop all services
down:
	docker compose down

# Build the app container
build:
	docker compose build app

# Run the axum server inside the app container
run:
	docker compose exec app cargo run

watch:
	docker compose exec app cargo watch -x run

# Open a shell in the app container
shell:
	docker compose exec app bash

# Tail logs from the app container
logs:
	docker compose logs -f app

# Restart the app container
restart:
	docker compose restart app

# Stop all services and remove volumes
clean:
	docker compose down -v
