# nasne_exporter

## env

- `DEVICE_IP_ADDRS`: set the IP address of nasne separated by commas

## features

- `nasne_total_volume_size`
- `nasne_free_volume_size`
- `nasne_used_volume_size`

# for raspberry pi

armv7

```
$ docker build -t cross-for-pi .
$ docker run -it -v "$PWD":/app cross-for-pi
```
