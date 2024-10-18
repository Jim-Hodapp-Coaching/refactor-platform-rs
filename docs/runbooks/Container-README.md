# **Rust Web Application with PostgreSQL in Docker**

This repository contains a simple setup for deploying a Rust web application connected to a PostgreSQL database using Docker and Docker Compose. It allows for easy development and deployment of Rust services, database management utilities, and database migrations.

## **Prerequisites**

Before you begin, ensure you have the following installed:

- [Docker](https://www.docker.com/products/docker-desktop) (version 20+)
- [Docker Compose](https://docs.docker.com/compose/install/) (version 1.29+)

## **Project Setup**

1. **Clone the repository**:

   ```bash
   git clone <repository-url>
   cd <repository-directory>
   ```

2. **Review the `docker-compose.yaml` file**:
   The Docker Compose file sets up both the Rust application and a PostgreSQL container. The key environment variables such as `POSTGRES_USER`, `POSTGRES_PASSWORD`, `POSTGRES_DB`, and `WEB_PORT` can be customized via `.env` or directly in the `docker-compose.yaml`.

   ```yaml
   services:
     postgres:
       image: postgres:13
       environment:
         POSTGRES_USER: refactor
         POSTGRES_PASSWORD: password
         POSTGRES_DB: refactor_platform
       ports:
         - "5432:5432"

     rust-app:
       build: .
       environment:
         POSTGRES_USER: refactor
         POSTGRES_PASSWORD: password
         POSTGRES_DB: refactor_platform
         POSTGRES_HOST: postgres
       ports:
         - "8080:8080"
       depends_on:
         - postgres
   ```

## **Building and Running the Application**

### **Step 1: Build the Docker images**

From the project root, run the following command to build the Docker images for both the Rust application and PostgreSQL.

```bash
docker-compose build
```

### **Step 2: Start the application**

Once the build is complete, you can start the containers using Docker Compose. This will start the PostgreSQL database and the Rust application.

```bash
docker-compose up
```

You should now have the Rust web API running on `http://localhost:8080` and PostgreSQL running on port `5432`.

## **Using the Database Utilities**

### **Rebuild the database**

To rebuild the database (create a new database, user, and schema):

```bash
docker-compose run rust-app rebuild-db
```

This will use the default settings (`POSTGRES_DB=refactor_platform`, `POSTGRES_USER=refactor`, `POSTGRES_SCHEMA=refactor_platform`). You can customize these values by passing environment variables.

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

## **Customizing Environment Variables**

The default environment variables are defined in the `docker-compose.yaml` file, but you can also create a `.env` file in the project root to override them. Here are some commonly used variables:

- `POSTGRES_USER`: The username for the PostgreSQL database (default: `refactor`).
- `POSTGRES_PASSWORD`: The password for the PostgreSQL user (default: `password`).
- `POSTGRES_DB`: The name of the PostgreSQL database (default: `refactor_platform`).
- `POSTGRES_SCHEMA`: The schema name (default: `refactor_platform`).
- `POSTGRES_HOST`: The hostname for the PostgreSQL service (default: `postgres`).
- `WEB_PORT`: The port on which the web API will run (default: `8080`).

Example `.env` file:

```
POSTGRES_USER=myuser
POSTGRES_PASSWORD=mypassword
POSTGRES_DB=mydb
POSTGRES_SCHEMA=myschema
WEB_PORT=9090
```

## **Stopping the Containers**

To stop the containers, you can use the following command:

```bash
docker-compose down
```

If you want to remove all the containers, networks, and volumes, including the PostgreSQL data:

```bash
docker-compose down -v
```

## **Troubleshooting and Gotchas**

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

  3. Ensure that the `POSTGRES_HOST` environment variable in the Rust app is set to `postgres`, which is the name of the PostgreSQL service in Docker Compose.

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

## **Tips for Development**

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

## **Conclusion**

This setup provides a simple yet powerful way to run and manage a Rust web application connected to PostgreSQL using Docker and Docker Compose. By leveraging the power of Docker, you can easily handle database management tasks, utility commands, and scale your application as needed.
