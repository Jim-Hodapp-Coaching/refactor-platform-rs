# Description: This file contains the instructions to build the Docker image for the application.
# Author: Levi McDonough

# Stage 1: Build the Rust application
FROM --platform=arm64 rust:1.55 AS builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files to the working directory
COPY Cargo.toml Cargo.lock ./

# Create an empty src directory and build dependencies (on a single layer)
RUN mkdir src && \
    echo "fn main() {}" > src/main.rd && \
    cargo build --release && \
    rm -rf src

# Copy the source code to the working directory
COPY . .

# Build the Rust application
RUN cargo build --release


# Stage 2: Create a smaller image with only the build binary
FROM --platform=arm64 debian:bullseye-slim

# Set the working directory
WORKDIR /usr/srr/app

# Copy the binary from the building stage to the working directory
COPY --from=builder /usr/src/app/target/release/app .

# Expose the port that the application listens on
EXPOSE 8000

# Set the command to run the application
CMD ["./app"]
