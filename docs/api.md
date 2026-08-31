# API

This is the HTTP API implemented by the server.
Runtime routes and handler extractors are authoritative for request behavior.
OpenAPI annotations describe the generated reference, but differences from runtime code are called out below.

## Contents

| Section                                                                               | Scope                                                       |
| ------------------------------------------------------------------------------------- | ----------------------------------------------------------- |
| [Overview and versioning](#overview-and-versioning)                                   | Base paths and generated documentation endpoints.           |
| [Authentication overview](#authentication-overview)                                   | Public routes, bearer authentication, and token lifecycles. |
| [Public endpoints](#public-endpoints)                                                 | Health, user, and statistics routes.                        |
| [Admin authentication endpoints](#admin-authentication-endpoints)                     | Single-use and session-token routes.                        |
| [Protected administration endpoints](#protected-administration-endpoints)             | Administrator listing and user-management routes.           |
| [Request and response DTOs](#request-and-response-dtos)                               | Shared `delta-api` wire representations.                    |
| [Error model](#error-model)                                                           | Error response shape and codes.                             |
| [OpenAPI and Swagger](#openapi-and-swagger)                                           | Generated reference behavior.                               |
| [Implementation and OpenAPI discrepancies](#implementation-and-openapi-discrepancies) | Known annotation/runtime differences.                       |

## Overview and versioning

The unversioned health endpoint is `GET /health`.
All business routes currently use the `/v1` prefix.
Swagger UI is served at `/docs`.
The generated OpenAPI document is served as JSON at `/api-doc/openapi.json`.
The paths in this document include the `/v1` prefix unless stated otherwise.

## Authentication overview

User and statistics routes are public.
The `/v1/admins` router is wrapped by bearer-token middleware.
The middleware exempts the password route after Axum strips the enclosing `/v1` and `/admins` prefixes.
All other admin routes require `Authorization: Bearer <token>`.
It decodes the bearer value as unpadded base64url containing exactly 32 bytes, then validates the token record and expiry.
The validator accepts any currently valid token record and does not check `TokenKind`.

`POST /v1/admins/pass` verifies credentials and issues a `SingleUse` token intended for the session exchange.
`POST /v1/admins/session` validates the bearer token and issues a `Session` token.
The TUI uses the single-use token for the session request and uses the resulting session token for subsequent admin requests.

Validation expires a single-use token as part of successful validation.
`DELETE /v1/admins/session` explicitly expires the bearer token supplied to that request.

Both token lifetimes are configuration-controlled by `auth.single_use_token_ttl_seconds` and `auth.admin_session_ttl_seconds`.
See [configuration.md](configuration.md) for the configured values and configuration format.

The password route is therefore unauthenticated at runtime, although its handler still verifies the supplied credentials.

## Public endpoints

These routes do not use bearer authentication.

| Method | Path                        | Request                                           | Response          | Notes                                                          |
| ------ | --------------------------- | ------------------------------------------------- | ----------------- | -------------------------------------------------------------- |
| `GET`  | `/health`                   | None                                              | `HealthResponse`  | Unversioned health check.                                      |
| `GET`  | `/v1/users`                 | None                                              | `UserDto[]`       | Lists users.                                                   |
| `GET`  | `/v1/users/resolve/{ident}` | Path `ident`                                      | `UserIdDto`       | Resolves a UUID, decimal `u32` card number, or ASCII username. |
| `GET`  | `/v1/users/{ident}`         | Path `ident`                                      | `UserDto`         | The handler extracts `UserIdDto`, so `{ident}` must be a UUID. |
| `POST` | `/v1/users/{ident}/spend`   | UUID path plus transparent `SpendRequestDto` body | `TransactionDto`  | The handler extracts `UserIdDto`; the JSON body is a number.   |
| `GET`  | `/v1/stats`                 | None                                              | `StatsDto`        | Comprehensive statistics.                                      |
| `GET`  | `/v1/stats/summary`         | None                                              | `StatsSummaryDto` | Summary statistics.                                            |

`/users/resolve/{ident}` is the flexible identifier endpoint.
Its parser tries UUID first, then decimal `u32`, then any ASCII string as a username.
The other user handlers deserialize the path directly into the transparent UUID-backed `UserIdDto`.

## Admin authentication endpoints

These routes are all under the middleware-wrapped `/v1/admins` router.

| Method   | Path                 | Authentication at runtime | Request       | Response                  | Notes                                                                           |
| -------- | -------------------- | ------------------------- | ------------- | ------------------------- | ------------------------------------------------------------------------------- |
| `POST`   | `/v1/admins/pass`    | None                      | `Credentials` | `AdminTokenDto`           | Verifies `userId` and `password`, then issues the short-lived single-use token. |
| `POST`   | `/v1/admins/session` | Bearer token required     | None          | `AdminTokenDto`           | Issues a session token for the authenticated token owner.                       |
| `DELETE` | `/v1/admins/session` | Bearer token required     | None          | Empty JSON value (`null`) | Expires the supplied bearer token.                                              |

The password route handler itself has no authentication extractor.

## Protected administration endpoints

Every route in this section requires a currently valid bearer token.

### Administration and listing

| Method | Path         | Request | Response    | Notes                 |
| ------ | ------------ | ------- | ----------- | --------------------- |
| `GET`  | `/v1/admins` | None    | `UserDto[]` | Lists administrators. |

### User creation

| Method | Path                                | Request                | Response  | Notes           |
| ------ | ----------------------------------- | ---------------------- | --------- | --------------- |
| `POST` | `/v1/admins/user_management/create` | `UserCreateRequestDto` | `UserDto` | Creates a user. |

### User update

| Method  | Path                                        | Request                       | Response  | Notes                                                                    |
| ------- | ------------------------------------------- | ----------------------------- | --------- | ------------------------------------------------------------------------ |
| `PATCH` | `/v1/admins/user_management/{ident}/update` | UUID path plus `UserPatchDto` | `UserDto` | Despite the route and annotation name, the handler extracts `UserIdDto`. |

### Top-up

| Method | Path                                       | Request                                           | Response         | Notes                      |
| ------ | ------------------------------------------ | ------------------------------------------------- | ---------------- | -------------------------- |
| `POST` | `/v1/admins/user_management/{ident}/topup` | UUID path plus transparent `TopupRequestDto` body | `TransactionDto` | The JSON body is a number. |

### Grant and revoke admin

| Method   | Path                                       | Request                                       | Response  | Notes                                            |
| -------- | ------------------------------------------ | --------------------------------------------- | --------- | ------------------------------------------------ |
| `POST`   | `/v1/admins/user_management/{ident}/admin` | UUID path plus transparent `PasswordDto` body | `UserDto` | The JSON body is a password string.              |
| `DELETE` | `/v1/admins/user_management/{ident}/admin` | UUID path                                     | `UserDto` | Revokes the role and returns the resulting user. |

The `{ident}` segments in all management handlers above are currently UUID-only because each handler uses
`Path<UserIdDto>`.

## Request and response DTOs

The shared wire types are defined in the [`delta-api`](../crates/delta-api/src/) crate.
Structs marked with `rename_all = "camelCase"` use names such as `userId`, `cardNumber`, `approvedBy`, and `totalTransactions` on the wire.
Transparent newtypes serialize as their inner JSON value rather than as an object.
For example, `SpendRequestDto(25)`, `TopupRequestDto(25)`, and `PasswordDto("example-password")` are JSON number, number, and string bodies respectively.
`AdminTokenDto` is also transparent and is returned as a JSON string.
Tokens are opaque 32-byte values encoded by the server as unpadded base64url strings.
Do not use real credentials or bearer tokens in examples.
`UserIdDto` and `TransactionIdDto` are transparent UUID values represented as JSON strings.
`AmountDto` is a transparent unsigned 32-bit integer representing an amount in the server's smallest unit.
`UserDto` contains `id`, `name`, `username`, `program`, `cardNumber`, `role`, `birthdate`, `comments`, `balance`, and `spent`.
`UserCreateRequestDto` contains `name`, `username`, `program`, `cardNumber`, and `birthdate`.
`UserPatchDto` contains optional `name`, `username`, `program`, `cardNumber`, and `comments` fields.
`TransactionDto` contains `id`, `userId`, `kind`, `amount`, `timestamp`, nullable `approvedBy`, and `source`.
Transaction kinds are `spend` and `topUp`.
Transaction sources are `live`, `migration`, and `adjustment`.
`StatsDto` contains `totalUsers`, `totalBalance`, and `totalSpent`.
`StatsSummaryDto` adds `totalTransactions`.
OpenAPI provides the complete generated schema details for these DTOs.

## Error model

API errors are JSON objects with this shape:

```json
{
  "code": "not_found",
  "message": "Resource not found"
}
```

The `message` field is nullable.

| API error code            | HTTP status | Typical source or meaning                                                                                          |
| ------------------------- | ----------: | ------------------------------------------------------------------------------------------------------------------ |
| `invalid_user_identifier` |         400 | The flexible identifier parser rejected the path value.                                                            |
| `bad_request`             |         400 | Invalid input or another server-classified bad request.                                                            |
| `not_found`               |         404 | The requested resource does not exist.                                                                             |
| `conflict`                |         409 | A resource conflict or insufficient balance.                                                                       |
| `unauthorized`            |         401 | Missing, malformed, expired, or otherwise invalid authorization, or authorization required by a service operation. |
| `forbidden`               |         403 | The operation is forbidden, including an underage-user restriction.                                                |
| `internal_error`          |         500 | Internal or storage failure.                                                                                       |

The server does not guarantee a particular message for every error.
Internal errors and invalid identifiers return no public message.

## OpenAPI and Swagger

The server builds the document with `utoipa`.
The top-level document includes the health operation and nests the versioned `v1` document.
The versioned document nests the users, admins, and stats documents.
Operation paths, parameters, security declarations, request bodies, and response schemas come from `#[utoipa::path]` annotations and DTO `ToSchema` implementations.
The admins document applies bearer security by default, while the password operation uses `security()` to declare no OpenAPI security requirement.
View the generated document at `/api-doc/openapi.json` or the interactive UI at `/docs`.

## Implementation and OpenAPI discrepancies

The following differences are present in the inspected source.

- The user route registrations use `{ident}` for `GET /v1/users/{ident}` and `POST /v1/users/{ident}/spend`, while their
  OpenAPI annotations use `{user_id}`. Runtime handlers deserialize `Path<UserIdDto>`, so they accept UUIDs only.
  `TODO:` Align route annotation parameter names with the registered path and document the UUID-only extractor
  consistently.

- Management route registrations use `{ident}`, but the top-up, grant, and revoke annotations declare a `user_id`
  parameter. Runtime handlers extract `Path<UserIdDto>` and therefore accept UUIDs only.
  `TODO:` Use one parameter name consistently in route declarations, annotations, and documentation.

- `POST /v1/admins/user_management/{ident}/admin` is annotated with `RoleDto` as its request body, but the handler
  extracts transparent `PasswordDto`. Runtime accepts a JSON string and returns `UserDto`.
  `TODO:` Change the OpenAPI request body annotation to `PasswordDto` and keep the response schema aligned with the
  handler.
