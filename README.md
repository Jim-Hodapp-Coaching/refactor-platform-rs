# Basic local DB setup and management

## Set Up Database (running commands against postgres server.)
```sql
--create user
CREATE USER refactor_rs WITH PASSWORD 'password';
--create schema
CREATE SCHEMA IF NOT EXISTS refactor_platform_rs;
--Check to see that the schema exists
SELECT schema_name FROM information_schema.schemata;
--Grant schema access to user
GRANT CREATE ON SCHEMA public TO refactor_rs;
```

## Generate new migration
```bash
sea-orm-cli migrate generate your_table_name
```

## Run migrations (Assumes database name is postgres)
```bash
DATABASE_URL=postgres://refactor_rs:password@localhost:5432/refactor_platform_rs sea-orm-cli migrate up -s refactor_platform_rs 
```

## Generate Entity from Database
```bash
 DATABASE_URL=postgres://refactor_rs:password@localhost:5432/refactor_platform_rs sea-orm-cli generate entity  -s refactor_platform_rs -o entity/src
```

# Project Directory Structure

`entity_api` - data operations on the various `Entity` models

`entity` - shape of the data models and the relationships to each other

`migration` - relational DB SQL migrations

`service` - CLI flags, environment variables, config handling and backend daemon setup

`src` - contains a main function that initializes logging and calls all sub-services

`web` - API endpoint definition, routing, handling of request/responses, controllers