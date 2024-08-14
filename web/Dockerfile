# Stage 1: Build the Rust application
FROM rust:latest AS builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the source code into the working directory
COPY . /usr/src/app/

# Copy the local dependencies
COPY ../entity /usr/src/entity
COPY ../entity_api /usr/src/entity_api
# COPY ../service /usr/src/service

# Fetch the dependencies and build the application
RUN cargo build 
RUN cargo fetch
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
RUN addgroup -S refactorgroup && adduser -S refactoruser -G refactorgroup

# Set the working directory
WORKDIR /app

# Copy the binary from the build stage to the working directory
COPY --from=builder /usr/src/app/target/release/web .

# Change ownership of the binary to the non-root user
RUN chown refactoruser:refactorgroup /app/web

# Expose the port that the app listens on
EXPOSE 8000

# Switch to the non-root user
USER  refactoruser

# Set the command to run the application
CMD ["./web"]