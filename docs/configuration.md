# Configuration

DeltaBeer reads YAML configuration separately in the server and TUI binaries.
The server requires the `server`, `auth`, and `logging` sections.
The TUI requires the `tui` section.

See [deployment.md](deployment.md) for installation and database procedures, and [operations.md](operations.md) for diagnosis and recovery.

## Configuration file selection

Both binaries accept `--config <path>`.
In debug builds, the default is `config/development.yaml`.
In non-debug builds, including release builds, the default is `config/production.yaml`.

The path is resolved relative to the process working directory.
The loader reads the selected file directly and does not search relative to the executable or the configuration file.

## Server configuration

These keys are deserialized by the server.
All fields are required in the YAML accepted by the server.

| Key                         | Type             | Required | Meaning                                   | Default |
| --------------------------- | ---------------- | -------- | ----------------------------------------- | ------- |
| `server.bind_addr`          | string           | yes      | Local address passed to the TCP listener. | none    |
| `server.database_url`       | string           | yes      | SQLite database URL passed to Diesel.     | none    |
| `server.database_pool_size` | unsigned integer | yes      | Maximum SQLite connection-pool size.      | none    |

The server does not supply application defaults for these fields.

## Authentication configuration

| Key                                 | Type           | Unit    | Meaning                                               |
| ----------------------------------- | -------------- | ------- | ----------------------------------------------------- |
| `auth.single_use_token_ttl_seconds` | signed integer | seconds | Policy duration for single-use authentication tokens. |
| `auth.admin_session_ttl_seconds`    | signed integer | seconds | Policy duration for administrator sessions.           |

The values control policy durations and are converted to `chrono::Duration` values.
The source does not add a range check for these values.
API consumers should treat TTL values as deployment policy rather than permanent API contract values.

## Logging configuration

`logging.filter` is the server's YAML `tracing_subscriber` filter.
If `RUST_LOG` is present, the server uses it instead of `logging.filter`.
If `RUST_LOG` is absent, the server applies `logging.filter` after loading the YAML file.
If the server cannot parse `RUST_LOG`, startup logging falls back to the source-level `info` filter and the YAML filter is not applied because the variable is still present.

| Key              | Type   | Required | Meaning                                                     | Default |
| ---------------- | ------ | -------- | ----------------------------------------------------------- | ------- |
| `logging.filter` | string | yes      | `tracing` filter directives used when `RUST_LOG` is absent. | none    |

## TUI configuration

| Key                             | Type             | Required | Meaning                                                                               | Unit         |
| ------------------------------- | ---------------- | -------- | ------------------------------------------------------------------------------------- | ------------ |
| `tui.api_base_url`              | string           | yes      | Base URL for the server HTTP API.                                                     | URL          |
| `tui.event_poll_interval_ms`    | unsigned integer | yes      | Interval used for terminal event polling and scanner flush cadence.                   | milliseconds |
| `tui.scanner_max_gap_ms`        | unsigned integer | yes      | Maximum gap between buffered digit events before they are released as ordinary input. | milliseconds |
| `tui.idle_splash_after_seconds` | unsigned integer | yes      | Inactivity duration before the active UI returns to the idle splash.                  | seconds      |
| `tui.locale`                    | string           | yes      | Initial bundled UI locale.                                                            | locale code  |

The TUI does not provide application defaults for these fields.

The scanner contract is documented in [tui.md](tui.md).

## Environment variables

| Variable       | Server runtime                               | TUI runtime                                                     | Diesel CLI and storage tests                                                                       |
| -------------- | -------------------------------------------- | --------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| `RUST_LOG`     | Optional filter override for server tracing. | Optional filter for TUI tracing; otherwise the TUI uses `info`. | Not configured by this repository as an application setting.                                       |
| `DATABASE_URL` | Not read by the server.                      | Not read by the TUI.                                            | Used to select the SQLite database for Diesel migration commands and by storage integration tests. |

`.env` files are tooling inputs only and are not application configuration sources documented here.
Do not copy secrets from environment files into configuration examples or operational records.

## Database path behavior

The server passes `server.database_url` directly to Diesel's SQLite connection manager.
A relative SQLite path is resolved against the server process working directory, not against the YAML file or executable directory.
The server does not create missing parent directories.
The database directory must therefore already exist and be writable by the server user.

## Locale configuration

The bundled locales are `en` (English) and `nb` (Norwegian Bokmål).
The TUI validates `tui.locale` against the bundled locale list before continuing startup.
An unsupported locale causes startup to fail.

## Validation and startup failures

Missing files, unreadable files, malformed YAML, missing required fields, or values with incompatible YAML types fail during configuration loading.
The source explicitly validates only whether the configured TUI locale is bundled.
The source does not perform general range validation for numeric settings.
Invalid bind addresses or unavailable bind addresses fail when the server creates its listener.
Database pool construction can fail later if the database URL, pool size, path, or permissions are unsuitable.
Terminal initialization and embedded splash-logo decoding can fail during TUI startup.
See [operations.md](operations.md) for grouped troubleshooting guidance.

## Complete example

The following is a representative example configuration, not a list of hidden application defaults.
The checked-in development and production files are examples with different database paths and server filters.

```yaml
server:
  bind_addr: "0.0.0.0:3000"
  database_url: "crates/storage-diesel/data/dev.sqlite"
  database_pool_size: 16
auth:
  single_use_token_ttl_seconds: 15
  admin_session_ttl_seconds: 600
logging:
  filter: "tower_http=info,axum=info,server=debug"
tui:
  api_base_url: "http://localhost:3000"
  event_poll_interval_ms: 20
  scanner_max_gap_ms: 80
  idle_splash_after_seconds: 60
  locale: "nb"
```
