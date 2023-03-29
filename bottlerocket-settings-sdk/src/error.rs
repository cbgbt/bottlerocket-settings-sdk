use snafu::Snafu;

#[derive(Debug, Snafu, Clone)]
pub enum Error {}

pub(crate) type Result<T> = anyhow::Result<T>;
