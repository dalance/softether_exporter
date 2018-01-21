# softether-exporter
A prometheus exporter of SoftEther VPN server

[![Build Status](https://travis-ci.org/dalance/softether-exporter.svg?branch=master)](https://travis-ci.org/dalance/softether-exporter)

## Description

## Install
Download from [release page](https://github.com/dalance/softether-exporter/releases/latest), and extract to any directory ( e.g. `/usr/local/bin` ).
See the example files: `example/softether-exporter.service` and `example/config.toml`

If the release build doesn't fit your environment, you can build and install from source code.

```
cargo install softether-exporter
```

## Requirement

softether-exporter uses `vpncmd` or `vpncmd.exe` to access SoftEther VPN server.
The binary can be got from [SoftEther VPN Download](http://www.softether-download.com/?product=softether).

## Usage

```
softether-exporter [config_file]
```

The format of `config_file` is below.

```
listen_port = 9999              # listen_port of expoter
vpncmd      = "vpncmd"          # path to vpncmd binary
server      = "localhost:8888"  # address:port of SoftEther VPN server

[[hubs]]
name     = "HUB1" ## HUB name
password = "xxx"  ## HUB password

[[hubs]]
name     = "HUB2"
password = "yyy"
```
