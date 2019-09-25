# tvheadproxy

The app proxies tvheadend so that it can be used with Plex DVR.

## Installation

Examples assume you ave the following:

- User on `tvheadend` called `livetvh` with password `mypassword`.
- Your `tvheadened` web interface is on `192.168.1.10:9981`.
- The machine you're running this on has IP address `192.168.1.10` and port `5004` is free.

### Local

```bash
cargo install tvheadproxy // add -f to update
tvheadproxy -h "http://192.168.1.10:5004" -t "http://192.168.1.10:9981" -u "livetvh" -p "mypassword"
```

### Docker

Example:

```bash
docker run -dit -p 5004:5004 simonhdickson/tvheadproxy -h "http://192.168.1.10:5004" -t "http://192.168.1.10:9981" -u "livetvh" -p "mypassword"
```

Then in Plex go to `Settings/Live TV PVR` click `Add Device` and type `http://192.168.1.10:5004`

## Help

```bash
tvheadproxy 0.1.0
TV Headend Proxy

USAGE:
    tvheadproxy [OPTIONS] --tvh_proxy_url <tvh-proxy-url> --tvh_url <tvh-url> --tvh_user <tvh-user>

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --port <port>                         [env: PORT=]  [default: 5004]
    -s, --stream_profile <stream-profile>     [env: STREAM_PROFILE=]  [default: pass]
    -c, --tuner_count <tuners>                [env: TUNER_COUNT=]  [default: 3]
    -w, --tv_weight <tv-weight>               [env: TV_WEIGHT=]  [default: 300]
    -p, --tvh_pass <tvh-pass>                 [env: TVH_PASS=]
    -h, --tvh_proxy_url <tvh-proxy-url>       [env: TVH_PROXY_URL=]
    -t, --tvh_url <tvh-url>                   [env: TVH_URL=]
    -u, --tvh_user <tvh-user>                 [env: TVH_USER=]
```
