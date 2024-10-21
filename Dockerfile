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

# Copy the Cargo.toml files and Cache dependencies to speed up builds
COPY Cargo.toml Cargo.lock ./
COPY web/Cargo.toml web/Cargo.toml
COPY service/Cargo.toml service/Cargo.toml
COPY entity_api/Cargo.toml entity_api/Cargo.toml
COPY entity/Cargo.toml entity/Cargo.toml
COPY migration/Cargo.toml migration/Cargo.toml

# Fetch dependencies (dependencies will be cached if Cargo.toml hasn't changed)
RUN cargo fetch

# Copy the source code into the container
COPY . .

# Build all projects in release mode using workspace
RUN cargo build --release

# Final stage
FROM debian:bullseye-slim

# Install necessary system dependencies
RUN apt-get update && apt-get install -y \
    libpq5 \
    logrotate \
    curl \
    nodejs \
    npm \
    libssl1.1 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install dbml2sql globally via npm
RUN npm install -g @dbml/cli

# Set working directory
WORKDIR /app

# Copy the compiled binaries from the builder stage for each component of the project
COPY --from=builder /app/target/release/refactor_platform_rs /app/refactor_platform_rs
COPY --from=builder /app/target/release/seed_db /app/seed_db
COPY --from=builder /app/target/release/web /app/web
COPY --from=builder /app/target/release/service /app/service
COPY --from=builder /app/target/release/entity_api /app/entity_api
COPY --from=builder /app/target/release/entity /app/entity

# Copy scripts for database management
COPY ./scripts/rebuild_db.sh /app/scripts/rebuild_db.sh

# Environment variables for Username, UID, and GID for the app user
ENV USERNAME=${USERNAME:-appuser}
ENV USER_UID=${USER_UID:-1000}
ENV USER_GID=${USER_GID:-1000}
# Set environment variables for database connection
ENV POSTGRES_USER=${POSTGRES_USER:-refactor}
ENV POSTGRES_PASSWORD=${POSTGRES_PASSWORD:-password}
ENV POSTGRES_DB=${POSTGRES_DB:-refactor_platform}
ENV POSTGRES_SCHEMA=${POSTGRES_SCHEMA:-public}
ENV POSTGRES_HOST=${POSTGRES_HOST:-localhost}
ENV DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:5432/${POSTGRES_DB}
ENV WEB_PORT=${WEB_PORT:-4000}
ENV SERVICE_PORT=${SERVICE_PORT:-4001}
ENV ENTITY_API_PORT=${ENTITY_API_PORT:-4002}

# Args for username, UID, and GID for the app user
ARG USERNAME=${USERNAME:-appuser}
ARG USER_UID=${USER_UID:-1000}
ARG USER_GID=${USER_GID:-1000}

# Create the app user and set appropriate permissions
RUN groupadd -g ${USER_GID} ${USERNAME} && \
    useradd -u ${USER_UID} -g ${USER_GID} -m ${USERNAME} && \
    chown -R ${USERNAME}:${USERNAME} /app

# Switch to the app user
USER ${USERNAME}

# Expose ports to the host
EXPOSE ${SERVICE_PORT}
EXPOSE ${ENTITY_API_PORT}
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
