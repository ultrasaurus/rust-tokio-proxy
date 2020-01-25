use std::error::Error;
use tokio::net::TcpListener;
use tokio::prelude::*;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

// type Reader = AsyncRead + Send + Unpin;

trait Reader: AsyncRead + Send + Unpin { } 
impl<T: AsyncRead + Send + Unpin> Reader for T {}

async fn read_some<'a, Reader>(mut reader: Reader) -> Result<(), std::io::Error>
{
  let mut buf = [0; 2];
  let n = match reader.read(&mut buf).await {
    // socket closed
    Ok(n) if n == 0 => {
      info!("src socket closed");
      return Ok(())
    },
    Ok(n) => n,
    Err(e) => {
      eprintln!("failed to read from socket; err = {:?}", e);
      return Err(e)
    }
  };
  info!("read {} bytes", n);
  Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  pretty_env_logger::init();

  let addr = "127.0.0.1:1234";

  info!("listen on {}", addr);
  let mut listener = TcpListener::bind(addr).await.expect("tcp listen bind");

  tokio::spawn(async move {
    let ( mut tcp, _) = listener.accept().await.expect("tcp listener accept");

    let ( reader, _writer) = tcp.split();

    read_some(reader).await.expect("read_stuff");

  });

  let mut input = String::new();
  println!("press return to quit");
  std::io::stdin().read_line(&mut input).expect("stdio read_line");
  Ok(())
}

