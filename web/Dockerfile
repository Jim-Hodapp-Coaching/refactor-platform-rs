# Stage 1: Build the Rust application
FROM rust:1.55 AS builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files to the working directory
COPY . .

# Build the Rust Frontend Service
RUN cargo build --release


# Stage 2: Create a smaller image with only the build binary
FROM debian:buster-slim

# Copy the binary from the build stage to the working directory
COPY --from=builder /usr/src/app/target/release/app /usr/local/bin/app

# Expose the port that the application listens on
EXPOSE 8000

# Set the command to run the application
CMD ["app"]