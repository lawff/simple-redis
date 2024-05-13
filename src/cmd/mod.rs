mod hmap;
mod map;
mod set;
mod string;

use crate::{Backend, RespArray, RespFrame, SimpleError, SimpleString};
use enum_dispatch::enum_dispatch;
use lazy_static::lazy_static;
use thiserror::Error;

// use once_cell::sync::Lazy;

// static RESP_OK: Lazy<RespFrame> = Lazy::new(|| {
//     SimpleString::new("OK").into()
// });
lazy_static! {
    static ref RESP_OK: RespFrame = SimpleString::new("OK").into();
}

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("Invalid command: {0}")]
    InvalidCommand(String),
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("from Utf8 error: {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
}

#[enum_dispatch]
pub trait CommandExecutor {
    fn execute(self, backend: &Backend) -> RespFrame;
}

#[enum_dispatch(CommandExecutor)]
#[derive(Debug)]
pub enum Command {
    Get(Get),
    Set(Set),
    HGet(HGet),
    HMGet(HMGet),
    HGetAll(HGetAll),
    HSet(HSet),
    ECHO(Echo),
    SAdd(SAdd),
    SMembers(SMembers),
    // unrecognized command
    Unrecognized(Unrecognized),
}

#[derive(Debug)]
pub struct Get {
    key: String,
}

#[derive(Debug)]
pub struct Set {
    key: String,
    value: RespFrame,
}

#[derive(Debug)]
pub struct HGet {
    key: String,
    field: String,
}

#[derive(Debug)]
pub struct HMGet {
    key: String,
    fields: Vec<String>,
}

#[derive(Debug)]
pub struct HSet {
    key: String,
    field: String,
    value: RespFrame,
}

#[derive(Debug)]
pub struct HGetAll {
    key: String,
    sort: bool,
}

#[derive(Debug)]
pub struct SAdd {
    key: String,
    values: Vec<String>,
}

#[derive(Debug)]
pub struct SMembers {
    key: String,
    sort: bool,
}

#[derive(Debug)]
pub struct Echo {
    message: String,
}

#[derive(Debug)]
pub struct Unrecognized;

impl TryFrom<RespFrame> for Command {
    type Error = CommandError;

    fn try_from(value: RespFrame) -> Result<Self, Self::Error> {
        match value {
            RespFrame::Array(value) => value.try_into(),
            _ => Err(CommandError::InvalidCommand(
                "Command must be an Array".to_string(),
            )),
        }
    }
}

impl TryFrom<RespArray> for Command {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        match value.first() {
            Some(RespFrame::BulkString(ref cmd)) => match cmd.to_ascii_lowercase().as_slice() {
                b"get" => Ok(Get::try_from(value)?.into()),
                b"set" => Ok(Set::try_from(value)?.into()),
                b"hget" => Ok(HGet::try_from(value)?.into()),
                b"hset" => Ok(HSet::try_from(value)?.into()),
                b"hgetall" => Ok(HGetAll::try_from(value)?.into()),
                b"echo" => Ok(Echo::try_from(value)?.into()),
                b"hmget" => Ok(HMGet::try_from(value)?.into()),
                b"sadd" => Ok(SAdd::try_from(value)?.into()),
                b"smembers" => Ok(SMembers::try_from(value)?.into()),
                _ => Ok(Unrecognized.into()),
            },
            _ => Err(CommandError::InvalidCommand(
                "Command must have a BulkString".to_string(),
            )),
        }
    }
}

impl CommandExecutor for Unrecognized {
    fn execute(self, _: &Backend) -> RespFrame {
        // RESP_OK.clone()
        SimpleError::new("Unrecognized command").into()
    }
}

fn validate_command(
    value: &RespArray,
    names: &[&'static str],
    n_args: usize,
    more_than: bool,
) -> Result<(), CommandError> {
    if more_than {
        if value.len() < names.len() + n_args {
            return Err(CommandError::InvalidCommand(format!(
                "Expected at least {} arguments, but get {}",
                n_args,
                value.len() - names.len()
            )));
        }
    } else if value.len() != names.len() + n_args {
        return Err(CommandError::InvalidCommand(format!(
            "Expected {} arguments, but get {}",
            n_args,
            value.len() - names.len()
        )));
    }

    for (i, name) in names.iter().enumerate() {
        match value[i] {
            RespFrame::BulkString(ref cmd) => {
                if cmd.to_ascii_lowercase() != name.as_bytes() {
                    return Err(CommandError::InvalidCommand(format!(
                        "Expected command {}, got {}",
                        name,
                        String::from_utf8_lossy(cmd.as_ref())
                    )));
                }
            }
            _ => {
                return Err(CommandError::InvalidCommand(
                    "Command must have a BulkString".to_string(),
                ))
            }
        }
    }

    Ok(())
}

fn extract_args(value: RespArray, start: usize) -> Result<Vec<RespFrame>, CommandError> {
    Ok(value.0.into_iter().skip(start).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RespDecode, RespNull};
    use anyhow::Result;
    use bytes::BytesMut;

    #[test]
    fn test_command() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*2\r\n$3\r\nGET\r\n$5\r\nhello\r\n");

        let frame = RespArray::decode(&mut buf)?;

        let cmd: Command = frame.try_into()?;

        let backend = Backend::new();

        let ret = cmd.execute(&backend);
        assert_eq!(ret, RespFrame::Null(RespNull));

        Ok(())
    }
}
