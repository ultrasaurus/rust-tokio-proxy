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
error[E0599]: no method named `read` found for type `Reader` in the current scope
  --> examples/reader-type.rs:16:24
   |
16 |   let n = match reader.read(&mut buf).await {
   |                        ^^^^ method not found in `Reader`
   |
   = note: the method `read` exists but the following trait bounds were not satisfied:
           `Reader : tokio::io::util::async_read_ext::AsyncReadExt`
   = help: items from traits can only be used if the type parameter is bounded by the trait
help: the following traits define an item `read`, perhaps you need to restrict type parameter `Reader` with one of them:
   |
13 | async fn read_some<'a, Reader: std::io::Read>(mut reader: Reader) -> Result<(), std::io::Error>
   |                        ^^^^^^^^^^^^^^^^^^^^^
13 | async fn read_some<'a, Reader: tokio::io::util::async_read_ext::AsyncReadExt>(mut reader: Reader) -> Result<(), std::io::Error>
   |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

```