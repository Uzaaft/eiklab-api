use std::error::Error;
use std::fmt::{Debug, Display};

#[derive(Debug, thiserror::Error)]
pub enum SentimentError {
    #[error("SentimentError")]
    SentimentError,
}

#[derive(Debug, thiserror::Error)]
pub enum SentimentError {
    #[error("SentimentError")]
    SentimentError,
}
