# Deploying the Coaching Platform Locally

## Getting Started

*This will guide you through the process of setting up the Rust-based coaching platform from scratch, including building Docker images, configuring Docker Compose, and running the application locally.*

### Prerequisites

1. **Install Docker and Docker Compose**

   Ensure Docker and Docker Compose are installed on your system. On macOS, install them via Homebrew:

    ```bash
    brew install --cask docker
    brew install docker-compose
    ```

2. **Install PostgreSQL**

   You will also need PostgreSQL installed for the database. On macOS, install it via Homebrew:

    ```bash
    brew install postgresql
    ```

3. **Install Rust**

   If Rust is not already installed, install it using rustup:

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

4. **Install dbml2sql Tool**

   Install the dbml2sql tool using npm:

    ```bash
    npm install -g @dbml/cli
    ```

### Step 1: Build Dockerfiles

First, create Dockerfiles for the Rust application and any other services. Here's an example `Dockerfile` for the Rust application:

**`Dockerfile` for Rust Application:**

```Dockerfile
# Stage 1: Build the Rust application
FROM rust:latest AS builder
WORKDIR /usr/src/app

# Copy the entire source code into the container
COPY . .

# Build the application
RUN cargo install --path .

# Stage 2: Create a smaller image with only the built binary
FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/app /usr/local/bin/app

# Set the command to run the binary
CMD ["app"]
```

### Step 2: Create Docker Compose File

Next, create a `docker-compose.yml` file that defines all the services, including the Rust application and PostgreSQL database.

**`docker-compose.yml`:**

```yaml
version: '3.8'

# Define a named network for all services to communicate
networks:
  coaching_network:

# Define persistent volumes for PostgreSQL data
volumes:
  postgres_data:

services:
  # PostgreSQL database service
  postgres:
    image: postgres:13
    environment:
      POSTGRES_USER: refactor
      POSTGRES_PASSWORD: password
      POSTGRES_DB: refactor_platform
    volumes:
      - postgres_data:/var/lib/postgresql/data
    networks:
      - coaching_network
    ports:
      - "5432:5432"
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U refactor"]
      interval: 10s
      timeout: 5s
      retries: 5

  # Rust application service
  app:
    build:
      context: .
      dockerfile: Dockerfile
    environment:
      DATABASE_URL: "postgres://refactor:password@postgres:5432/refactor_platform"
    networks:
      - coaching_network
    depends_on:
      postgres:
        condition: service_healthy
    ports:
      - "8080:8080"
```

### Step 3: Build Docker Images

Now that your Dockerfiles and `docker-compose.yml` are ready, build the Docker images:

```bash
docker-compose build
```

This command will build the images for all the services defined in the `docker-compose.yml` file.

### Step 4: Run the Containers

Start the containers using Docker Compose:

```bash
docker-compose up -d
```

This command will:

- Start the PostgreSQL container, create the necessary database, and expose it on port 5432.
- Start the Rust application container once the PostgreSQL service is healthy.
- Expose the Rust application on port 8080.

To see the logs and ensure everything is running smoothly:

```bash
docker-compose logs -f
```

### Step 5: Verify the Setup

To verify that everything is working as expected, you can:

1. **Check PostgreSQL**: Connect to the PostgreSQL container and verify the database setup.

    ```bash
    docker exec -it <postgres_container_id> psql -U refactor -d refactor_platform
    ```

2. **Check Rust Application**: Visit `http://localhost:8080` in your browser or use `curl` to verify the Rust application is running and responding to requests.

### Step 6: Local Database Setup

If you haven't already initialized the database schema, follow these steps:

1. **Run the Database Setup Script**

   Use the provided script to set up your database. This creates the necessary database, user, and schema:

    ```bash
    ./scripts/rebuild_db.sh
    ```

### Common Gotchas

- **Database Connection Issues**: Ensure that your PostgreSQL server is running and accessible.
- **Docker Permission Denied**: If you encounter permission errors with Docker, run `sudo usermod -aG docker $USER` and restart your terminal.
- **Container Dependencies**: If containers fail to start in the correct order, ensure the `depends_on` directive is correctly set in the `docker-compose.yml` file.
