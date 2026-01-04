pub mod backend;
mod handle;
pub mod span;
pub use neorg_syntax as neorg;
pub mod types;

pub type OkSome<T> = Result<Option<T>, anyhow::Error>;
