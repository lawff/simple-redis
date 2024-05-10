use crate::{Backend, BulkString, RespArray, RespFrame};

use super::{extract_args, validate_command, CommandError, CommandExecutor, SAdd, SMembers};

impl CommandExecutor for SAdd {
    fn execute(self, backend: &Backend) -> RespFrame {
        RespFrame::Integer(backend.sadd(self.key, self.values))
    }
}

impl CommandExecutor for SMembers {
    fn execute(self, backend: &Backend) -> RespFrame {
        // TODO: 返回的顺序 通过 unit test
        match backend.smembers(&self.key) {
            Some(values) => {
                let mut array = Vec::with_capacity(values.len());
                for value in values.iter() {
                    array.push(BulkString::from((*value).clone()).into());
                }
                RespArray::new(array).into()
            }
            None => RespArray::new([]).into(),
        }
    }
}

impl TryFrom<RespArray> for SAdd {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["sadd"], 2, true)?;

        let mut args = extract_args(value, 1)?.into_iter();
        match args.next() {
            Some(RespFrame::BulkString(key)) => {
                let values = args
                    .map(|f| match f {
                        RespFrame::BulkString(f) => Ok(String::from_utf8(f.0)?),
                        _ => Err(CommandError::InvalidArgument("Invalid value".to_string())),
                    })
                    .collect::<Result<Vec<String>, CommandError>>()?;
                Ok(SAdd {
                    key: String::from_utf8(key.0)?,
                    values,
                })
            }
            _ => Err(CommandError::InvalidArgument("Invalid key".to_string())),
        }
    }
}

impl TryFrom<RespArray> for SMembers {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["smembers"], 1, false)?;

        let mut args = extract_args(value, 1)?.into_iter();
        match args.next() {
            Some(RespFrame::BulkString(key)) => Ok(SMembers {
                key: String::from_utf8(key.0)?,
            }),
            _ => Err(CommandError::InvalidArgument("Invalid key".to_string())),
        }
    }
}
