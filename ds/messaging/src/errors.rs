#[derive(Debug)]
pub enum SenderError {
    InitError(String),
    MessageSerializationError(String),
    MessageSendError(String),
}
#[derive(Debug)]
pub enum ProcessorError {
    GenericError(String),
}
#[derive(Debug)]
pub enum ListenerError {
    MessageDeserializationError(String),
    GenericError(String),
}

impl From<serde_json::Error> for SenderError {
    fn from(e: serde_json::Error) -> Self {
        SenderError::MessageSerializationError(e.to_string())
    }
}

impl From<serde_json::Error> for ListenerError {
    fn from(e: serde_json::Error) -> Self {
        ListenerError::MessageDeserializationError(e.to_string())
    }
}

impl From<ProcessorError> for ListenerError {
    fn from(pe: ProcessorError) -> Self {
        match pe {
            ProcessorError::GenericError(str) => ListenerError::GenericError(str),
        }
    }
}
