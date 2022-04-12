use easy_modbus::{ClientCodec, Request, Response, ServerCodec};
use futures::SinkExt;
use std::error::Error;
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:8081".to_string();
    let stream = TcpStream::connect(&addr).await?;
    let mut transport = Framed::new(stream, ClientCodec);
    let request = Request::read_coils(0x01, 0x01, 0x02, 0x08);

    transport.send(request).await?;
    while let Some(response) = transport.next().await {
        match response {
            Ok(response) => {
                println!("{:?}", response);
            }
            Err(e) => return Err(e.into()),
        }
    }
    Ok(())
}
