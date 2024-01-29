use serde::{ser::Serializer, Serialize};

// create the error type that represents all errors possible in our program
#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error(transparent)]
    Reply(#[from] x11rb::rust_connection::ReplyError),

    #[error(transparent)]
    Connection(#[from] x11rb::rust_connection::ConnectionError),

    #[error(transparent)]
    FromUtf8(#[from] std::string::FromUtf8Error),


    #[error(transparent)]
    Simulate(#[from] rdev::SimulateError),

    #[error("{0}")]
    String(#[from] anyhow::Error),
}

// we must manually implement serde::Serialize
impl Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub type CommandResult<T, E = CommandError> = Result<T, E>;
