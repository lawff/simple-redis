use std::ops::Deref;

use bytes::{Buf, BytesMut};

use crate::{RespDecode, RespEncode, RespError};

use super::{parse_length, CRLF_LEN};

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct BulkString(pub(crate) Vec<u8>);

// #[derive(Debug, PartialEq, Clone, Eq, Hash)]
// pub struct RespNullBulkString;

// Bulk strings: $<length>\r\n<data>\r\n
impl RespEncode for BulkString {
    fn encode(self) -> Vec<u8> {
        if self.is_empty() {
            return b"$-1\r\n".to_vec();
        }
        let mut buf = Vec::with_capacity(self.len() + 16);
        buf.extend_from_slice(format!("${}\r\n", self.len()).as_bytes());
        buf.extend_from_slice(&self);
        buf.extend_from_slice(b"\r\n");
        buf
    }
}

impl RespDecode for BulkString {
    const PREFIX: &'static str = "$";
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;
        if len == 0 {
            buf.advance(end + CRLF_LEN);
            return Ok(BulkString::new(Vec::new()));
        }
        let remained = &buf[end + CRLF_LEN..];
        if remained.len() < len + CRLF_LEN {
            return Err(RespError::NotComplete);
        }

        buf.advance(end + CRLF_LEN);

        let data = buf.split_to(len + CRLF_LEN);
        Ok(BulkString::new(&data[..len]))
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;
        if len == 0 {
            return Ok(end + CRLF_LEN);
        }
        Ok(end + CRLF_LEN + len + CRLF_LEN)
    }
}

// Null bulk strings: $-1\r\n
// impl RespEncode for RespNullBulkString {
//     fn encode(self) -> Vec<u8> {
//         b"$-1\r\n".to_vec()
//     }
// }

// impl RespDecode for RespNullBulkString {
//     const PREFIX: &'static str = "$";

//     fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
//         extract_fixed_data(buf, "$-1\r\n", "NullBulkString")?;
//         Ok(RespNullBulkString)
//     }

//     fn expect_length(_buf: &[u8]) -> Result<usize, RespError> {
//         Ok(5)
//     }
// }

impl Deref for BulkString {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl BulkString {
    pub fn new(s: impl Into<Vec<u8>>) -> Self {
        Self(s.into())
    }
}

impl From<&str> for BulkString {
    fn from(value: &str) -> Self {
        BulkString(value.as_bytes().to_vec())
    }
}

impl From<String> for BulkString {
    fn from(value: String) -> Self {
        BulkString(value.into_bytes())
    }
}

impl<const N: usize> From<&[u8; N]> for BulkString {
    fn from(s: &[u8; N]) -> Self {
        BulkString(s.to_vec())
    }
}

impl AsRef<[u8]> for BulkString {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::RespFrame;

    use super::*;
    use anyhow::Result;

    #[test]
    fn test_bulk_string_encode() {
        let frame: RespFrame = BulkString::new(b"hello".to_vec()).into();
        assert_eq!(frame.encode(), b"$5\r\nhello\r\n");
    }

    #[test]
    fn test_null_bulk_string_encode() {
        let frame: RespFrame = BulkString::new(Vec::new()).into();
        assert_eq!(frame.encode(), b"$-1\r\n");
    }

    #[test]
    fn test_bulk_string_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"$5\r\nhello\r\n");

        let frame = BulkString::decode(&mut buf)?;
        assert_eq!(frame, BulkString::new(b"hello"));

        buf.extend_from_slice(b"$5\r\nhello");
        let ret = BulkString::decode(&mut buf);
        assert_eq!(ret.unwrap_err(), RespError::NotComplete);

        buf.extend_from_slice(b"\r\n");
        let frame = BulkString::decode(&mut buf)?;
        assert_eq!(frame, BulkString::new(b"hello"));

        Ok(())
    }

    #[test]
    fn test_null_bulk_string_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"$-1\r\n");

        let frame = BulkString::decode(&mut buf)?;
        assert_eq!(frame, BulkString::new(Vec::new()));

        Ok(())
    }
}
