# Makefile for managing the project

# Variables
DOCKER_COMPOSE = docker-compose
DB_CONTAINER = postgres
DB_USER = refactor
DB_PASSWORD = password
DB_NAME = refactor_platform
DB_URL = postgres://$(DB_USER):$(DB_PASSWORD)@localhost:5432/$(DB_NAME)

# Default target
.PHONY: help
help: ## Show this help message
    @echo "Usage: make [target]"
    @echo ""
    @echo "Targets:"
    @awk 'BEGIN {FS = ":.*##"; printf "\033[36m%-20s\033[0m %s\n", "Target", "Description"} /^[a-zA-Z_-]+:.*?##/ { printf "\033[36m%-20s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST)

.PHONY: up
up: ## Start all services using Docker Compose
    $(DOCKER_COMPOSE) up -d

.PHONY: down
down: ## Stop all services using Docker Compose
    $(DOCKER_COMPOSE) down

.PHONY: rebuild-db
rebuild-db: ## Rebuild the database using the provided shell script
    ./scripts/rebuild_db.sh

.PHONY: migrate
migrate: ## Run database migrations
    DATABASE_URL=$(DB_URL) sea-orm-cli migrate up -s $(DB_NAME)

.PHONY: seed-db
seed-db: ## Seed the database with test data
    cargo run --bin seed_db

.PHONY: logs
logs: ## Show logs for all services
    $(DOCKER_COMPOSE) logs -f

.PHONY: ps
ps: ## List all running services
    $(DOCKER_COMPOSE) ps

.PHONY: build
build: ## Build all Docker images
    $(DOCKER_COMPOSE) build

.PHONY: clean
clean: ## Remove all stopped containers and unused images
    docker system prune -f

.PHONY: generate-entity
generate-entity: ## Generate a new Entity from the database
    DATABASE_URL=$(DB_URL) sea-orm-cli generate entity -s $(DB_NAME) -o entity/src -v --with-serde both --serde-skip-deserializing-primary-key --ignore-tables {table to ignore} --ignore-tables {other table to ignore}