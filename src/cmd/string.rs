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
