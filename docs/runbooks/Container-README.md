# Refactor Coaching & Mentoring Platform with Docker & Docker Compose

*This project is a Rust-based backend/web API that connects to a PostgreSQL database. It uses Docker and Docker Compose for easy local development and deployment, and includes utilities for database management, migrations, and more. You can choose to run PostgreSQL either locally (via Docker) or remotely by configuring the environment variables.*

## Prerequisites

Before you begin, ensure you have the following installed:

- [Docker](https://www.docker.com/products/docker-desktop) (version 20+)
- [Docker Compose](https://docs.docker.com/compose/install/) (version 1.29+)

## Project Setup

1. **Clone the repository**:

   ```bash
   git clone <repository-url>
   cd <repository-directory>
   ```

2. **Environment Configuration**:

   You'll need to decide whether you're connecting to a **local PostgreSQL container** (using Docker) or a **remote PostgreSQL instance** (e.g., on a different host or in the cloud). This is configured using `.env` files.

   - **For local PostgreSQL** (default, Docker-based):
     Create a `.env.local` file based on the template provided below and specify `POSTGRES_HOST=postgres`.

   - **For remote PostgreSQL**:
     Create a `.env.remote-db` file and set `POSTGRES_HOST` to the external IP or hostname of the remote PostgreSQL instance.

   Example `.env.local` for local development:
  
   ```env
   POSTGRES_USER=refactor
   POSTGRES_PASSWORD=password
   POSTGRES_DB=refactor_platform
   POSTGRES_HOST=postgres
   POSTGRES_PORT=5432
   WEB_PORT=4000
   USERNAME=appuser
   USER_UID=1000
   USER_GID=1000
   ```

   Example `.env.remote-db` for remote PostgreSQL:

   ```env
   POSTGRES_USER=remote_user
   POSTGRES_PASSWORD=remote_password
   POSTGRES_DB=remote_db
   POSTGRES_HOST=remote-db-host.com
   POSTGRES_PORT=5432
   WEB_PORT=8080
   USERNAME=remote_appuser
   USER_UID=1001
   USER_GID=1001
   ```

3. **Review the `docker-compose.yaml` file**:

   The Docker Compose file is configured to use environment variables defined in your `.env` files. The PostgreSQL container can either run locally (if specified in your environment file) or you can connect to a remote instance by setting the appropriate `POSTGRES_HOST`.

   ```yaml
   services:
     postgres:
       image: postgres:17
       environment:
         POSTGRES_USER: ${POSTGRES_USER}
         POSTGRES_PASSWORD: ${POSTGRES_PASSWORD}
         POSTGRES_DB: ${POSTGRES_DB}
       ports:
         - "${POSTGRES_PORT}:5432"
       volumes:
         - postgres_data:/var/lib/postgresql/data

     rust-app:
       build: .
       environment:
         POSTGRES_USER: ${POSTGRES_USER}
         POSTGRES_PASSWORD: ${POSTGRES_PASSWORD}
         POSTGRES_DB: ${POSTGRES_DB}
         POSTGRES_HOST: ${POSTGRES_HOST}
       ports:
         - "${WEB_PORT}:4000"
       depends_on:
         - postgres
   ```

## Building and Running the Application

### **1. Building the rust-backend image**

To build the Docker image for the Rust application, run the following command:
  
  ```bash
  docker buildx build --platform linux/amd64,linux/arm64 -t rust-backend . 
  ```

- this will build the image for both amd64 and arm64 architectures or you can choose to build for a specific architecture by specifying the `--platform` flag.
- the image will be tagged as `rust-backend` and will be used in the `docker-compose.yaml` file.

### **2. Running the Rust Backend Application**

  **Interactive Shell Access to Functions:** When you start the container the default behavior is to enter an interactive session, both the binaries and each of the functions from rebuild_db.sh are immediately available:

  ```bash
  docker run -it rns-backend:latest
  ```

- Inside the container shell, you can directly call any function from `rebuild_db.sh` directly by name:
- You can also run any of the binaries or functions from `rebuild_db.sh` directly by name as arguments to the `docker run` command:

```bash
docker run -it rns-backend:latest seed_db
```

## Building and Running the Entire Application

To build and run the entire application, including the Rust backend and PostgreSQL database, follow these steps:

### **Step 1: Build the Docker images**

From the project root, run the following command to build the Docker images for both the Rust application and PostgreSQL (if running locally).

```bash
docker-compose --env-file .env.local build
```

or for remote PostgreSQL:

```bash
docker-compose --env-file .env.remote-db build
```

### **Step 2: Start the application**

Once the build is complete, start the containers using Docker Compose. This will start the PostgreSQL database (if local) and the Rust application.

For local PostgreSQL:

```bash
docker-compose --env-file .env.local up
```

For remote PostgreSQL:

```bash
docker-compose --env-file .env.remote-db up
```

The Rust web API should now be running on `http://localhost:8080` and PostgreSQL should be available on port `5432` (or remotely if using the `.env.remote-db` setup).

## Using the Database Utilities

### **Rebuild the database**

To rebuild the database (create a new database, user, and schema):

```bash
docker-compose run rust-app rebuild-db
```

This will use the default settings from your environment variables (`POSTGRES_DB`, `POSTGRES_USER`, and `POSTGRES_SCHEMA`).

### **Seed the database**

To seed the database with test data:

```bash
docker-compose run rust-app seed-db
```

This runs a Rust service that seeds the PostgreSQL database with predefined data.

### **Convert DBML to SQL**

If you have a DBML file (`schema.dbml`), you can convert it into an SQL schema:

```bash
docker-compose run -v $(pwd)/sql:/app/sql -v $(pwd)/schema.dbml:/app/schema.dbml rust-app dbml2sql
```

This will output the generated SQL file in the `sql` directory.

## Stopping the Containers

To stop the containers, you can use the following command:

```bash
docker-compose down
```

If you want to remove all the containers, networks, and volumes, including the PostgreSQL data:

```bash
docker-compose down -v
```

## Troubleshooting and Gotchas

### **1. Cannot connect to PostgreSQL**

- **Problem**: The Rust application cannot connect to the PostgreSQL container.
- **Solution**:
  1. Ensure that PostgreSQL is running by checking the container status:

     ```bash
     docker-compose ps
     ```

  2. Check the PostgreSQL logs to see if it's running correctly:

     ```bash
     docker-compose logs postgres
     ```

  3. Ensure that the `POSTGRES_HOST` environment variable in the Rust app is set to `postgres` (for local) or to the correct remote hostname.

### **2. Web API not accessible**

- **Problem**: The web API is not accessible on `localhost:8080`.
- **Solution**:
  1. Verify that the container is running:

     ```bash
     docker-compose ps
     ```

  2. Check if the correct port is exposed:

     ```bash
     docker-compose logs rust-app
     ```

  3. If you have changed the port (e.g., `WEB_PORT=9090`), make sure you access the web API at `http://localhost:9090`.

### **3. Port conflicts**

- **Problem**: The default ports `8080` for the web API or `5432` for PostgreSQL are already in use by another service.
- **Solution**: Change the ports in the `.env` file or in `docker-compose.yaml` to different, unused ports:

  ```yaml
  services:
    postgres:
      ports:
        - "5433:5432"

    rust-app:
      ports:
        - "9090:8080"
  ```

### **4. Rebuild the application after changes**

- **Problem**: You made changes to the Rust code or the Dockerfile, but they are not reflected when you restart the container.
- **Solution**: Make sure to rebuild the Docker image after code changes:

  ```bash
  docker-compose build
  docker-compose up
  ```

### **5. Database persistence**

- **Problem**: The database resets every time the containers are stopped and restarted.
- **Solution**: Docker volumes are used to persist data between container restarts. Ensure that the volume `postgres_data` is properly configured in `docker-compose.yaml`:

  ```yaml
  volumes:
    postgres_data:
  ```

  If the data is still being wiped, make sure you are not running `docker-compose down -v` unless you want to remove the database volume.

## Tips for Development

- You can bring up the containers in detached mode by running:

  ```bash
  docker-compose up -d
  ```

- To access the running containers (e.g., for debugging):

  ```bash
  docker exec -it <container_name> bash
  ```

  Example to access the Rust app container:

  ```bash
  docker exec -it rust-app bash
  ```

- If you want to restart only one service (e.g., `rust-app`):

  ```bash
  docker-compose restart rust-app
  ```

- To stop all containers:

  ```bash
  docker-compose down
  ```

To stop and remove all containers, networks, and volumes:
  
  ```bash
  docker-compose down -v
  ```

---

## Working with the Rust application:

- To run the container with `rebuild_db.sh` (or any other binary) interactively, you can launch the container with a specific `CMD` override or call it directly:

  ```bash
    # Run the container and execute the shell script
    docker run -it --entrypoint ./rebuild_db.sh my-image-name:latest
  ```

## Testing and Debugging in Interactive Mode

- To debug or test interactively with all scripts and binaries in place:

  ```bash
  # Run the container interactively
  docker run -it --entrypoint /bin/bash my-image-name:latest
  ```

- Inside the container, you can manually check and execute:
  
  ```bash
  # Manually run the shell script
  ./rebuild_db.sh
  # Manually run other binaries
  ./target/release/seed_db
  ./target/release/refactor_platform_rs
  ```
