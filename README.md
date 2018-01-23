# softether_exporter
[Prometheus](https://prometheus.io) exporter for [SoftEther VPN server](http://www.softether.org)

[![Build Status](https://travis-ci.org/dalance/softether_exporter.svg?branch=master)](https://travis-ci.org/dalance/softether_exporter)
[![Crates.io](https://img.shields.io/crates/v/softether_exporter.svg)](https://crates.io/crates/softether_exporter)

## Description

softether_exporter is a prometheus exporter for SoftEther VPN server.
The provided metrics are below.

| metric                               | description                            | labels                         |
| ------------------------------------ | -------------------------------------- | ------------------------------ |
| softether_up                         | The last query is valid                | hub                            |
| softether_online                     | Hub is online                          | hub                            |
| softether_sessions_total             | Total number of sessions               | hub                            |
| softether_sessions_client_total      | Total number of client sessions        | hub                            |
| softether_sessions_bridge_total      | Total number of bridge sessions        | hub                            |
| softether_users_total                | Total number of users                  | hub                            |
| softether_groups_total               | Total number of groups                 | hub                            |
| softether_mac_tables_total           | Total number of entries in MAC table   | hub                            |
| softether_ip_tables_total            | Total number of entries in IP table    | hub                            |
| softether_logins_total               | Total number of logins                 | hub                            |
| softether_outgoing_unicast_packets   | Outgoing unicast transfer in packets   | hub                            |
| softether_outgoing_unicast_bytes     | Outgoing unicast transfer in bytes     | hub                            |
| softether_outgoing_broadcast_packets | Outgoing broadcast transfer in packets | hub                            |
| softether_outgoing_broadcast_bytes   | Outgoing broadcast transfer in bytes   | hub                            |
| softether_incoming_unicast_packets   | Incoming unicast transfer in packets   | hub                            |
| softether_incoming_unicast_bytes     | Incoming unicast transfer in bytes     | hub                            |
| softether_incoming_broadcast_packets | Incoming broadcast transfer in packets | hub                            |
| softether_incoming_broadcast_bytes   | Incoming broadcast transfer in bytes   | hub                            |
| softether_build_info                 | Build information                      | version, revision, rustversion |

Each metric has `hub` label. For example, outgoing unicast packet rate of HUB1 is below.

```
rate(softether_outgoing_unicast_packets{hub="HUB1"}[1m])
```

## Install
Download from [release page](https://github.com/dalance/softether_exporter/releases/latest), and extract to any directory ( e.g. `/usr/local/bin` ).
See the example files in `example` directory as below.

| File                               | Description                    |
| ---------------------------------- | ------------------------------ |
| example/softether_exporter.service | systemd unit file              |
| example/config.toml                | softether_exporter config file |


If the release build doesn't fit your environment, you can build and install from source code.

```
cargo install softether_exporter
```

## Requirement

softether_exporter uses `vpncmd` or `vpncmd.exe` to access SoftEther VPN server.
The binary can be got from [SoftEther VPN Download](http://www.softether-download.com/?product=softether).

## Usage

```
softether_exporter [config_file]
```

The format of `config_file` is below.

```
listen_port = 9411                    # listen_port of expoter ( 9411 is the default port of softether_exporter )
vpncmd      = "/usr/local/bin/vpncmd" # path to vpncmd binary
server      = "localhost:8888"        # address:port of SoftEther VPN server

[[hubs]]
name     = "HUB1" # HUB name
password = "xxx"  # HUB password

[[hubs]]
name     = "HUB2"
password = "yyy"
```
