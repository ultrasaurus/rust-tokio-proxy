use tokio::net::TcpStream;
extern crate pretty_env_logger;
#[macro_use] extern crate log;

#[tokio::main]
async fn main() {
  pretty_env_logger::init();

  let addr = "127.0.0.1:1935";

  let tcp = TcpStream::connect(addr).await.expect("tcp connection failed");
  tcp.set_nodelay(true).expect("set_nodelay call failed");

  let mut input = String::new();
  println!("press return to quit");
  std::io::stdin().read_line(&mut input).expect("stdio read_line");
}


