pub mod classifier;

use poem::{
    error::{BadRequest, InternalServerError},
    web::Data,
    Result, Route,
};
use poem_openapi::{
    param::Query,
    payload::{Json, PlainText},
    ApiResponse, Object, OpenApi, Tags,
};
use rust_bert::pipelines::sentiment::{SentimentModel, SentimentPolarity};
use tokio::sync::Mutex;

pub struct Sentiment {
    pub classifier: crate::SentimentClassifier,
}
impl Default for Sentiment {
    fn default() -> Self {
        let (_handle, classifier) = crate::SentimentClassifier::spawn();
        Self { classifier }
    }
}

#[derive(Object)]
pub struct SentimentResponse {
    score: f64,
}

impl From<rust_bert::pipelines::sentiment::Sentiment> for SentimentResponse {
    fn from(sentiment: rust_bert::pipelines::sentiment::Sentiment) -> Self {
        if sentiment.polarity == SentimentPolarity::Negative {
            return Self {
                score: -sentiment.score,
            };
        }
        Self {
            score: sentiment.score,
        }
    }
}

#[derive(ApiResponse)]
enum PostResponse {
    #[oai(status = 200)]
    OK(Json<Vec<SentimentResponse>>),

    #[oai(status = 400)]
    BadRequest(PlainText<String>),
}

#[derive(Object)]
struct Texts {
    /// The texts to classify
    text: Vec<String>,
}
#[OpenApi]
impl Sentiment {
    /// Classify a list of strings
    #[oai(path = "/sentiment", method = "post")]
    async fn post_sentiment(&self, text: Json<Texts>) -> Result<PostResponse> {
        let (_handle, classifier) = crate::SentimentClassifier::spawn();
        let sentiments = classifier
            .predict(text.0.text)
            .await?
            .into_iter()
            .map(|x| x.into())
            .collect();
        Ok(PostResponse::OK(Json(sentiments)))
    }
}
