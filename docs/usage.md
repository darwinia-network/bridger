Darwinia bridger
===

## Env

The bridger program will read some environments.

| Name             | Description                                                                                                                       |
|------------------|-----------------------------------------------------------------------------------------------------------------------------------|
| `BRIDGER_HOME`   | All data of bridger and bridges will be store in this path. If not set this value, the default will be store in `$HOME/.bridger`. |
| `LOG_ADAPTER`    | Custom log adapter, you can use `json` value. after do this, all logs will output use json format.                                |
| `LOG_MAX_LEVEL`  | Max log level, default is `trace`                                                                                                 |
| `RUST_BACKTRACE` | You can set  `1` or `0` to enable or disable error full error trace                                                               |
| `RUST_LOG`       | Custom log level for target or crate                                                                                              |

## Compile

```bash
git clone https://github.com/darwinia-network/bridger
cd bridger
```

### Set registry

You need set registry when you want to compile from source, because default
registry from download precompile binary from Github release page. So set
registry to local

```text
./bridger.sh registry set --type local
```

### Run

pangolin-ropsten
```bash
./bridger.sh pangolin-ropsten -h
```

pangolin-pangoro
```bash
./bridger.sh pangolin-pangoro -h
```


## Precompile

Please download latest bridger binary from [Github release page](https://github.com/darwinia-network/bridger/releases).

When you get bridger binary, you can direct run

```bash
bridger pangolin-ropsten -h
```

```bash
bridger pangolin-pangoro -h
```
