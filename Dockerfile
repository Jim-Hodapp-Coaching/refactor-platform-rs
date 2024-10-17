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

# Install PostgreSQL client libraries and logrotate
RUN apt-get update && apt-get install -y \
    libpq5 \
    logrotate \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy built binaries from builder stage
COPY --from=builder /app/target/release/web /app/web
COPY --from=builder /app/target/release/service /app/service
COPY --from=builder /app/target/release/entity_api /app/entity_api
COPY --from=builder /app/target/release/entity /app/entity

# Create non-root user early to avoid permission issues
RUN groupadd -g 1000 appgroup && \
    useradd -u 1000 -g appgroup appuser && \
    chown -R appuser:appgroup /app

# Switch to non-root user
USER appuser

# Expose necessary ports
EXPOSE 8080 5432

# Health check
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 CMD curl --fail http://localhost:8080/health || exit 1

# Use ENTRYPOINT to ensure proper signal handling
ENTRYPOINT ["/app/web"]
