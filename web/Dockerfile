# Stage 1: Build the Rust application
FROM rust:1.55 AS builder

# Set the working directory
WORKDIR /usr/src/app

# Copy Cargo.toml and Cargo.lock and install dependencies
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

# Copy the rest of the code and build the application
COPY . .
RUN cargo build --release


# Stage 2: Create a smaller image with only the build binary
FROM alpine:3.14

# Add metadata
LABEL maintainer="Levi McDonough <levimcdonough@gmail.com>"
LABEL version="1.0"
LABEL description="The web service for the refactor platoform project."

# Install necessary runtime dependencies
RUN apk add --no-cache ca-certificates

# Create a non-root user and group
RUN addgroup -S appgroup && adduser -S refactoruser -G refactorgroup

# Set the working directory
WORKDIR /app

# Copy the binary from the build stage to the working directory
COPY --from=builer /usr/src/app/target/release/app .

# Change ownership of the binary to the non-root user
RUN chown refactoruser:refactorgroup /app/app

# Expose the port that the app listens on
EXPOSE 8000

# Switch to the non-root user
USER  refactoruser

# Set the command to run the application
CMD ["./app"]