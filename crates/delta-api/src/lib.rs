//! Serializable request, response, and error DTOs for the DeltaBeer API.
//!
//! These types are the shared wire contract between the HTTP server and its
//! clients, including the TUI.
//!
//! Transport handlers and domain services live in other crates. This crate
//! describes how values cross the API boundary and intentionally contains no
//! business logic or persistence behavior.

pub mod auth;
pub mod error;
pub mod stats;
pub mod transaction;
pub mod user;

pub use auth::{AdminTokenDto, Credentials, PasswordDto};
pub use error::{ApiErrorCode, ApiErrorResponse};
pub use stats::{StatsDto, StatsSummaryDto};
pub use transaction::{
    SpendRequestDto, TopupRequestDto, TransactionDto, TransactionIdDto, TransactionKindDto,
    TransactionSourceDto, UserIdentificationDto,
};
pub use user::{AmountDto, RoleDto, UserCreateRequestDto, UserDto, UserIdDto, UserPatchDto};
