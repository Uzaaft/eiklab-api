use std::{
    collections::HashMap,
    sync::mpsc,
    thread::{self, JoinHandle},
};

use poem::Result;
use poem_openapi::{
    param::Query,
    payload::{Json, PlainText},
    ApiResponse, Object, OpenApi,
};
use rust_bert::pipelines::{
    conversation::{ConversationConfig, ConversationManager, ConversationModel},
    sentiment::{Sentiment, SentimentConfig, SentimentModel},
};
use tokio::{sync::oneshot, task};
use uuid::Uuid;

type Conversations = HashMap<Uuid, ConversationManager>;

/// Runner for sentiment classification
pub struct Chat {
    conversations_manager: ConversationManager,
    model: ConversationModel,
}

impl Chat {
    pub fn new() -> Self {
        let config = ConversationConfig {
            do_sample: false,
            num_beams: 3,
            ..Default::default()
        };
        let conversation_model = ConversationModel::new(config).unwrap();
        let conversations_manager = ConversationManager::new();
        Self {
            model: conversation_model,
            conversations_manager,
        }
    }

    pub fn new_conversation(&mut self) -> anyhow::Result<Uuid> {
        let mut conversation_manager = ConversationManager::new();

        Ok(conversation_manager.create("Answer my questions"))
    }
}

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct Message {
    pub text: String,
}
#[derive(ApiResponse)]
enum ChatResponse {
    #[oai(status = 200)]
    Ok,
    #[oai(status = 400)]
    BadRequest,
}

// #[OpenApi]
// impl Chat {
//     /// Initialize and create the chat
//     #[oai(path = "/chat", method = "post")]
//     async fn index(&self, message: Json<Message>) -> Result<ChatResponse> {
//         Ok(ChatResponse::Ok)
//     }
// }
