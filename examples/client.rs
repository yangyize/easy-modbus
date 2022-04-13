use std::error::Error;

use futures::SinkExt;
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;

use easy_modbus::{Frame, TcpClientCodec};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:8081".to_string();
    let stream = TcpStream::connect(&addr).await?;
    let mut transport = Framed::new(stream, TcpClientCodec);
    let frame = Frame::tcp();
    let request = frame.read_coils_request(0x01, 0x02, 0x08);

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
