# TCP Proxy

```
RUST_LOG=trace  cargo run 
```

Then after running proxy, run TCP server listening on 1935 and make client 
connection on 19355.

For example, run an RTMP server (default port 1935) then...
```
rtmpdump -V --live -r rtmp://localhost:19355/live/cameraFeed -o test.flv
```

# Compile error

````
$ cargo run
   Compiling tcp-proxy v0.1.0 (/Users/sallen/src/rust/tcp-proxy)
warning: unused import: `tokio::io`
 --> src/main.rs:4:5
  |
4 | use tokio::io;
  |     ^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

error[E0310]: the parameter type `Reader` may not live long enough
  --> src/main.rs:14:5
   |
9  | async fn pipe<'a, Reader: AsyncRead + Send + Unpin, Writer: AsyncWrite + Send + Unpin>
   |                   ------- help: consider adding an explicit lifetime bound `Reader: 'static`...
...
14 |     tokio::spawn(async move {
   |     ^^^^^^^^^^^^
   |
note: ...so that the type `impl std::future::Future` will meet its required lifetime bounds
  --> src/main.rs:14:5
   |
14 |     tokio::spawn(async move {
   |     ^^^^^^^^^^^^

error[E0310]: the parameter type `Writer` may not live long enough
  --> src/main.rs:14:5
   |
9  | async fn pipe<'a, Reader: AsyncRead + Send + Unpin, Writer: AsyncWrite + Send + Unpin>
   |                                                     ------- help: consider adding an explicit lifetime bound `Writer: 'static`...
...
14 |     tokio::spawn(async move {
   |     ^^^^^^^^^^^^
   |
note: ...so that the type `impl std::future::Future` will meet its required lifetime bounds
  --> src/main.rs:14:5
   |
14 |     tokio::spawn(async move {
   |     ^^^^^^^^^^^^

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0310`.
error: could not compile `tcp-proxy`.

