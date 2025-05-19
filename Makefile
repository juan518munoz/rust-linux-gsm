.PHONY: front back all kill

# Load .env variables
include .env
export $(shell sed 's/=.*//' .env)

front:
	RUST_LOG=$(RUST_LOG) cargo run --manifest-path=frontend/Cargo.toml

back:
	RUST_LOG=$(RUST_LOG) cargo run --manifest-path=backend/Cargo.toml

all:
	RUST_LOG=$(RUST_LOG) cargo run --manifest-path=frontend/Cargo.toml & \
	RUST_LOG=$(RUST_LOG) cargo run --manifest-path=backend/Cargo.toml & \
	wait

kill:
	@echo "Killing process on port $(FRONTEND_PORT)..."
	@lsof -ti tcp:$(FRONTEND_PORT) | xargs -r kill
	@echo "Killing process on port $(BACKEND_PORT)..."
	@lsof -ti tcp:$(BACKEND_PORT) | xargs -r kill

fmt:
	cargo fmt --all --manifest-path=frontend/Cargo.toml
	cargo fmt --all --manifest-path=backend/Cargo.toml