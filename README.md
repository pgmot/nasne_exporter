# nasne_exporter

## env

- `DEVICE_IP_ADDRS`: set the IP address of nasne separated by commas

## features

- `nasne_total_volume_size`
- `nasne_free_volume_size`
- `nasne_used_volume_size`

# for raspberry pi

```
rustup target add aarch64-unknown-linux-gnu
cargo build --target aarch64-unknown-linux-gnu --release
```