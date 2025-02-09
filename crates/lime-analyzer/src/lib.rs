pub mod backend;
pub mod neorg_lang;
pub mod span;

pub use neorg_syntax as neorg;

pub type OkSome<T> = Result<Option<T>, anyhow::Error>;
