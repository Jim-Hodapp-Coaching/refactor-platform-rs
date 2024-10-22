# Stage 1: Build the application
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /app

# Copy the main Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the Cargo.toml files from each crate
COPY web/Cargo.toml web/Cargo.toml
COPY service/Cargo.toml service/Cargo.toml
COPY entity_api/Cargo.toml entity_api/Cargo.toml
COPY entity/Cargo.toml entity/Cargo.toml
COPY migration/Cargo.toml migration/Cargo.toml

# Build dependencies to cache them
RUN cargo fetch

# Copy the source code
COPY . .

# Build the application in release mode
RUN cargo build --release

# Stage 2: Create the runtime image
FROM debian:buster-slim

# Install necessary system dependencies
RUN apt-get update && apt-get install -y \
    libssl1.1 \
    ca-certificates \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the compiled binaries from the builder stage
COPY --from=builder /app/target/release/refactor_platform_rs /app/src/refactor_platform_rs
COPY --from=builder /app/target/release/seed_db /app/seed_db

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
