### Technical Spec for Deploying the Rust-based Coaching Platform

---

<details>
    <summary>Ticket Contents</summary><br>

**Objective:** Create a detailed technical spec to set up the Rust-based coaching platform from scratch. The platform includes two Rust applications and a PostgreSQL database, all managed via Docker Compose. The guide will cover everything from installing necessary tools, building Dockerfiles, setting up Docker Compose with a named network and persistent volumes, and ensuring all containers run smoothly.

</details>

<details>
    <summary>Summary</summary><br>

*This technical spec provides step-by-step instructions to deploy the Rust-based coaching platform, including the Rust applications and PostgreSQL database, using Docker Compose. The setup involves installing necessary tools, creating Dockerfiles, configuring Docker Compose, and running the services with persistent volumes and a named network for easy management and inspection.*
</details>

<details>
    <summary>I. Install Necessary Tools</summary><br>

### For macOS

### Sub Task 1: Install Homebrew

1. **Install Homebrew** (if not already installed):

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

## Sub Task 2: Install Docker, Docker Compose, and PostgreSQL

1. **Install Docker**:

```bash
brew install --cask docker
```

1. **Install Docker Compose**:

```bash
brew install docker-compose
```

1. **Install PostgreSQL**:

```bash
brew install postgresql
```

### Sub Task 3: Install Rust

1. **Install Rust** using `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Sub Task 4: Install dbml2sql Tool

1. **Install dbml2sql** using npm:

```bash
npm install -g @dbml/cli
```

### For Linux (Ubuntu/Debian)

**Sub Task 1: Update Package List**

1. **Update the package list**:

```bash
sudo apt update
```

**Sub Task 2: Install Docker, Docker Compose, and PostgreSQL**

1. **Install Docker**:

```bash
sudo apt install docker.io
```

2. **Install Docker Compose**:

```bash
sudo apt install docker-compose
```

3. **Install PostgreSQL**:

```bash
sudo apt install postgresql postgresql-contrib
```

**Sub Task 3: Install Rust**

1. **Install Rust** using `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Sub Task 4: Install dbml2sql Tool**

1. **Install Node.js** (required for `dbml2sql`):

```bash
sudo apt install nodejs npm
```

2. **Install dbml2sql** using npm:

```bash
sudo npm install -g @dbml/cli
```

</details>

<details>
    <summary>II. Build Dockerfiles for the Rust Applications and PostgreSQL</summary><br>

**Sub Task 1: Create Dockerfile for the Rust Applications**

1. **Create a `Dockerfile`** for the Rust application (both frontend and backend apps will use similar configurations).

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

*Explanation*: This multi-stage Dockerfile builds the Rust application in a clean environment and then creates a smaller runtime image to run the application.

**Sub Task 2: Create Dockerfile for the PostgreSQL Database**

1. **Create a `Dockerfile`** for PostgreSQL.

```Dockerfile
# PostgreSQL Dockerfile
FROM postgres:13

ENV POSTGRES_USER=refactor
ENV POSTGRES_PASSWORD=password
ENV POSTGRES_DB=refactor_platform

EXPOSE 5432
CMD ["postgres"]
```

*Explanation*: This Dockerfile uses the official PostgreSQL image and sets up the database with environment variables for user, password, and database name.

</details>

<details>
    <summary>III. Create Docker Compose File</summary><br>

**Sub Task 1: Set Up `docker-compose.yml`**

1. **Create a `docker-compose.yml`** file in the root directory.

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

*Explanation*: This Docker Compose file sets up a multi-service environment with PostgreSQL and the Rust application. It ensures that services are isolated but can communicate through a named network.

</details>

<details>
    <summary>IV. Build Docker Images</summary><br>

**Sub Task 1: Build All Docker Images**

1. **Run the following command** to build the Docker images:

```bash
docker-compose build
```

*Explanation*: This command builds the images for all services as defined in the `docker-compose.yml` file, ensuring everything is ready for deployment.

</details>

<details>
    <summary>V. Run the Containers</summary><br>

**Sub Task 1: Start the Containers**

1. **Run the following command** to start all services in detached mode:

```bash
docker-compose up -d
```

*Explanation*: This command starts the PostgreSQL and Rust application containers, ensuring they run in the background.

**Sub Task 2: Monitor Logs and Verify**

1. **Check the logs** to ensure all services are running smoothly:

```bash
docker-compose logs -f
```

*Explanation*: Monitoring logs helps in troubleshooting any issues that might arise during startup.

</details>

<details>
    <summary>VI. Verify the Setup</summary><br>

**Sub Task 1: Verify PostgreSQL Setup**

1. **Access PostgreSQL** to ensure the database is set up correctly:

```bash
docker exec -it <postgres_container_id> psql -U refactor -d refactor_platform
```

*Explanation*: This command connects to the PostgreSQL container for direct database inspection.

**Sub Task 2: Verify Rust Application**

1. **Test the Rust application** by visiting `http://localhost:8080` or using `curl`.

*Explanation*: This step ensures that the Rust application is up and running as expected.

</details>

<details>
    <summary>VII. Set Up Local Database</summary><br>

**Sub Task 1: Run Database Setup Script**

1. **Execute the database setup script** to initialize the database:

```bash
./scripts/rebuild_db.sh
```

*Explanation*: This script sets up the database schema and seeds it with necessary data.

**Sub Task 2: Run Migrations**

1. **Execute database migrations** to apply the latest schema:

```bash
docker exec -it <app_container_id> bash -c "DATABASE_URL=postgres://refactor:password@postgres:5432/refactor_platform sea-orm-cli migrate up -s refactor_platform"
```

*Explanation*: Running migrations ensures the database is up-to-date with the latest schema changes.

</details>

<details>
    <summary>VIII. Create a Makefile for Common Commands</summary><br>

**Sub Task 1: Create a `Makefile`** in the root directory for easier management of Docker commands.

```Makefile
.PHONY: build up down logs psql migrate rebuild_db

build:
    docker-compose build

up:
    docker-compose up -d

down:
    docker-compose down

logs:
    docker-compose logs -f

psql:
    docker exec -it postgres psql -U refactor -d refactor_platform

migrate:
    docker exec -it app bash -c "DATABASE_URL=postgres://refactor:password@postgres:5432/refactor_platform sea-orm-cli migrate up -s refactor_platform"

rebuild_db:
    ./scripts/rebuild_db.sh
```

*Explanation*: This `Makefile` provides a simple CLI interface for managing Docker commands,

<details>
    <summary>VIII. Create a Makefile for Common Commands (continued)</summary><br>

*Explanation*: This `Makefile` provides a simple CLI interface for managing Docker commands, making it easier to build images, start and stop containers, view logs, access the PostgreSQL database, run migrations, and rebuild the database. Here's a breakdown of the commands:

- **`build`:** Builds all Docker images defined in the `docker-compose.yml` file.
- **`up`:** Starts all services defined in the `docker-compose.yml` file in detached mode.
- **`down`:** Stops and removes all running containers.
- **`logs`:** Follows the logs of all running containers.
- **`psql`:** Accesses the PostgreSQL database in the running container.
- **`migrate`:** Runs database migrations using `sea-orm-cli` inside the Rust application container.
- **`rebuild_db`:** Executes the database setup script to initialize or reset the database schema.

</details>

<details>
    <summary>IX. Final Review and Testing</summary><br>

**Sub Task 1: Review the Setup**

1. **Go over the setup** to confirm that all steps are correctly documented and the system is fully functional.

*Explanation*: Reviewing the setup ensures that all configurations and implementations align with the project requirements and that the documentation accurately reflects the necessary steps.

**Sub Task 2: Final Testing**

1. **Test the entire system** in a real-world scenario to ensure everything works as intended.

*Explanation*: Final testing helps catch any missed issues and ensures the setup is production-ready.

</details>

<details>
    <summary>X. Common Gotchas</summary><br>

- **Database Connection Issues**: Ensure that your PostgreSQL server is running and accessible.
- **Docker Permission Denied**: If you encounter permission errors with Docker, run `sudo usermod -aG docker $USER` and restart your terminal.
- **Container Dependencies**: If containers fail to start in the correct order, ensure the `depends_on` directive is correctly set in the `docker-compose.yml` file.

</details>

<details>
    <summary>XI. Definition of Done</summary><br>

- The Rust applications and PostgreSQL database are successfully built, started, and accessible via their respective ports.
- All containers run in a named network with persistent volumes for PostgreSQL data.

- The entire setup is managed and deployable via Docker Compose.

- A `Makefile` provides a simple CLI interface for managing common Docker and database tasks.

- Documentation is clear and suitable for users with minimal technical experience.

</details>
