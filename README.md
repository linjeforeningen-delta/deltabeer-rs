# DeltaBeer

DeltaBeer is a Rust workspace for managing users, account balances, and transactions. It provides an HTTP server backed
by SQLite and a terminal user interface that communicates with the server through the shared API contract. Business
rules and application services are kept separate from transport, persistence, and presentation concerns.

## Components

| Crate            | Purpose                                                                                              |
| ---------------- | ---------------------------------------------------------------------------------------------------- |
| `delta-core`     | Defines domain values, business rules, application services, and repository ports.                   |
| `delta-api`      | Defines the serializable request and response types shared by the server and TUI.                    |
| `storage-diesel` | Implements core repository ports with Diesel, SQLite, schema support, and migrations.                |
| `server`         | Loads configuration, assembles dependencies, and serves the Axum HTTP API and OpenAPI documentation. |
| `tui`            | Provides the terminal interface and makes asynchronous requests to the HTTP API.                     |
| `cli`            | Placeholder command-line binary with no current production behavior.                                 |

## Getting started

Install Rust and Cargo, then work from the repository root.
Review [configuration.md](docs/configuration.md) and apply the SQLite migration described
in [deployment.md](docs/deployment.md). The server does not migrate the database at startup.

Start the server:

```sh
cargo run -p server -- --config config/development.yaml
```

In a second terminal, start the TUI:

```sh
cargo run -p tui -- --config config/development.yaml
```

See [development.md](docs/development.md) for prerequisites and validation commands.

## Documentation

| Document                               | Purpose                                                                                   |
| -------------------------------------- | ----------------------------------------------------------------------------------------- |
| [Architecture](docs/architecture.md)   | Crate boundaries, dependency flow, runtime request flow, and key invariants.              |
| [Configuration](docs/configuration.md) | Configuration files, keys, defaults, and environment-variable behavior.                   |
| [Deployment](docs/deployment.md)       | Release builds, database migrations, backups, restores, and upgrade guidance.             |
| [API](docs/api.md)                     | HTTP routes, authentication, request and response types, errors, and OpenAPI notes.       |
| [TUI](docs/tui.md)                     | Terminal pages, input behavior, key bindings, scanner handling, and authentication flows. |
| [Operations](docs/operations.md)       | Startup troubleshooting, database and scanner checks, logging, and health checks.         |
| [Development](docs/development.md)     | Workspace layout, local development, tests, migrations, and release preparation.          |
