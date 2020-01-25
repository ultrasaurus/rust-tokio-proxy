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

