use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};

use crate::{FnProcessMessage, ListenerError, Message, MessageListener, MessageProcessor};

// pub struct Listener<'a, K, P>
pub struct Listener<K, P>
where
    K: Serialize + DeserializeOwned + Debug,
    P: Serialize + DeserializeOwned + Debug,
{
    // processor: &'a dyn MessageProcessor<K = K, P = P>,
    pub fn_process_message: FnProcessMessage<K, P>,
}

// impl<K, P> MessageListener for Listener<'_, K, P>
impl<K, P> MessageListener for Listener<K, P>
where
    K: Serialize + DeserializeOwned + Debug,
    P: Serialize + DeserializeOwned + Debug,
{
    fn start(&self) -> Result<(), ListenerError> {
        // implement the start method here
        let payload: P = serde_json::from_str("\"helloworld\"")?;
        let msg: Message<K, P> = Message::new(payload);
        let fn_process_message = &self.fn_process_message;
        fn_process_message(msg)?;
        Ok(())
    }
}

// impl<'a, K, P> Listener<'a, K, P>
impl<K, P> Listener<K, P>
where
    K: Serialize + DeserializeOwned + Debug,
    P: Serialize + DeserializeOwned + Debug,
{
    // pub fn new(processor: &'a dyn MessageProcessor<K = K, P = P>) -> Self {
    //     Self { processor }
    // }
    pub fn new(fn_process_message: FnProcessMessage<K, P>) -> Self {
        Self { fn_process_message }
    }
}

#[cfg(test)]
mod tests {
    use crate::ProcessorError;

    use super::*;

    #[derive(Debug)]
    pub struct MyMessageProcessor {}

    impl MyMessageProcessor {
        pub fn new() -> Self {
            MyMessageProcessor {}
        }
    }

    impl MessageProcessor for MyMessageProcessor {
        type K = String;
        type P = String;

        fn process_message(&self, msg: Message<Self::K, Self::P>) -> Result<(), ProcessorError> {
            println!("Received message: {:?}", msg);
            Ok(())
        }
    }

    #[test]
    fn test1() {
        let process_message = |msg: Message<String, String>| -> Result<(), ProcessorError> {
            println!("Received message: {:?}", msg);
            Ok(())
        };

        let l = Listener::new(process_message);
        let rs = l.start();
        match rs {
            Ok(_) => println!("great!"),
            Err(e) => println!("err : {:?}", e),
        }
    }
}
