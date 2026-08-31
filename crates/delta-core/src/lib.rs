//! Domain and application services for DeltaBeer.
//!
//! This crate owns business invariants and application workflows, including
//! authentication, user management, and account transactions.
//!
//! Its ports define the capabilities that workflows need from the outside
//! world, such as repositories, clocks, identifiers, and token sources.
//! Adapters such as HTTP and Diesel depend on these interfaces, so dependency
//! direction points inward toward this crate.
//!
//! The core intentionally does not own transport formats, HTTP routing,
//! database schemas, SQL queries, or terminal presentation.

pub mod domain;
pub mod infra;
pub mod ports;
pub mod services;
