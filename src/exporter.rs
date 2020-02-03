use crate::softether_reader::SoftEtherReader;
use hyper::header::ContentType;
use hyper::mime::{Mime, SubLevel, TopLevel};
use hyper::server::{Request, Response, Server};
use hyper::uri::RequestUri;
use lazy_static::lazy_static;
use prometheus;
use prometheus::{register_gauge_vec, Encoder, GaugeVec, TextEncoder, __register_gauge_vec, opts};
use serde_derive::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use toml;

lazy_static! {
    static ref UP: GaugeVec =
        register_gauge_vec!("softether_up", "The last query is successful.", &["hub"]).unwrap();
    static ref ONLINE: GaugeVec =
        register_gauge_vec!("softether_online", "Hub online.", &["hub"]).unwrap();
    static ref SESSIONS: GaugeVec =
        register_gauge_vec!("softether_sessions", "Number of sessions.", &["hub"]).unwrap();
    static ref SESSIONS_CLIENT: GaugeVec = register_gauge_vec!(
        "softether_sessions_client",
        "Number of client sessions.",
        &["hub"]
    )
    .unwrap();
    static ref SESSIONS_BRIDGE: GaugeVec = register_gauge_vec!(
        "softether_sessions_bridge",
        "Number of bridge sessions.",
        &["hub"]
    )
    .unwrap();
    static ref USERS: GaugeVec =
        register_gauge_vec!("softether_users", "Number of users.", &["hub"]).unwrap();
    static ref GROUPS: GaugeVec =
        register_gauge_vec!("softether_groups", "Number of groups.", &["hub"]).unwrap();
    static ref MAC_TABLES: GaugeVec = register_gauge_vec!(
        "softether_mac_tables",
        "Number of entries in MAC table.",
        &["hub"]
    )
    .unwrap();
    static ref IP_TABLES: GaugeVec = register_gauge_vec!(
        "softether_ip_tables",
        "Number of entries in IP table.",
        &["hub"]
    )
    .unwrap();
    static ref LOGINS: GaugeVec =
        register_gauge_vec!("softether_logins", "Number of logins.", &["hub"]).unwrap();
    static ref OUTGOING_UNICAST_PACKETS: GaugeVec = register_gauge_vec!(
        "softether_outgoing_unicast_packets",
        "Outgoing unicast transfer in packets.",
        &["hub"]
    )
    .unwrap();
    static ref OUTGOING_UNICAST_BYTES: GaugeVec = register_gauge_vec!(
        "softether_outgoing_unicast_bytes",
        "Outgoing unicast transfer in bytes.",
        &["hub"]
    )
    .unwrap();
    static ref OUTGOING_BROADCAST_PACKETS: GaugeVec = register_gauge_vec!(
        "softether_outgoing_broadcast_packets",
        "Outgoing broadcast transfer in packets.",
        &["hub"]
    )
    .unwrap();
    static ref OUTGOING_BROADCAST_BYTES: GaugeVec = register_gauge_vec!(
        "softether_outgoing_broadcast_bytes",
        "Outgoing broadcast transfer in bytes.",
        &["hub"]
    )
    .unwrap();
    static ref INCOMING_UNICAST_PACKETS: GaugeVec = register_gauge_vec!(
        "softether_incoming_unicast_packets",
        "Incoming unicast transfer in packets.",
        &["hub"]
    )
    .unwrap();
    static ref INCOMING_UNICAST_BYTES: GaugeVec = register_gauge_vec!(
        "softether_incoming_unicast_bytes",
        "Incoming unicast transfer in bytes.",
        &["hub"]
    )
    .unwrap();
    static ref INCOMING_BROADCAST_PACKETS: GaugeVec = register_gauge_vec!(
        "softether_incoming_broadcast_packets",
        "Incoming broadcast transfer in packets.",
        &["hub"]
    )
    .unwrap();
    static ref INCOMING_BROADCAST_BYTES: GaugeVec = register_gauge_vec!(
        "softether_incoming_broadcast_bytes",
        "Incoming broadcast transfer in bytes.",
        &["hub"]
    )
    .unwrap();
    static ref BUILD_INFO: GaugeVec = register_gauge_vec!(
        "softether_build_info",
        "A metric with a constant '1' value labeled by version, revision and rustversion",
        &["version", "revision", "rustversion"]
    )
    .unwrap();
}

static LANDING_PAGE: &'static str = "<html>
<head><title>SoftEther Exporter</title></head>
<body>
<h1>SoftEther Exporter</h1>
<p><a href=\"/metrics\">Metrics</a></p>
</body>
";

static VERSION: &'static str = env!("CARGO_PKG_VERSION");
static GIT_REVISION: Option<&'static str> = option_env!("GIT_REVISION");
static RUST_VERSION: Option<&'static str> = option_env!("RUST_VERSION");

#[derive(Debug, Deserialize)]
pub struct Config {
    listen_port: Option<u32>,
    vpncmd: Option<String>,
    server: Option<String>,
    hubs: Vec<Hub>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Hub {
    name: Option<String>,
    password: Option<String>,
}

impl Config {
    pub fn from_file(file: &str) -> Result<Config, Box<dyn Error>> {
        let mut f = File::open(file)?;
        let mut s = String::new();
        let _ = f.read_to_string(&mut s);
        let config: Config = toml::from_str(&s)?;
        Ok(config)
    }
}

pub struct Exporter;

impl Exporter {
    pub fn start(config: Config) -> Result<(), Box<dyn Error>> {
        let encoder = TextEncoder::new();
        let addr = format!("0.0.0.0:{}", config.listen_port.unwrap_or(9411));
        let vpncmd = config.vpncmd.unwrap_or(String::from("vpncmd"));
        let server = config.server.unwrap_or(String::from("localhost"));
        let hubs = config.hubs;

        println!("Server started: {}", addr);

        Server::http(addr)?.handle(move |req: Request, mut res: Response| {
            if req.uri == RequestUri::AbsolutePath("/metrics".to_string()) {
                for hub in hubs.clone() {
                    let name = hub.name.unwrap_or(String::from(""));
                    let password = hub.password.unwrap_or(String::from(""));
                    let status =
                        match SoftEtherReader::hub_status(&vpncmd, &server, &name, &password) {
                            Ok(x) => x,
                            Err(x) => {
                                UP.with_label_values(&[&name]).set(0.0);
                                println!("Hub status read failed: {}", x);
                                continue;
                            }
                        };

                    UP.with_label_values(&[&status.name]).set(1.0);
                    ONLINE
                        .with_label_values(&[&status.name])
                        .set(if status.online { 1.0 } else { 0.0 });
                    SESSIONS
                        .with_label_values(&[&status.name])
                        .set(status.sessions);
                    SESSIONS_CLIENT
                        .with_label_values(&[&status.name])
                        .set(status.sessions_client);
                    SESSIONS_BRIDGE
                        .with_label_values(&[&status.name])
                        .set(status.sessions_bridge);
                    USERS.with_label_values(&[&status.name]).set(status.users);
                    GROUPS.with_label_values(&[&status.name]).set(status.groups);
                    MAC_TABLES
                        .with_label_values(&[&status.name])
                        .set(status.mac_tables);
                    IP_TABLES
                        .with_label_values(&[&status.name])
                        .set(status.ip_tables);
                    LOGINS.with_label_values(&[&status.name]).set(status.logins);
                    OUTGOING_UNICAST_PACKETS
                        .with_label_values(&[&status.name])
                        .set(status.outgoing_unicast_packets);
                    OUTGOING_UNICAST_BYTES
                        .with_label_values(&[&status.name])
                        .set(status.outgoing_unicast_bytes);
                    OUTGOING_BROADCAST_PACKETS
                        .with_label_values(&[&status.name])
                        .set(status.outgoing_broadcast_packets);
                    OUTGOING_BROADCAST_BYTES
                        .with_label_values(&[&status.name])
                        .set(status.outgoing_broadcast_bytes);
                    INCOMING_UNICAST_PACKETS
                        .with_label_values(&[&status.name])
                        .set(status.incoming_unicast_packets);
                    INCOMING_UNICAST_BYTES
                        .with_label_values(&[&status.name])
                        .set(status.incoming_unicast_bytes);
                    INCOMING_BROADCAST_PACKETS
                        .with_label_values(&[&status.name])
                        .set(status.incoming_broadcast_packets);
                    INCOMING_BROADCAST_BYTES
                        .with_label_values(&[&status.name])
                        .set(status.incoming_broadcast_bytes);
                }

                let git_revision = GIT_REVISION.unwrap_or("");
                let rust_version = RUST_VERSION.unwrap_or("");
                BUILD_INFO
                    .with_label_values(&[&VERSION, &git_revision, &rust_version])
                    .set(1.0);

                let metric_familys = prometheus::gather();
                let mut buffer = vec![];
                encoder.encode(&metric_familys, &mut buffer).unwrap();
                res.headers_mut()
                    .set(ContentType(encoder.format_type().parse::<Mime>().unwrap()));
                res.send(&buffer).unwrap();
            } else {
                res.headers_mut()
                    .set(ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![])));
                res.send(LANDING_PAGE.as_bytes()).unwrap();
            }
        })?;

        Ok(())
    }
}
