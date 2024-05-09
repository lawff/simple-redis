use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

use bytes::BytesMut;
use enum_dispatch::enum_dispatch;
use thiserror::Error;

mod decode;
mod encode;

#[enum_dispatch]
pub trait RespEncode {
    fn encode(self) -> Vec<u8>;
}

pub trait RespDecode: Sized {
    const PREFIX: &'static str;
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError>;
    fn expect_length(buf: &[u8]) -> Result<usize, RespError>;
}

#[derive(Debug, Error, PartialEq)]
pub enum RespError {
    #[error("Invalid frame: {0}")]
    InvalidFrame(String),
    #[error("Invalid frame type: {0}")]
    InvalidFrameType(String),
    #[error("Invalid frame length： {0}")]
    InvalidFrameLength(isize),
    #[error("Frame is not complete")]
    NotComplete,

    #[error("Parse error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Parse float error: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
}

#[enum_dispatch(RespEncode)]
#[derive(Debug, PartialEq, Clone)]
pub enum RespFrame {
    SimpleString(SimpleString),
    Error(SimpleError),
    Integer(i64),

    BulkString(BulkString),
    NullBulkString(RespNullBulkString),

    Array(RespArray),
    NullArray(RespNullArray),

    Null(RespNull),
    Boolean(bool),
    Double(f64),
    Map(RespMap),
    Set(RespSet),
}

#[derive(Debug, PartialEq, Clone)]
pub struct SimpleString(String);
#[derive(Debug, PartialEq, Clone)]
pub struct SimpleError(String);
#[derive(Debug, PartialEq, Clone)]
pub struct BulkString(pub(crate) Vec<u8>);
#[derive(Debug, PartialEq, Clone)]
pub struct RespNullBulkString;
#[derive(Debug, PartialEq, Clone)]
pub struct RespArray(pub(crate) Vec<RespFrame>);
#[derive(Debug, PartialEq, Clone)]
pub struct RespNullArray;
#[derive(Debug, PartialEq, Clone)]
pub struct RespNull;
#[derive(Debug, PartialEq, Clone)]
pub struct RespMap(BTreeMap<String, RespFrame>);
#[derive(Debug, PartialEq, Clone)]
pub struct RespSet(Vec<RespFrame>);

impl Deref for BulkString {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for RespArray {
    type Target = Vec<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for RespMap {
    type Target = BTreeMap<String, RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RespMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for RespSet {
    type Target = Vec<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SimpleString {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

impl SimpleError {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

impl BulkString {
    pub fn new(s: impl Into<Vec<u8>>) -> Self {
        Self(s.into())
    }
}

impl RespArray {
    pub fn new(s: impl Into<Vec<RespFrame>>) -> Self {
        Self(s.into())
    }
}

impl RespMap {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }
}

impl Default for RespMap {
    fn default() -> Self {
        Self::new()
    }
}

impl RespSet {
    pub fn new(s: impl Into<Vec<RespFrame>>) -> Self {
        Self(s.into())
    }
}

impl<const N: usize> From<&[u8; N]> for BulkString {
    fn from(s: &[u8; N]) -> Self {
        BulkString(s.to_vec())
    }
}

impl<const N: usize> From<&[u8; N]> for RespFrame {
    fn from(s: &[u8; N]) -> Self {
        BulkString(s.to_vec()).into()
    }
}

impl AsRef<[u8]> for BulkString {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
