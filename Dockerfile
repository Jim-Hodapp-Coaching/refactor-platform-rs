# Build stage
FROM rust:1.68-slim AS builder

# Set working directory
WORKDIR /app

# Install necessary build dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Cache dependencies to speed up builds
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

# Copy the source code
COPY . .

# Build all projects in release mode using workspace
RUN cargo build --release

# Final stage
FROM debian:bullseye-slim

# Install system dependencies including PostgreSQL libraries and Node.js
RUN apt-get update && apt-get install -y \
    libpq5 \
    logrotate \
    curl \
    nodejs \
    npm \
    && rm -rf /var/lib/apt/lists/*

# Install dbml2sql globally via npm
RUN npm install -g @dbml/cli

# Set working directory
WORKDIR /app

# Copy built binaries from builder stage
COPY --from=builder /app/target/release/web /app/web
COPY --from=builder /app/target/release/service /app/service
COPY --from=builder /app/target/release/entity_api /app/entity_api
COPY --from=builder /app/target/release/entity /app/entity

# Copy scripts for database management
COPY ./scripts/rebuild_db.sh /app/scripts/rebuild_db.sh

# Create non-root user to avoid permission issues
ARG USERNAME=appuser
ARG USER_UID=1000
ARG USER_GID=1000

RUN groupadd -g ${USER_GID} ${USERNAME} && \
    useradd -u ${USER_UID} -g ${USER_GID} -m ${USERNAME} && \
    chown -R ${USERNAME}:${USERNAME} /app

# Switch to non-root user
USER ${USERNAME}

# Environment variables for PostgreSQL connection (can be overridden at runtime)
ENV POSTGRES_USER=refactor \
    POSTGRES_PASSWORD=password \
    POSTGRES_DB=refactor_platform \
    POSTGRES_SCHEMA=refactor_platform \
    POSTGRES_HOST=postgres \
    DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:5432/${POSTGRES_DB}

# Expose configurable ports for web API and database
ARG WEB_PORT=8080
ARG POSTGRES_PORT=5432

EXPOSE ${WEB_PORT} ${POSTGRES_PORT}

# Health check to ensure the web API is running
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD curl --fail http://localhost:${WEB_PORT}/health || exit 1

# Set environment variables for custom command usage
ENV COMMANDS="help, rebuild-db, seed-db, dbml2sql"

# Use ENTRYPOINT to ensure proper signal handling
ENTRYPOINT ["/bin/sh", "-c", "if [ \"$1\" = 'rebuild-db' ]; then \
    /app/scripts/rebuild_db.sh ${POSTGRES_DB} ${POSTGRES_USER} ${POSTGRES_SCHEMA}; \
    elif [ \"$1\" = 'seed-db' ]; then \
    cargo run --bin seed_db; \
    elif [ \"$1\" = 'dbml2sql' ]; then \
    dbml2sql --database postgres --output /app/sql/structure.sql /app/schema.dbml; \
    else \
    /app/web $@; \
    fi"]
