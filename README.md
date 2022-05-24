# Easy Modbus
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-red.svg
[mit-url]: https://github.com/yangyize/easy-modbus/blob/main/LICENSE
[actions-badge]: https://github.com/yangyize/easy-modbus/actions/workflows/main.yml/badge.svg?branch=main
[actions-url]: https://github.com/yangyize/easy-modbus/actions/workflows/main.yml

A Rust Modbus library.

# Examples

A simple Modbus TCP Server:

```rust,no_run
use std::error::Error;
//!
use futures::SinkExt;
use tokio::net::{TcpListener, TcpStream};
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;
//!
use easy_modbus::{Frame, codec::TcpServerCodec};
//!
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:502".to_string();
    let server = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);
//!
    loop {
        let (stream, _) = server.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = process(stream).await {
                println!("failed to  process connection; error = {}", e);
            }
        });
    }
}
//!
async fn process(stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut transport = Framed::new(stream, TcpServerCodec);
    let frame = Frame::tcp();
    while let Some(request) = transport.next().await {
        match request {
            Ok(request) => {
                println!("load request --- {:?}", request);
                let response = frame.read_coils_response(0x01, vec![0x00, 0x01]);
                println!("send response --- {:?}", response);
                transport.send(response).await?;
            }
            Err(e) => return Err(e.into()),
        }
    }
    Ok(())
}
```

A simple Modbus TCP Client:

``` rust,no_run
use std::error::Error;
//!
use futures::SinkExt;
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;
//!
use easy_modbus::{Frame, codec::TcpClientCodec};
//!
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:502".to_string();
    let stream = TcpStream::connect(&addr).await?;
    let mut transport = Framed::new(stream, TcpClientCodec);
    let frame = Frame::tcp();
    let request = frame.read_coils_request(0x01, 0x02, 0x08);
//!
    transport.send(request).await?;
    while let Some(response) = transport.next().await {
        match response {
            Ok(response) => {
                println!("{:?}", response);
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
    Ok(())
}
```

## Source Code Mirror
[sourcehut](https://git.sr.ht/~yangyize/easy-modbus)

## License

This project is licensed under the [MIT license].

[MIT license]: https://github.com/yangyize/easy-modbus/blob/main/LICENSE
