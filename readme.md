## Cloudflare DNS API Client

This is just a simple poc to consume cloudflare DNS api written in rust using `reqwest` crates.


### to try
To try this project just simply get your Secret key from Cloudflre, because thanks to cloudflare all we need to do is to set `X-Auth-Email` and `X-Auth-Key` header. And also you need to get your `$ZONE_ID` for interacting with your domain. 

Then run these to test :
```
cargo run 
```
or to build :
```
cargo build 
target/debug/cloudflare-dns-api
```

But sometimes you may need to set your build target. I'am using linux btw