# TCP Proxy

```
RUST_LOG=trace  cargo run 
```


## example

after running proxy, run RTMP server, listening on 1935
then...
```
rtmpdump -V --live -r rtmp://localhost:19355/live/cameraFeed -o test.flv
```