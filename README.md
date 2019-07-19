# Contact Open Source with Actix

Contact Open Source is free contact management software, akin to a contacts app, or address book, or customer relationship manganer (CRM).

Contents:

* [Introduction](#introduction)
* [Implementation](#implementation)
  * [Actix Web framework](#actix-web-framework)
  * [Diesel ORM data access](#diesel-orm-data-access)
  * [PostgreSQL database](#postgresql-database)
  * [Rust crates](#rust-crates)


## Introduction

The goal of this project is to provide free, open, modern software that is especially easy to change and adapt by software programmers, application developers, and database administrators.


## Implementation


### Actix Web framework

The web application is implemented with Actix Web.

We welcome contributing code that provides additional implementations, such as for more web frameworks (e.g. Node Express, Ruby on Rails, Python Django).

The current code is deliberately very simple: there are only two web pages: a contact list, and a contact item.

We encourage you to extend this code as you like, for your own kinds of web pages and capabilties.


### Diesel ORM data access

The data access is implemented with Diesel, which is a mainstream Rust ORM.

We welcome contributing code that provides additional implementations, such as for more data access capabilities (e.g. GraphQL) and for more data speed (e.g. caching, tuning, optimizing).

The current code is deliberately very simple: there is only one model: a contact item. 

We encourage you to extend this code as you like, for your own kinds of models functionality.


### PostgreSQL database

The database is implemented with PostgreSQL, using Liquibase migration annotations. 

We welcome contributing code that provides additional implementations, such as for more databases (e.g. MySQL, SQL Server, Oracle) and for more migration tooling (e.g. Flyway, DB-Migrate, DBMate).

The current code is deliberately very simple: there is only one table: a contact item. 

We encourage you to extend this code as you like, for your own kinds of SQL and functionality.


### Rust crates

The application is implemented with these Rust crates:

  * [actix-diesel](https://crates.io/crates/actix-diesel): Actix integration of Diesel ORM
  * [actix-files](https://crates.io/crates/actix-files): Actix file support, for static files, for actix web
  * [actix-form-data](https://crates.io/crates/actix-form-data): library for retrieving form data from Actix Web's multipart streams.
  * [actix-identity](https://crates.io/crates/actix-identity): Actix identity service for actix web framework
  * [actix-multipart-rfc7578](https://crates.io/crates/actix-multipart-rfc7578): implements multipart/form-data media type described in RFC 7578.
  * [actix-net](https://crates.io/crates/actix-net): Actix networking services framework
  * [actix-rt](https://crates.io/crates/actix-rt): Actix runtime
  * [actix-session](https://crates.io/crates/actix-session): Actix session, a general solution for session management, using cookies.
  * [actix-tools](https://crates.io/crates/actix-tools): components for Actix and Actix-web to build modular, 12-factor compliant apps.
  * [actix-web](https://crates.io/crates/actix-web): Actix web framework is simple, pragmatic, extremely fast, and for Rust.
  * [actix-web-httpauth](https://crates.io/crates/actix-web-httpauth): HTTP authentication schemes for actix-web framework.
  * [background-jobs](https://crates.io/crates/background-jobs): tooling to run processes asynchronously, such as web services.
  * [bcrypt](https://crates.io/crates/bcrypt): easily hash and verify passwords using bcrypt.
  * [bigdecimal](https://crates.io/crates/bigdecimal): enable Rust to use big decimal numbers.
  * [Note](https://crates.io/crates/Note): version must match the diesel dependency version.
  * [bytes](https://crates.io/crates/bytes): a utility library for working with bytes.
  * [chrono](https://crates.io/crates/chrono): date and time library for Rust.
  * [clap](https://crates.io/crates/clap): Command Line Argument Parser for Rust.
  * [diesel](https://crates.io/crates/diesel): A safe, extensible ORM and Query Builder for Rust.
  * [dotenv](https://crates.io/crates/dotenv): environment configuration implementation of dotenv for Rust.
  * [env_logger](https://crates.io/crates/env_logger): a logging implementation for log which is configured via an environment variable.
  * [failure](https://crates.io/crates/failure): experimental error handling abstraction.
  * [futures](https://crates.io/crates/futures): an implementation of futures and streams featuring zero allocations, composability, and iterator-like interfaces.
  * [jsonwebtoken](https://crates.io/crates/jsonwebtoken): create and parse JWT in a strongly typed way.
  * [r2d2](https://crates.io/crates/r2d2): a generic connection pool.
  * [rand](https://crates.io/crates/rand): A Rust library for random number generation.
  * [rust-argon2](https://crates.io/crates/rust-argon2): Rust library for hashing passwords using Argon2 password-hashing function.
  * [Serde](https://crates.io/crates/Serde): a framework for serializing and deserializing Rust data structures.
  * [serde_derive](https://crates.io/crates/serde_derive): macros 1.1 implementation of #[derive(Serialize, Deserialize)].
  * [serde_json](https://crates.io/crates/serde_json): a JSON serialization file format.
  * [uuid](https://crates.io/crates/uuid): Generate and parse UUIDs.


