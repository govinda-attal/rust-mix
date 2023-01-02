#![feature(async_fn_in_trait)]

use serde::{Serialize, de::DeserializeOwned};
use std::fmt::Debug;
pub mod kafka;
pub mod errors;
pub use errors::*;

#[derive(Debug)]
pub struct Message<K, P>
where
    K: Serialize + DeserializeOwned + Debug,
    P: Serialize + DeserializeOwned + Debug,
{
    key: Option<K>,
    payload: P,
}

impl<K, P> Message<K, P>
where
    K: Serialize + DeserializeOwned + Debug,
    P: Serialize + DeserializeOwned + Debug,
{
    pub fn new( payload: P) -> Self {
        Self { key: None, payload }
    }
    pub fn key(mut self, key: K) -> Self {
        self.key = Some(key);
        self
    }
}


trait MessageSender {
    async fn send_message<K: Serialize + DeserializeOwned + Debug, P: Serialize + DeserializeOwned + Debug>(
        &self,
        msg: Message<K, P>,
    ) -> Result<(), SenderError>;
}

pub trait MessageListener {
    fn start(&self) -> Result<(), ListenerError>;
}


pub trait MessageProcessor {
    type K: Serialize + DeserializeOwned + Debug;
    type P: Serialize + DeserializeOwned + Debug;

    fn process_message(
        &self,
        msg: Message<Self::K, Self::P>,
    ) -> Result<(), ProcessorError>;
}
