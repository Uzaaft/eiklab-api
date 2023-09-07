pub mod chat;
pub mod named_entity_recognition;
pub mod sentiment;

use poem::{
    error::InternalServerError, listener::TcpListener, middleware::Cors, web::Data, EndpointExt,
    Result, Route, Server,
};

use poem_openapi::{param::Query, payload::PlainText, OpenApi, OpenApiService};
use sentiment::{classifier::SentimentClassifier, Sentiment};

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/hello", method = "get")]
    async fn index(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("hello, {name}!")),
            None => PlainText("hello!".to_string()),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();
    let sentiment_service = crate::Sentiment::default();

    let api_service =
        OpenApiService::new(sentiment_service, "Sentiment", "1.0").server("http://localhost:3000");
    let spec = api_service.spec();
    // let ui = api_service.swagger_ui();

    let route = Route::new()
        .nest("/", api_service)
        // .nest("/docs", ui)
        .at("/spec", poem::endpoint::make_sync(move |_| spec.clone()));

    Server::new(TcpListener::bind("127.0.0.1:4200"))
        .run(route)
        .await
}
