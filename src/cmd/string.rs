use crate::{Backend, BulkString, RespArray, RespFrame};

use super::{extract_args, validate_command, CommandError, CommandExecutor, Echo};

impl CommandExecutor for Echo {
    fn execute(self, _backend: &Backend) -> RespFrame {
        BulkString::from(self.message).into()
    }
}

impl TryFrom<RespArray> for Echo {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["echo"], 1, false)?;

        let mut args = extract_args(value, 1)?.into_iter();
        match args.next() {
            Some(RespFrame::BulkString(message)) => Ok(Echo {
                message: String::from_utf8(message.0)?,
            }),
            _ => Err(CommandError::InvalidArgument("Invalid message".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::RespDecode;

    use super::*;
    use anyhow::Result;
    use bytes::BytesMut;

    #[test]
    fn test_echo_from_resp_array() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*2\r\n$4\r\necho\r\n$5\r\nhello\r\n");

        let frame = RespArray::decode(&mut buf)?;
        let ret: Echo = frame.try_into()?;

        assert_eq!(ret.message, "hello");

        Ok(())
    }

    #[test]
    fn test_echo_command() {
        let echo = Echo {
            message: "hello".to_string(),
        };

        let frame: RespFrame = echo.execute(&Backend::new());
        let expected = RespFrame::BulkString(BulkString::from("hello"));

        assert_eq!(frame, expected);
    }
}
