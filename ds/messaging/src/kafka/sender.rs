use std::{fmt::Debug, time::Duration};

use crate::{Message, MessageSender, SenderError};
use rdkafka::{
    config::ClientConfig,
    error::KafkaError,
    producer::{FutureProducer, FutureRecord},
    util::Timeout, message::OwnedMessage,
};
use serde::{Serialize, de::DeserializeOwned};

pub struct KafkaMessageSender {
    timeout: Timeout,
    topic: String,
    producer: FutureProducer,
}

impl KafkaMessageSender {
    pub fn new(brokers: &str, topic: &str) -> Result<Self, SenderError> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("acks", "all")
            .create()?;
        Ok(KafkaMessageSender {
            producer,
            topic: topic.to_string(),
            timeout: Timeout::After(Duration::from_secs(5)),
        })
    }
}

impl MessageSender for KafkaMessageSender {
    async fn send_message<K: Serialize + DeserializeOwned + Debug, P: Serialize + DeserializeOwned + Debug>(
        &self,
        msg: Message<K, P>,
    ) -> Result<(), SenderError> {
        let payload = serde_json::to_string(&msg.payload)?;
        let key: String;

        let topic = &self.topic;
        let timeout = self.timeout.clone();
        let mut record: FutureRecord<String, String> = FutureRecord::to(&topic).payload(&payload);
        if let Some(k) = msg.key {
            key = serde_json::to_string(&k)?;
            record = record.key(&key);
        }

        let producer = self.producer.clone();
        _ = producer.send(record, timeout).await?;
            
        Ok(())
    }
}

impl From<KafkaError> for SenderError {
    fn from(e: KafkaError) -> Self {
        SenderError::InitError(e.to_string())
    }
}
impl From<(KafkaError, OwnedMessage)> for SenderError {
    fn from(e: (KafkaError, OwnedMessage)) -> Self {
        SenderError::MessageSendError(e.0.to_string())
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    struct MyPayload {
        content: String,
    }

    #[tokio::test]
    async fn test_send() {
        let sender = KafkaMessageSender::new("localhost:9092", "topic-b").unwrap();
        let payload = MyPayload {
            content: "hello".to_string(),
        };
        let msg: Message<String, MyPayload> = Message::new(payload).key("some-key-1".to_owned());

        let res = sender.send_message(msg).await;
        if let Err(e) = res {
            println!("{:?}", e)
        }

        let payload = MyPayload {
            content: "world".to_string(),
        };
        let msg: Message<String, MyPayload> = Message::new(payload);
        

        let res = sender.send_message(msg).await;
        if let Err(e) = res {
            println!("{:?}", e)
        }
    }
}
