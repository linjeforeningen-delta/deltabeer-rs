# Deployment

This document describes a manually assembled DeltaBeer installation.

Configuration semantics are in [configuration.md](configuration.md).
Operational diagnosis is in [operations.md](operations.md).

## Deployment model

The repository supplies Rust workspace crates, release binaries, YAML examples, bundled TUI assets/locales, Diesel migrations, and tests.
The repository does not supply packaging scripts, installers, containers, or systemd service units.
An operator must choose the service manager, users, permissions, filesystem layout, and process supervision.

## Release build

From the repository root, build both release binaries with:

```sh
cargo build --release -p server -p tui
```

The resulting binaries are `target/release/server` and `target/release/tui`.

## Runtime files and directories

The server needs its binary, a readable YAML file, an existing writable parent directory for the SQLite database, and a database that has been migrated before first use.
The TUI needs its binary, a readable YAML file, and access to a terminal.
The TUI logo and locale files are embedded in the binary at build time.
The server has no enforced installation directories.
`/etc/deltabeer/` for configuration, `/var/lib/deltabeer/` for SQLite data, and `/var/log/deltabeer/` for operator-managed logs are recommendations only.
Relative configuration and database paths resolve against each process's working directory.
The TUI creates a `logs` directory under its working directory for its own log file.

## Server startup

From the chosen working directory, start the server with:

```sh
/path/to/server --config /path/to/server.yaml
```

The server does not create database parent directories or run migrations at startup.

## TUI startup

From the chosen working directory, start the TUI with:

```sh
/path/to/tui --config /path/to/tui.yaml
```

The TUI must be able to initialize the terminal and connect to the configured API base URL.

## Service management

No official systemd unit or other service definition is supplied.
An operator-created service should specify an explicit working directory, runtime user, absolute executable and configuration paths, required permissions, restart policy, and log handling.
Do not assume that `journalctl` applies unless the operator has selected a service manager that sends logs there.

## Database migrations

The server does not run migrations automatically.
Run pending migrations from the repository root with Diesel CLI and the repository migration directory:

```sh
DATABASE_URL=/path/to/app.sqlite diesel migration run --migration-dir crates/storage-diesel/migrations
```

The migration directory may contain more than one migration over the lifetime of the installation.

Inspect migration state with:

```sh
DATABASE_URL=/path/to/app.sqlite diesel migration list --migration-dir crates/storage-diesel/migrations
```

The checked-in Diesel configuration names `migrations` as the migration directory when commands are run from `crates/storage-diesel`.

## Backup

Use a destination with restricted permissions and enough free space.

1. Stop the server and all other database writers.
2. Confirm the source database path from the active server YAML and working directory.
3. Create a SQLite backup to a separate destination, for example with `sqlite3` and `.backup`, or copy the file only while no writer can modify it.
4. Record the binary version, configuration used, and Diesel migration state.
5. Back up the configuration separately, after removing or protecting secrets as appropriate.
6. Verify that the backup file can be opened and that its tables are present.

## Restore

1. Stop the server, TUI, and every other process that can write the database.
2. Preserve the current database before replacing it, if it is still readable.
3. Restore a known-good SQLite backup into the configured database path.
4. Check ownership, permissions, parent-directory access, and the working-directory assumptions.
5. Confirm migration state before applying any pending migration.
6. Start the server and perform the health check.
7. Test a representative TUI operation before returning the installation to normal use.

Test restores periodically on a separate copy or disposable environment.

## Upgrade

1. Create and verify a backup.
2. Stop the server, TUI, and other database writers.
3. Install the new binaries.
4. Run pending Diesel migrations with the configured database URL.
5. Verify configuration paths, database permissions, and the intended working directory.
6. Start the server.
7. Check `/health` and the OpenAPI endpoint.
8. Start the TUI and perform a scanner and authenticated-operation smoke test.

## Deployment checklist

- [ ] Release binaries built.
- [ ] Configuration paths and working directories recorded.
- [ ] Database parent directory exists and is writable.
- [ ] Database migrations applied and state recorded.
- [ ] Server and TUI runtime users and permissions verified.
- [ ] Backup and restore procedure tested.
- [ ] Server health and OpenAPI checks pass.
- [ ] TUI connection, scanner, and authentication smoke tests pass.
