use std::{fmt::Debug, marker::PhantomData, time::Duration};

use crate::{Message, MessageSender, SenderError};
use rdkafka::{
    config::ClientConfig,
    error::KafkaError,
    message::OwnedMessage,
    producer::{FutureProducer, FutureRecord},
    util::Timeout,
};
use serde::{de::DeserializeOwned, Serialize};

pub struct KafkaMessageSender<K, P>
where
    K: Serialize + DeserializeOwned + Debug,
    P: Serialize + DeserializeOwned + Debug,
{
    timeout: Timeout,
    topic: String,
    producer: FutureProducer,
    phantom_key: PhantomData<K>,
    phantom_payload: PhantomData<P>,
}

impl<K, P> KafkaMessageSender<K, P>
where
    K: Serialize + DeserializeOwned + Debug,
    P: Serialize + DeserializeOwned + Debug,
{
    pub fn new(brokers: &str, topic: &str) -> Result<Self, SenderError> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("acks", "all")
            .create()?;
        Ok(KafkaMessageSender {
            producer,
            topic: topic.to_string(),
            timeout: Timeout::After(Duration::from_secs(5)),
            phantom_key: PhantomData,
            phantom_payload: PhantomData,
        })
    }
}

impl<K, P> MessageSender<K, P> for KafkaMessageSender<K, P>
where
    K: Serialize + DeserializeOwned + Debug,
    P: Serialize + DeserializeOwned + Debug,
{
    type K = K;
    type P = P;
    async fn send_message(&self, msg: Message<Self::K, Self::P>) -> Result<(), SenderError> {
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

        let sender = KafkaMessageSender::new("localhost:9092", "topic-b").unwrap();
        let payload = MyPayload {
            content: "world".to_string(),
        };
        let msg: Message<Option<String>, MyPayload> = Message::new(payload);

        let res = sender.send_message(msg).await;
        if let Err(e) = res {
            println!("{:?}", e)
        }
    }
}
