[![Build & Tests (backend)](https://github.com/Jim-Hodapp-Coaching/refactor-platform-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Jim-Hodapp-Coaching/refactor-platform-rs/actions/workflows/ci.yml)

# Refactor Coaching & Mentoring Platform
### Backend

## Intro

A Rust-based backend that provides a web API for various client applications (e.g. a web frontend) that facilitate the coaching and mentoring of software engineers.

The platform itself is useful for professional independent coaches, informal mentors and engineering leaders who work with individual software engineers and/or teams by providing a single application that facilitates and enhances your coaching practice.

## Basic Local DB Setup and Management

### Set Up Database

Note: these are commands meant to run against a real Postgresql server with an admin level user.

```sql
--create new database `refactor_platform`
CREATE DATABASE refactor_platform;
```

Change to the refactor_platform DB visually if using app like Postico, otherwise change using the
Postgresql CLI:

```sh
\c refactor_platform
```

```sql
--create new database user `refactor`
CREATE USER refactor WITH PASSWORD 'password';
--create a new schema owned by user `refactor`
CREATE SCHEMA IF NOT EXISTS refactor_platform AUTHORIZATION refactor;
--Check to see that the schema `refactor_platform` exists in the results
SELECT schema_name FROM information_schema.schemata;
--Grant all privileges on schema `refactor_platform` to user `refactor`
GRANT ALL PRIVILEGES ON SCHEMA refactor_platform TO refactor;
```

### Run Migrations

Note: this assumes a database name of `refactor_platform`

```bash
DATABASE_URL=postgres://refactor:password@localhost:5432/refactor_platform sea-orm-cli migrate up -s refactor_platform
```

### Generate a new Entity from Database
```bash
 DATABASE_URL=postgres://refactor:password@localhost:5432/refactor_platform sea-orm-cli generate entity  -s refactor_platform -o entity/src
```

## Project Directory Structure

`docs` - project documentation including architectural records, DB schema, API docs, etc

`entity_api` - data operations on the various `Entity` models

`entity` - shape of the data models and the relationships to each other

`migration` - relational DB SQL migrations

`service` - CLI flags, environment variables, config handling and backend daemon setup

`src` - contains a main function that initializes logging and calls all sub-services

`web` - API endpoint definition, routing, handling of request/responses, controllers
