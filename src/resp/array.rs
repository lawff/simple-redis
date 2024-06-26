use std::ops::Deref;

use bytes::{Buf, BytesMut};

use crate::{RespDecode, RespEncode, RespError, RespFrame};

use super::{calc_total_length, parse_length, BUF_CAP, CRLF_LEN};

#[derive(Debug, PartialEq, Clone)]
pub struct RespArray(pub(crate) Vec<RespFrame>);

// #[derive(Debug, PartialEq, Clone, Eq)]
// pub struct RespNullArray;

// Arrays: *<number-of-elements>\r\n<element-1>...<element-n>
impl RespEncode for RespArray {
    fn encode(self) -> Vec<u8> {
        if self.is_empty() {
            return b"*-1\r\n".to_vec();
        }
        let mut buf = Vec::with_capacity(BUF_CAP);
        buf.extend_from_slice(format!("*{}\r\n", self.len()).as_bytes());
        for element in self.0 {
            buf.extend_from_slice(&element.encode());
        }
        buf
    }
}

impl RespDecode for RespArray {
    const PREFIX: &'static str = "*";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;

        if len == 0 {
            buf.advance(end + CRLF_LEN);
            return Ok(RespArray::new(Vec::new()));
        }

        let total = calc_total_length(buf, end, len, Self::PREFIX)?;
        if buf.len() < total {
            return Err(RespError::NotComplete);
        }

        buf.advance(end + CRLF_LEN);

        let mut frames = Vec::with_capacity(len);
        for _ in 0..len {
            let frame = RespFrame::decode(buf)?;
            frames.push(frame);
        }

        Ok(RespArray::new(frames))
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;
        if len == 0 {
            return Ok(end + CRLF_LEN);
        }
        calc_total_length(buf, end, len, Self::PREFIX)
    }
}

// Null arrays: *-1\r\n
// impl RespEncode for RespNullArray {
//     fn encode(self) -> Vec<u8> {
//         b"*-1\r\n".to_vec()
//     }
// }

// impl RespDecode for RespNullArray {
//     const PREFIX: &'static str = "*";

//     fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
//         extract_fixed_data(buf, "*-1\r\n", "NullArray")?;
//         Ok(RespNullArray)
//     }

//     fn expect_length(_buf: &[u8]) -> Result<usize, RespError> {
//         Ok(5)
//     }
// }

impl RespArray {
    pub fn new(s: impl Into<Vec<RespFrame>>) -> Self {
        Self(s.into())
    }
}

impl Deref for RespArray {
    type Target = Vec<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BulkString;
    use anyhow::Result;

    #[test]
    fn test_array_encode() {
        let frame: RespFrame = RespArray::new(vec![
            BulkString::new("set".to_string()).into(),
            BulkString::new("hello".to_string()).into(),
            BulkString::new("world".to_string()).into(),
        ])
        .into();
        assert_eq!(
            &frame.encode(),
            b"*3\r\n$3\r\nset\r\n$5\r\nhello\r\n$5\r\nworld\r\n"
        );
    }

    #[test]
    fn test_null_array_encode() {
        let frame: RespFrame = RespArray::new(Vec::new()).into();
        assert_eq!(frame.encode(), b"*-1\r\n");
    }

    #[test]
    fn test_null_array_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*-1\r\n");

        let frame = RespArray::decode(&mut buf)?;
        assert_eq!(frame, RespArray::new(Vec::new()));

        Ok(())
    }

    #[test]
    fn test_array_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*2\r\n$3\r\nset\r\n$5\r\nhello\r\n");

        let frame = RespArray::decode(&mut buf)?;
        assert_eq!(frame, RespArray::new([b"set".into(), b"hello".into()]));

        buf.extend_from_slice(b"*2\r\n$3\r\nset\r\n");
        let ret = RespArray::decode(&mut buf);
        assert_eq!(ret.unwrap_err(), RespError::NotComplete);

        buf.extend_from_slice(b"$5\r\nhello\r\n");
        let frame = RespArray::decode(&mut buf)?;
        assert_eq!(frame, RespArray::new([b"set".into(), b"hello".into()]));

        Ok(())
    }
}
