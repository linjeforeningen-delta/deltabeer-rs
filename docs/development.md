# Development

## Workspace layout

The root `Cargo.toml` defines the following workspace members.

| Member           | Responsibility                                                                           |
| ---------------- | ---------------------------------------------------------------------------------------- |
| `delta-core`     | Domain values, business rules, application services, and ports.                          |
| `delta-api`      | Serializable request and response DTOs shared by the server and TUI.                     |
| `storage-diesel` | Diesel/SQLite persistence, mappings, schema, and migrations.                             |
| `server`         | Configuration, dependency assembly, Axum HTTP routes, and OpenAPI.                       |
| `tui`            | Terminal UI, input handling, presentation state, localization, and HTTP client behavior. |
| `cli`            | Placeholder command-line binary.                                                         |

See [architecture.md](architecture.md) for dependency and boundary rules.

## Prerequisites

Install Rust and Cargo. The Diesel CLI is optional for manual migration workflows; automated tests do not require it.

Install the SQLite command-line tool for inspecting databases during development and operations.

The repository toolchain file intentionally uses the floating stable Rust channel and requires `rustfmt` and `clippy`, so local, CI, and release builds track the current stable compiler rather than claiming an MSRV that the project has not defined. CI and release Markdown checks use Node.js 22.

## Running locally

Run these commands from the repository root so the relative configuration and database paths resolve as documented.

```sh
cargo run -p server -- --config config/development.yaml
cargo run -p tui -- --config config/development.yaml
```

Start the server before starting the TUI.
Both binaries accept `--config <path>` and use the development configuration by default in debug builds.
Configuration paths and keys are described in [configuration.md](configuration.md).
The server does not run migrations at startup, so apply migrations to the configured SQLite database before first use.

## Build and validation commands

Run formatting checks:

```sh
cargo fmt --all -- --check
```

Run workspace tests:

```sh
cargo test --workspace --locked
```

Run Clippy across workspace targets:

```sh
cargo clippy --workspace --all-targets --locked -- -D warnings
```

Build Rust documentation without documenting dependencies:

```sh
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked
```

The Markdown tooling is installed and checked with npm:

```sh
npm ci
npm run docs:check
```

These are the same principal checks run by the CI workflow. CI runs on pushes to `main` and `dev`, and on pull requests. Security checks run on pull requests, pushes to `main`, and weekly. Keep the `Rust`, `Test`, and `Markdown` checks required in the `main` branch ruleset; require dependency review too if it is available for the repository and part of the project’s merge policy.

Build the workspace in release mode:

```sh
cargo build --workspace --release
```

Run the storage tests separately when changing persistence code.

```sh
cargo test -p storage-diesel
```

## Database development and migrations

Diesel CLI reads `DATABASE_URL` to select the SQLite database for manual migration commands.
The migration directory is `crates/storage-diesel/migrations`.
Migration directories must use the naming format required by the installed Diesel CLI.
The checked-in migration is currently in the `0001-initial-database` directory.
Verify migration command compatibility with the installed Diesel CLI before applying migrations to a development or operational database.

Run migrations from the repository root with an explicit migration directory:

```sh
DATABASE_URL=crates/storage-diesel/data/dev.sqlite \
  diesel migration run --migration-dir crates/storage-diesel/migrations
```

The storage crate contains `crates/storage-diesel/.env`, which supplies a default `DATABASE_URL` for Diesel commands run from that crate.
For a specific database, set `DATABASE_URL` explicitly rather than relying on that default.

To inspect migration state:

```sh
DATABASE_URL=crates/storage-diesel/data/dev.sqlite \
  diesel migration list --migration-dir crates/storage-diesel/migrations
```

Storage integration tests create temporary SQLite files, run the repository migrations directly through Diesel's migration harness, and discard those files after each test. They do not require the Diesel CLI.

Stop database writers and follow [operations.md](operations.md) before backing up, restoring, or migrating an operational database.

## OpenAPI development

The shared API DTOs define `utoipa::ToSchema` implementations in `crates/delta-api/src`.
The server defines endpoint `#[utoipa::path]` annotations in `crates/server/src/http` and assembles the document with `#[openapi]` declarations in the same module tree.
Run the server locally, then inspect the interactive Swagger UI at [http://localhost:3000/docs](http://localhost:3000/docs) or the generated JSON at [http://localhost:3000/api-doc/openapi.json](http://localhost:3000/api-doc/openapi.json).
The port and bind address come from the selected server configuration.

## Architectural conventions

Keep business rules and use-case policy in `delta-core`.
Put infrastructure concerns such as Diesel, SQLite, and blocking database work behind the ports defined by core.
Keep conversions at the boundary that owns each representation.
Keep HTTP wire DTOs in `delta-api` separate from core domain values and TUI presentation models.
Treat database checks and triggers as integrity enforcement for invariants that must also hold for direct SQL writes.
See [architecture.md](architecture.md) for the verified dependency graph, mappings, and invariants.

## Generated documentation

Generate workspace API documentation with:

```sh
cargo doc --workspace --no-deps
```

Cargo writes generated documentation below `target/doc`.
The repository ignores `/target/`, so generated documentation is not committed.

## Release preparation

Choose and verify the intended semantic version according to the change being released.
Update the workspace package `version` field in the root `Cargo.toml`.
Confirm that all workspace packages inherit the intended version and that no package has an unintentional override.
Regenerate or validate `Cargo.lock` if the version change or dependency changes affect it.
Run formatting, tests, Clippy, and Rustdoc using the commands above.
Build the workspace with `cargo build --workspace --release`.

Review whether the release changes require a new migration or any operational migration instructions.
Update relevant developer, API, configuration, deployment, or operations documentation.
Review the final diff and confirm that the version-bearing files, checks, migrations, and documentation are ready to commit.

## Tagging a release

Commit the version-bearing files and all release changes before creating the tag.
The release workflow reruns the Rust and Markdown quality gates, validates that an annotated tag matches the `server` and `tui` versions inherited from the root workspace package version, then builds and publishes those Linux binaries as a checksummed GitHub Release artifact. The TUI logo and locale files are embedded in the TUI binary; configuration and database migrations remain operator-supplied and are not included in the archive. It does not deploy to production; production installation and migrations remain a manual, operator-controlled step described in [deployment.md](deployment.md).

Create and push an annotated semantic-version tag after committing the version-bearing files:

```sh
git tag -a vX.Y.Z -m "DeltaBeer X.Y.Z"
git push origin main vX.Y.Z
```

The annotated tag follows the repository's existing `v<version>` tag convention. GitHub Actions builds `deltabeer-vX.Y.Z-x86_64-linux.tar.gz` with a `deltabeer-vX.Y.Z/` top-level directory containing `deltabeer-server` and `deltabeer-tui`, plus its `.sha256` checksum, and creates the GitHub Release from that existing tag. The default Ubuntu GNU target is used; the artifact is not claimed to be static.
Verify the commit and tag references before pushing when the release is prepared from a branch or a non-default remote.

Release provenance attestation is required by the release workflow. The repository must have GitHub artifact attestations available; if the repository or account does not support them, enable the required GitHub feature before publishing releases.

## GitHub repository settings

Workflow files do not enable branch protection or rulesets automatically. Configure the `main` ruleset to require the stable `Rust`, `Test`, and `Markdown` checks before merging, and require `Dependency review` if GitHub Code Security support is enabled and that check is part of the project’s policy. Optionally require branches to be up to date before merging. If the repository is maintained by one developer, pull-request requirements can remain disabled if that is intentional. Once history rewriting is no longer needed, block force pushes to `main`.

## Release checklist

- [ ] Update and verify the workspace version.
- [ ] Confirm `Cargo.lock` consistency where affected.
- [ ] Run `fmt`, tests, Clippy, and Rustdoc.
- [ ] Complete a release build.
- [ ] Review and apply required migrations.
- [ ] Update relevant documentation.
- [ ] Commit the version-bearing files and release changes.
- [ ] Create an annotated `v<version>` tag.
- [ ] Push the intended commit and tag.
