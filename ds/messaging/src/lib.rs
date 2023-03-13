#![feature(async_fn_in_trait)]

use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;
pub mod errors;
pub mod kafka;
pub use errors::*;

#[derive(Debug)]
pub struct Message<K, P>
where
    K: Serialize + DeserializeOwned + Debug,
    P: Serialize + DeserializeOwned + Debug,
{
    pub key: Option<K>,
    pub payload: P,
}

impl<K, P> Message<K, P>
where
    K: Serialize + DeserializeOwned + Debug,
    P: Serialize + DeserializeOwned + Debug,
{
    pub fn new(payload: P) -> Self {
        Self { key: None, payload }
    }
    pub fn key(mut self, key: K) -> Self {
        self.key = Some(key);
        self
    }
}

trait MessageSender<K, P> {
    type K: Serialize + DeserializeOwned + Debug;
    type P: Serialize + DeserializeOwned + Debug;
    async fn send_message(&self, msg: Message<Self::K, Self::P>) -> Result<(), SenderError>;
}

pub trait MessageListener {
    fn start(&self) -> Result<(), ListenerError>;
}

pub trait MessageProcessor {
    type K: Serialize + DeserializeOwned + Debug;
    type P: Serialize + DeserializeOwned + Debug;
    fn process_message(&self, msg: Message<Self::K, Self::P>) -> Result<(), ProcessorError>;
}

pub type FnProcessMessage<K, P> = fn(msg: Message<K, P>) -> Result<(), ProcessorError>;
