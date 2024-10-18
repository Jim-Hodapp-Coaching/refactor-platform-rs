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

# Install PostgreSQL client libraries and other utilities
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

# Configurable non-root user, UID, and GID for the app user
ARG USERNAME=${USERNAME:-appuser}
ARG USER_UID=${USER_UID:-1000}
ARG USER_GID=${USER_GID:-1000}

# Create a non-root user and set appropriate permissions
RUN groupadd -g ${USER_GID} ${USERNAME} && \
    useradd -u ${USER_UID} -g ${USER_GID} -m ${USERNAME} && \
    chown -R ${USERNAME}:${USERNAME} /app

# Switch to non-root user
USER ${USERNAME}

# Expose environment variables for PostgreSQL connection
ENV POSTGRES_USER=${POSTGRES_USER:-refactor}
ENV POSTGRES_PASSWORD=${POSTGRES_PASSWORD:-password}
ENV POSTGRES_DB=${POSTGRES_DB:-refactor_platform}
ENV POSTGRES_HOST=${POSTGRES_HOST:-localhost}
ENV DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:5432/${POSTGRES_DB}

# Expose configurable ports for web API
ENV WEB_PORT=${WEB_PORT:-8080}
EXPOSE ${WEB_PORT}

# Use ENTRYPOINT to handle different commands like rebuild-db, seed-db, etc.
ENTRYPOINT ["/bin/sh", "-c", "if [ \"$1\" = 'rebuild-db' ]; then \
    /app/scripts/rebuild_db.sh ${POSTGRES_DB} ${POSTGRES_USER} ${POSTGRES_SCHEMA}; \
    elif [ \"$1\" = 'seed-db' ]; then \
    cargo run --bin seed_db; \
    elif [ \"$1\" = 'dbml2sql' ]; then \
    dbml2sql --database postgres --output /app/sql/structure.sql /app/schema.dbml; \
    else \
    /app/web $@; \
    fi"]
