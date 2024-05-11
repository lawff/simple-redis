use crate::{Backend, BulkString, RespArray, RespFrame};

use super::{extract_args, validate_command, CommandError, CommandExecutor, SAdd, SMembers};

impl CommandExecutor for SAdd {
    fn execute(self, backend: &Backend) -> RespFrame {
        RespFrame::Integer(backend.sadd(self.key, self.values))
    }
}

impl CommandExecutor for SMembers {
    fn execute(self, backend: &Backend) -> RespFrame {
        match backend.smembers(&self.key) {
            Some(values) => {
                let mut array = Vec::with_capacity(values.len());
                for value in values.iter() {
                    array.push((
                        value.key().to_owned(),
                        BulkString::from((*value).clone()).into(),
                    ));
                }
                if self.sort {
                    array.sort_by(|a, b| a.0.cmp(&b.0));
                }
                // 不需要返回 key
                let ret = array
                    .into_iter()
                    .map(|(_, v)| v)
                    .collect::<Vec<RespFrame>>();
                RespArray::new(ret).into()
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
                sort: false,
            }),
            _ => Err(CommandError::InvalidArgument("Invalid key".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::RespDecode;
    use anyhow::Result;
    use bytes::BytesMut;

    #[test]
    fn test_sadd_from_resp_array() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*3\r\n$4\r\nsadd\r\n$3\r\nkey\r\n$5\r\nhello\r\n");

        let frame = RespArray::decode(&mut buf)?;
        let ret: SAdd = frame.try_into()?;

        assert_eq!(ret.key, "key");
        assert_eq!(ret.values, vec!["hello"]);

        Ok(())
    }

    #[test]
    fn test_smembers_from_resp_array() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*2\r\n$8\r\nsmembers\r\n$3\r\nkey\r\n");

        let frame = RespArray::decode(&mut buf)?;
        let ret: SMembers = frame.try_into()?;

        assert_eq!(ret.key, "key");

        Ok(())
    }

    #[test]
    fn test_sadd_smembers_commands() {
        let backend = crate::Backend::new();
        let sadd = SAdd {
            key: "key".to_string(),
            values: vec!["hello".to_string(), "world".to_string()],
        };

        let frame: RespFrame = sadd.execute(&backend);
        let expected = RespFrame::Integer(2);

        assert_eq!(frame, expected);

        let smembers = SMembers {
            key: "key".to_string(),
            sort: true,
        };
        let frame: RespFrame = smembers.execute(&backend);
        let expected = RespFrame::Array(RespArray::new(vec![
            BulkString::from("hello").into(),
            BulkString::from("world").into(),
        ]));

        assert_eq!(frame, expected)
    }
}
