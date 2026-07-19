//! Enterprise + application business rules (Clean Architecture inner rings).
//! Must not depend on `api` or `utils` — those depend on this, never the reverse.

pub mod domain;
pub mod usecase;
