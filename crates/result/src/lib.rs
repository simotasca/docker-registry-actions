mod error;

pub use error::{Backtrace, Error, ResultExt};

pub type Result<T> = std::result::Result<T, crate::error::Error>;