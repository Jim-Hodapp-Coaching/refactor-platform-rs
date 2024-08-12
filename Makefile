# Makefile

.PHONY: help install_postgresql install_dbml2sql rebuild_db seed_db setup_db_manual run_migrations generate_entity

help:
    @echo "Available commands:"
    @echo "  install_postgresql    Install PostgreSQL using Homebrew"
    @echo "  install_dbml2sql      Install dbml2sql tool using npm"
    @echo "  rebuild_db            Run the rebuild_db.sh script with default settings"
    @echo "  seed_db               Seed the database with test data"
    @echo "  setup_db_manual       Set up the database manually using SQL commands"
    @echo "  run_migrations        Run database migrations"
    @echo "  generate_entity       Generate a new entity from the database"

install_postgresql:
    brew install postgresql

install_dbml2sql:
    npm install -g @dbml/cli

rebuild_db:
    ./scripts/rebuild_db.sh

seed_db:
    cargo run --bin seed_db

setup_db_manual:
    psql -U postgres -c "CREATE DATABASE refactor_platform;"
    psql -U postgres -d refactor_platform -c "CREATE USER refactor WITH PASSWORD 'password';"
    psql -U postgres -d refactor_platform -c "CREATE SCHEMA IF NOT EXISTS refactor_platform AUTHORIZATION refactor;"
    psql -U postgres -d refactor_platform -c "GRANT ALL PRIVILEGES ON SCHEMA refactor_platform TO refactor;"

run_migrations:
    DATABASE_URL=postgres://refactor:password@localhost:5432/refactor_platform sea-orm-cli migrate up -s refactor_platform

generate_entity:
    DATABASE_URL=postgres://refactor:password@localhost:5432/refactor_platform sea-orm-cli generate entity -s refactor_platform -o entity/src -v --with-serde both --serde-skip-deserializing-primary-key --ignore-tables {table to ignore} --ignore-tables {other table to ignore}