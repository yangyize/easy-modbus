//! Codec based [tokio-util](https://docs.rs/tokio-util/latest/tokio_util/codec/index.html)

mod decoder;
mod encoder;

/// Mutual convert TCP Client frames and buffers.
#[derive(Debug, Default)]
pub struct TcpClientCodec;

/// Mutual convert TCP Server frames and buffers.
#[derive(Debug, Default)]
pub struct TcpServerCodec;

/// Mutual convert RTU Client frames and buffers.
#[derive(Debug, Default)]
pub struct RtuClientCodec;

/// Mutual convert RTU Server frames and buffers.
#[derive(Debug, Default)]
pub struct RtuServerCodec;
