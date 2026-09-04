# Operations

Use [configuration.md](configuration.md) for setting meanings.
Use [deployment.md](deployment.md) for installation, migration, backup, restore, and upgrade procedures.

## Logging

The server writes formatted tracing output to standard output through the default tracing formatter.
The server uses `RUST_LOG` when that variable is present.
When `RUST_LOG` is absent, it uses the YAML `logging.filter` value.

The TUI writes to a daily rolling `tui.log` appender in a `logs` directory relative to its working directory.
The TUI creates that directory before opening the appender.
The TUI uses `RUST_LOG` when it is valid and otherwise falls back to `info`.

## Health and API diagnostics

The unversioned health endpoint is:

```sh
curl -i https://localhost:3000/health
```

If the local development CA is not installed in the system trust store, pass `--cacert <path-to-ca-cert>`:

```sh
curl -i --cacert certs/rootCA.pem https://localhost:3000/health
```

It should return HTTP 200 with an `ok` value of `true` and the server version.
The generated OpenAPI JSON is available at:

```sh
curl -i https://localhost:3000/api-doc/openapi.json
```

The interactive Swagger UI is available at `https://localhost:3000/docs`.
These checks verify HTTPS reachability and routing, not database content or TUI operation.

## Startup failures

### Configuration and TLS

Check the working directory, the `--config` path, file readability, YAML syntax, required sections, and value types.
Remember that relative configuration, certificate, and database paths are resolved from the process working directory.

For TLS errors:

- **Missing certificate or key file**: Verify `server.tls.cert_path` and `server.tls.key_path` point to existing files.
- **Malformed certificate or private key**: Verify that the files are valid PEM format and not empty or corrupted.
- **Certificate validation / SAN mismatch**: Verify the certificate contains the expected hostname or IP (e.g. `localhost`, `127.0.0.1`, or `::1`) in its Subject Alternative Names (SANs).
- **TUI insecure URL error**: The TUI requires an `https://` base URL and will fail immediately if configured with `http://`.
- **TUI untrusted certificate error**: If using a custom development CA, verify `tui.ca_cert_path` is configured with the CA PEM certificate or that the CA is installed in the system trust store.

### Database

Check that the configured database parent directory exists and is writable.
Check that the database has been migrated before starting the server.
The server does not run migrations automatically.

### Bind and network

Check that the configured bind address is valid and that the port is available.
For TUI connection failures, compare `tui.api_base_url` with the server listener and network policy.

### Terminal

TUI startup can fail when raw-mode or alternate-screen terminal initialization fails.
Run it from a real compatible terminal and inspect the TUI log if initialization gets far enough to create it.

### Locale and assets

The TUI rejects locales other than the bundled `en` and `nb` values.
The splash logo is embedded in the binary, so a splash initialization error indicates a binary or embedded-asset problem rather than a missing runtime asset directory.

## Database diagnostics

Run these checks with placeholders substituted for the active installation.

```sh
pwd
ls -ld /path/to/database-parent
ls -l /path/to/app.sqlite
sqlite3 /path/to/app.sqlite '.tables'
DATABASE_URL=/path/to/app.sqlite diesel migration list --migration-dir crates/storage-diesel/migrations
```

Run the Diesel command from the repository root, or use a migration-directory path valid for the directory from which it is run.
Stop all writers before copying, restoring, or migrating a live SQLite database.

## Scanner diagnostics

DeltaBeer's TUI parser accepts only terminal key events containing ASCII decimal digits.
An `Enter` event terminates a non-empty digit sequence and emits it as one card identifier.
If the configured inter-key gap expires, the buffered digits are released as ordinary keyboard input.
A non-digit event while digits are buffered also releases the buffered events as ordinary input.
The expected scanner contract and input precedence are documented in [tui.md](tui.md#scanner-input-model).
An operator can diagnose whether received terminal events satisfy this digit-plus-Enter contract and whether input gaps exceed `scanner_max_gap_ms`.
The TUI does not discover readers, inspect firmware, reset USB devices, or repair terminal event delivery.

### Hardware, firmware, USB, and OS input events

If a problem is reproducible in a plain terminal or a raw input-event tool, it is upstream of DeltaBeer's TUI parsing.
Do not attempt to compensate in TUI configuration for input events the application never receives.
The exact upstream cause is not established by the application source.
Operator diagnostics on Linux may include `lsusb`, `udevadm`, `evtest`, and observing terminal output, subject to local security policy.
These tools are operator diagnostics, not DeltaBeer application dependencies.

## Backup and recovery

Follow the ordered [backup](deployment.md#backup) and [restore](deployment.md#restore) procedures.
As an immediate reminder, stop all database writers, preserve the current file before replacement when possible, repair ownership and permissions, and verify the restored database before resuming service.

## Secrets and log safety

Do not intentionally share or log passwords, bearer tokens, or configuration secrets.
Restrict access to server output, TUI log files, configuration backups, and database backups.
Operational logs may also reveal filesystem paths, network addresses, request statuses, user or administrator identifiers, and error details.
Treat that metadata as potentially sensitive and redact it before external sharing where appropriate.
