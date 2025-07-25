pub mod backend;
pub mod span;
mod handle;
pub use neorg_syntax as neorg;

pub type OkSome<T> = Result<Option<T>, anyhow::Error>;
