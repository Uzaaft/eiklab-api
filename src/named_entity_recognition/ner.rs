use anyhow::Result;
use rust_bert::pipelines::ner::{Entity, NERModel};
use std::{
    sync::mpsc,
    thread::{self, JoinHandle},
};
use tokio::sync::oneshot;

type Message = (Vec<String>, oneshot::Sender<Vec<Vec<Entity>>>);

pub struct NERClassifier {
    sender: mpsc::SyncSender<Message>,
}

impl NERClassifier {
    pub fn spawn() -> (JoinHandle<Result<()>>, NERClassifier) {
        let (sender, receiver) = mpsc::sync_channel(100);
        let handle = thread::spawn(move || Self::runner(receiver));
        (handle, NERClassifier { sender })
    }
    /// The classification runner itself
    fn runner(receiver: mpsc::Receiver<Message>) -> Result<()> {
        // Needs to be in sync runtime, async doesn't work
        let model = NERModel::new(Default::default())?;

        while let Ok((texts, sender)) = receiver.recv() {
            let texts: Vec<&str> = texts.iter().map(String::as_str).collect();
            let sentiments = model.predict(&texts);
            sender.send(sentiments).expect("sending results");
        }

        Ok(())
    }

    /// Make the runner predict a sample and return the result
    pub async fn predict(&self, texts: Vec<String>) -> Result<Vec<Vec<Entity>>> {
        let (sender, receiver) = oneshot::channel();
        tokio::task::block_in_place(|| self.sender.send((texts, sender)))?;
        Ok(receiver.await?)
    }
}
