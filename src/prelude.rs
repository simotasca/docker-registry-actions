use anyhow::Error;
use std::result::Result as StdResult;

pub use std::format as f;

pub struct W<T>(pub T);

pub type Result<T> = StdResult<T, Error>;
