use chrono::offset::Utc;
use chrono::DateTime;
use std::time::SystemTime;
use futures::future::try_join;
use std::env;
use std::error::Error;
use tokio::net::{TcpStream, TcpListener};
use tokio::prelude::*;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

trait AsyncReader: AsyncRead + Send + Unpin { } 
impl<T: AsyncRead + Send + Unpin> AsyncReader for T {}

trait AsyncWriter: AsyncWrite + Send + Unpin { } 
impl<T: AsyncWrite + Send + Unpin> AsyncWriter for T {}


async fn pipe<R,W>(label: &str, mut reader: R, mut writer: W) -> Result<(), Box<dyn Error + Send + Sync + 'static>>
where
  R: AsyncReader,
  W: AsyncWriter
{
  let mut buf = [0; 2048];
  // In a loop, read data from the src and write to the dest.
  loop {
    let n = match reader.read(&mut buf).await {
      // socket closed
      Ok(n) if n == 0 => {
        info!("src socket closed");
        return Ok(())
      },
      Ok(n) => n,
      Err(e) => {
        eprintln!("failed to read from socket; err = {:?}", e);
        return Ok(())
      }
    };
    let system_time = SystemTime::now();
    let datetime: DateTime<Utc> = system_time.into();
    // println!("{}", datetime.format("%F-%T-%f"));

    info!(target: label, "{} bytes at {}", n, datetime.format("%F-%T-%f"));


    // let mut output = String::new();
    // for i in 0..n {
    //   let byte = buf[i];
    //   output.push_str(&format!("{:02x} ", byte));
    // }
    // info!(target: label, "{}", output);

    // Write the data back
    if let Err(e) = writer.write_all(&buf[0..n]).await {
      eprintln!("failed to write to socket; err = {:?}", e);
      return Ok(())
    }
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  pretty_env_logger::init();
  let args: Vec<String> = env::args().collect();
  println!("{:?}", args);

  let src_addr = "127.0.0.1:19355";
  let dest_addr = "127.0.0.1:1935";

  info!("listen on {}", src_addr);
  let mut listener = TcpListener::bind(src_addr).await.expect("tcp listen bind");
  tokio::spawn(async move {
    let ( mut src, _) = listener.accept().await.expect("tcp listener accept");
    let ( src_reader, src_writer) = src.split();

    info!("connect to {}", dest_addr);
    let mut dest = TcpStream::connect(dest_addr).await.expect("tcp connection to destination");
    dest.set_nodelay(true).expect("dest tcp set_nodelay");
    let ( dest_reader, dest_writer) = dest.split();

    let dest_to_src = pipe("client", src_reader, dest_writer);
    let src_to_dest = pipe("server", dest_reader, src_writer);

    try_join(dest_to_src, src_to_dest).await.expect("try join");

  });

  let mut input = String::new();
  println!("press return to quit");
  std::io::stdin().read_line(&mut input).expect("stdio read_line");
  Ok(())
}


