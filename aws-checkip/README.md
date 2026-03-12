# aws-checkip - AWS Public IP Lookup

> **Status**: Pre-1.0, API may change. Feedback welcome.

Fetch your public IP address as seen by AWS from [`checkip.amazonaws.com`](https://checkip.amazonaws.com), returned as an [`IpNet`](https://docs.rs/ipnet/latest/ipnet/enum.IpNet.html) host address (`/32` for IPv4, `/128` for IPv6).

## Why this crate?

Your public IP can differ depending on which service you ask (due to CGNAT, split tunneling, or multi-homed routing). If you need the IP that AWS sees - for example, to add to a security group - you must ask AWS directly.

Existing crates like `public-ip` and `external-ip` query services like ipify.org, which may report a different address than what AWS observes.

## Usage

```rust
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), aws_checkip::Error> {
    let client = reqwest::Client::new();
    let net = aws_checkip::read_net(&client).await?;

    // Prints e.g. "203.0.113.42/32" or "2001:db8::1/128"
    println!("{net}");
    Ok(())
}
```

## Response Decoding

Response decoding uses [`typed-reqwest`](../typed-reqwest/)'s decoder infrastructure. The public `DECODER` static accepts HTTP 200 with any content type and parses the body as an IP address.

```rust,ignore
use aws_checkip::DECODER;

// Build a fake response for testing
let response: reqwest::Response = http::Response::builder()
    .status(http::StatusCode::OK)
    .body("203.0.113.42\n")
    .unwrap()
    .into();

let net = DECODER.decode(response).await.unwrap();
assert_eq!(net, "203.0.113.42/32".parse::<ipnet::IpNet>().unwrap());
```
