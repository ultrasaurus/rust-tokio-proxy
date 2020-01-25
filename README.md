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

## Compile error

```
$ RUST_LOG=trace  cargo run 
   Compiling tcp-proxy v0.1.0 (/rust/tcp-proxy)
error[E0277]: `(dyn std::error::Error + 'static)` cannot be sent between threads safely
   --> src/main.rs:49:3
    |
49  |   tokio::spawn(async move {
    |   ^^^^^^^^^^^^ `(dyn std::error::Error + 'static)` cannot be sent between threads safely
    | 
   ::: /tokio-0.2.10/src/task/spawn.rs:123:21
    |
123 |         T: Future + Send + 'static,
    |                     ---- required by this bound in `tokio::task::spawn::spawn`
    |
    = help: the trait `std::marker::Send` is not implemented for `(dyn std::error::Error + 'static)`
    = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<(dyn std::error::Error + 'static)>`
    = note: required because it appears within the type `std::boxed::Box<(dyn std::error::Error + 'static)>`
    = note: required because it appears within the type `std::result::Result<(), std::boxed::Box<(dyn std::error::Error + 'static)>>`
    = note: required because it appears within the type `futures_util::future::maybe_done::MaybeDone<futures_util::future::try_future::into_future::IntoFuture<impl core::future::future::Future>>`
    = note: required because it appears within the type `futures_util::future::try_join::TryJoin<impl core::future::future::Future, impl core::future::future::Future>`
    = note: required because it appears within the type `for<'r, 's, 't0, 't1, 't2, 't3, 't4, 't5, 't6, 't7, 't8, 't9, 't10, 't11, 't12, 't13, 't14, 't15, 't16, 't17, 't18, 't19, 't20, 't21, 't22, 't23, 't24, 't25, 't26, 't27, 't28, 't29, 't30, 't31, 't32, 't33, 't34, 't35, 't36, 't37, 't38, 't39, 't40, 't41, 't42, 't43, 't44, 't45, 't46, 't47, 't48> {&'r mut tokio::net::tcp::listener::TcpListener, tokio::net::tcp::listener::TcpListener, impl core::future::future::Future, impl core::future::future::Future, (), tokio::net::tcp::stream::TcpStream, tokio::net::tcp::split::ReadHalf<'t1>, tokio::net::tcp::split::WriteHalf<'t2>, fn(&'t3 str) -> impl core::future::future::Future {tokio::net::tcp::stream::TcpStream::connect::<&'t3 str>}, &'t4 str, impl core::future::future::Future, impl core::future::future::Future, (), tokio::net::tcp::stream::TcpStream, tokio::net::tcp::split::ReadHalf<'t7>, tokio::net::tcp::split::WriteHalf<'t8>, impl core::future::future::Future, impl core::future::future::Future, fn(impl core::future::future::Future, impl core::future::future::Future) -> futures_util::future::try_join::TryJoin<impl core::future::future::Future, impl core::future::future::Future> {futures_util::future::try_join::try_join::<impl core::future::future::Future, impl core::future::future::Future>}, impl core::future::future::Future, impl core::future::future::Future, futures_util::future::try_join::TryJoin<impl core::future::future::Future, impl core::future::future::Future>, futures_util::future::try_join::TryJoin<impl core::future::future::Future, impl core::future::future::Future>, ()}`
    = note: required because it appears within the type `[static generator@src/main.rs:49:27: 63:4 listener:tokio::net::tcp::listener::TcpListener, dest_addr:&str for<'r, 's, 't0, 't1, 't2, 't3, 't4, 't5, 't6, 't7, 't8, 't9, 't10, 't11, 't12, 't13, 't14, 't15, 't16, 't17, 't18, 't19, 't20, 't21, 't22, 't23, 't24, 't25, 't26, 't27, 't28, 't29, 't30, 't31, 't32, 't33, 't34, 't35, 't36, 't37, 't38, 't39, 't40, 't41, 't42, 't43, 't44, 't45, 't46, 't47, 't48> {&'r mut tokio::net::tcp::listener::TcpListener, tokio::net::tcp::listener::TcpListener, impl core::future::future::Future, impl core::future::future::Future, (), tokio::net::tcp::stream::TcpStream, tokio::net::tcp::split::ReadHalf<'t1>, tokio::net::tcp::split::WriteHalf<'t2>, fn(&'t3 str) -> impl core::future::future::Future {tokio::net::tcp::stream::TcpStream::connect::<&'t3 str>}, &'t4 str, impl core::future::future::Future, impl core::future::future::Future, (), tokio::net::tcp::stream::TcpStream, tokio::net::tcp::split::ReadHalf<'t7>, tokio::net::tcp::split::WriteHalf<'t8>, impl core::future::future::Future, impl core::future::future::Future, fn(impl core::future::future::Future, impl core::future::future::Future) -> futures_util::future::try_join::TryJoin<impl core::future::future::Future, impl core::future::future::Future> {futures_util::future::try_join::try_join::<impl core::future::future::Future, impl core::future::future::Future>}, impl core::future::future::Future, impl core::future::future::Future, futures_util::future::try_join::TryJoin<impl core::future::future::Future, impl core::future::future::Future>, futures_util::future::try_join::TryJoin<impl core::future::future::Future, impl core::future::future::Future>, ()}]`
    = note: required because it appears within the type `std::future::GenFuture<[static generator@src/main.rs:49:27: 63:4 listener:tokio::net::tcp::listener::TcpListener, dest_addr:&str for<'r, 's, 't0, 't1, 't2, 't3, 't4, 't5, 't6, 't7, 't8, 't9, 't10, 't11, 't12, 't13, 't14, 't15, 't16, 't17, 't18, 't19, 't20, 't21, 't22, 't23, 't24, 't25, 't26, 't27, 't28, 't29, 't30, 't31, 't32, 't33, 't34, 't35, 't36, 't37, 't38, 't39, 't40, 't41, 't42, 't43, 't44, 't45, 't46, 't47, 't48> {&'r mut tokio::net::tcp::listener::TcpListener, tokio::net::tcp::listener::TcpListener, impl core::future::future::Future, impl core::future::future::Future, (), tokio::net::tcp::stream::TcpStream, tokio::net::tcp::split::ReadHalf<'t1>, tokio::net::tcp::split::WriteHalf<'t2>, fn(&'t3 str) -> impl core::future::future::Future {tokio::net::tcp::stream::TcpStream::connect::<&'t3 str>}, &'t4 str, impl core::future::future::Future, impl core::future::future::Future, (), tokio::net::tcp::stream::TcpStream, tokio::net::tcp::split::ReadHalf<'t7>, tokio::net::tcp::split::WriteHalf<'t8>, impl core::future::future::Future, impl core::future::future::Future, fn(impl core::future::future::Future, impl core::future::future::Future) -> futures_util::future::try_join::TryJoin<impl core::future::future::Future, impl core::future::future::Future> {futures_util::future::try_join::try_join::<impl core::future::future::Future, impl core::future::future::Future>}, impl core::future::future::Future, impl core::future::future::Future, futures_util::future::try_join::TryJoin<impl core::future::future::Future, impl core::future::future::Future>, futures_util::future::try_join::TryJoin<impl core::future::future::Future, impl core::future::future::Future>, ()}]>`
    = note: required because it appears within the type `impl core::future::future::Future`

error: aborting due to previous error
```