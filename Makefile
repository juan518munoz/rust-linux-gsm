.PHONY: front back all

# Load .env variables
include .env
export $(shell sed 's/=.*//' .env)

front:
	cargo run --manifest-path=frontend/Cargo.toml

back:
	cargo run --manifest-path=backend/Cargo.toml

all:
	cargo run --manifest-path=frontend/Cargo.toml & \
	cargo run --manifest-path=backend/Cargo.toml & \
	wait

kill:
	@echo "Killing process on port $(FRONTEND_PORT)..."
	@lsof -ti tcp:$(FRONTEND_PORT) | xargs -r kill
	@echo "Killing process on port $(BACKEND_PORT)..."
	@lsof -ti tcp:$(BACKEND_PORT) | xargs -r kill