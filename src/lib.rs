mod auth;
mod client;
mod error;
mod models;
mod pagination;
mod rate_limit;
#[cfg(feature = "blocking")]
pub mod blocking;

pub use client::{ClientBuilder, CoverArt, DiscogsClient};
pub use error::DiscogsError;
pub use models::*;
pub use pagination::{Paginated, PaginationParams};
