# Use an official Rust runtime as a parent image
FROM rust:1.55

# Set the working directory
WORKDIR /usr/src/migration

# Copy the Cargo.toml and Cargo.lock files to the working directory
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the migration binary
RUN cargo build --release

# Set the command to run the migrations
CMD ["cargo", "run", "--release"]