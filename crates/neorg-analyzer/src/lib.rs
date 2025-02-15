pub mod backend;
pub mod span;

pub use neorg_lang as neorg;

pub type OkSome<T> = Result<Option<T>, anyhow::Error>;
