use std::{fmt::Debug};

use serde::{Serialize, de::DeserializeOwned};

use crate::{MessageProcessor, ProcessorError, ListenerError, Message, MessageListener};

pub struct Listener<'a, K, P>
where
    K: Serialize + DeserializeOwned + Debug,
    P: Serialize + DeserializeOwned + Debug,
{
    processor: &'a dyn MessageProcessor<K = K, P = P>,
}

impl <K,P> MessageListener for Listener<'_,K,P>
where
    K: Serialize + DeserializeOwned + Debug,
    P: Serialize + DeserializeOwned + Debug,
{
    fn start(&self) -> Result<(), ListenerError> {
        // implement the start method here
        let payload: P = serde_json::from_str("\"helloworld\"")?;
        let msg:Message<K,P> = Message::new(payload);
        self.processor.process_message(msg)?;
        Ok(())
    }
}

impl <'a,K,P>Listener<'a,K,P>
where
    K: Serialize + DeserializeOwned + Debug,
    P: Serialize + DeserializeOwned + Debug,
{
    pub fn new(processor: &'a dyn MessageProcessor<K=K,P=P>) -> Self {
        Self { processor }
    }
}





#[cfg(test)] 
mod tests{
    use super::*;
    fn tmp () {
        for x in (1..2) {

        }
    }

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

        fn process_message(
            &self,
            msg: Message<Self::K, Self::P>,
        ) -> Result<(), ProcessorError> {
            println!("Received message: {:?}", msg);
            Ok(())
        }
    }


    #[test]
    fn test1() {
        let x =  MyMessageProcessor::new();
        let l = Listener::new(&x);
        let rs = l.start();
        match rs {
            Ok(_) => println!("great!"),
            Err(e) => println!("err : {:?}",e),
        }
    }
}