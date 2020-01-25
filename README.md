# TCP Proxy

```
RUST_LOG=trace  cargo run 
```

Then after running proxy, run TCP server listening on 1935 and make client 
connection on 19355.

For example, run an RTMP server (default port 1935) then...
```
rtmpdump -V --stop 1 -r rtmp://localhost:19355/vod/media -o test.flv
```



## example reader-type -- compile error

```
RUST_LOG=trace  cargo run --example reader-type
```

```
warning: trait objects without an explicit `dyn` are deprecated
 --> examples/reader-type.rs:9:15
  |
9 | type Reader = AsyncRead + Send + Unpin;
  |               ^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn AsyncRead + Send + Unpin`
  |
  = note: `#[warn(bare_trait_objects)]` on by default

error[E0277]: the size for values of type `(dyn tokio::io::async_read::AsyncRead + std::marker::Send + std::marker::Unpin + 'static)` cannot be known at compilation time
  --> examples/reader-type.rs:11:24
   |
11 | async fn read_some<'a>(mut reader: Reader) -> Result<(), std::io::Error>
   |                        ^^^^^^^^^^ doesn't have a size known at compile-time
   |
   = help: the trait `std::marker::Sized` is not implemented for `(dyn tokio::io::async_read::AsyncRead + std::marker::Send + std::marker::Unpin + 'static)`
   = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
   = note: all local variables must have a statically known size
   = help: unsized locals are gated as an unstable feature

error[E0308]: mismatched types
  --> examples/reader-type.rs:44:15
   |
44 |     read_some(reader).await.expect("read_stuff");
   |               ^^^^^^ expected trait tokio::io::async_read::AsyncRead, found struct `tokio::net::tcp::split::ReadHalf`
   |
   = note: expected type `(dyn tokio::io::async_read::AsyncRead + std::marker::Send + std::marker::Unpin + 'static)`
              found type `tokio::net::tcp::split::ReadHalf<'_>`

error[E0277]: the size for values of type `(dyn tokio::io::async_read::AsyncRead + std::marker::Send + std::marker::Unpin + 'static)` cannot be known at compilation time
  --> examples/reader-type.rs:44:5
   |
44 |     read_some(reader).await.expect("read_stuff");
   |     ^^^^^^^^^ doesn't have a size known at compile-time
   |
   = help: the trait `std::marker::Sized` is not implemented for `(dyn tokio::io::async_read::AsyncRead + std::marker::Send + std::marker::Unpin + 'static)`
   = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
   = note: all function arguments must have a statically known size
   = help: unsized locals are gated as an unstable feature

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `tcp-proxy`.

```