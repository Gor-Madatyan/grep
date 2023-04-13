use derive_more::{Display, Error};

pub use config::*;

pub mod grep;
pub mod grep_over_file;


mod config;


#[derive(Debug, Copy, Clone, Display, Error)]
pub enum Error {
    HaystackNotExisted,
    PathNotExisted,
}

