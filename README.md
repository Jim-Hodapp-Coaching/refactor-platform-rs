[![Build & Tests (backend)](https://github.com/Jim-Hodapp-Coaching/refactor-platform-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Jim-Hodapp-Coaching/refactor-platform-rs/actions/workflows/ci.yml)

# Refactor Coaching & Mentoring Platform
### Backend

## Intro

A Rust-based backend that provides a web API for various client applications (e.g. a web frontend) that facilitate the coaching and mentoring of software engineers.

The platform itself is useful for professional independent coaches, informal mentors and engineering leaders who work with individual software engineers and/or teams by providing a single application that facilitates and enhances your coaching practice.

## Basic Local DB Setup and Management

### Set Up Database

Note: these are commands meant to run against a real Postgresql server.

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

### Generate a New Migration
```bash
sea-orm-cli migrate generate your_table_name
```

### Run Migrations

Note: this assumes a database name of `refactor_platform_rs`

```bash
DATABASE_URL=postgres://refactor_rs:password@localhost:5432/refactor_platform_rs sea-orm-cli migrate up -s refactor_platform_rs 
```

### Generate Entity from Database
```bash
 DATABASE_URL=postgres://refactor_rs:password@localhost:5432/refactor_platform_rs sea-orm-cli generate entity  -s refactor_platform_rs -o entity/src
```

## Project Directory Structure

`entity_api` - data operations on the various `Entity` models

`entity` - shape of the data models and the relationships to each other

`migration` - relational DB SQL migrations

`service` - CLI flags, environment variables, config handling and backend daemon setup

`src` - contains a main function that initializes logging and calls all sub-services

`web` - API endpoint definition, routing, handling of request/responses, controllers
