use toml;
use hyper::header::ContentType;
use hyper::server::{Server, Request, Response};
use hyper::mime::Mime;
use prometheus;
use prometheus::{GaugeVec,Encoder,TextEncoder};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use softether_reader::SoftEtherReader;

lazy_static! {
    static ref ONLINE: GaugeVec = register_gauge_vec!(
        "softether_online",
        "Hub online.",
        &["hub"]
    ).unwrap();

    static ref SESSIONS: GaugeVec = register_gauge_vec!(
        "softether_sessions_total",
        "Total number of sessions.",
        &["hub"]
    ).unwrap();

    static ref SESSIONS_CLIENT: GaugeVec = register_gauge_vec!(
        "softether_sessions_client_total",
        "Total number of client sessions.",
        &["hub"]
    ).unwrap();

    static ref SESSIONS_BRIDGE: GaugeVec = register_gauge_vec!(
        "softether_sessions_bridge_total",
        "Total number of bridge sessions.",
        &["hub"]
    ).unwrap();

    static ref USERS: GaugeVec = register_gauge_vec!(
        "softether_users_total",
        "Total number of users.",
        &["hub"]
    ).unwrap();

    static ref GROUPS: GaugeVec = register_gauge_vec!(
        "softether_groups_total",
        "Total number of groups.",
        &["hub"]
    ).unwrap();

    static ref MAC_TABLES: GaugeVec = register_gauge_vec!(
        "softether_mac_tables_total",
        "Total number of entries in MAC table.",
        &["hub"]
    ).unwrap();

    static ref IP_TABLES: GaugeVec = register_gauge_vec!(
        "softether_ip_tables_total",
        "Total number of entries in IP table.",
        &["hub"]
    ).unwrap();

    static ref LOGINS: GaugeVec = register_gauge_vec!(
        "softether_logins_total",
        "Total number of logins.",
        &["hub"]
    ).unwrap();

    static ref OUTGOING_UNICAST_PACKETS: GaugeVec = register_gauge_vec!(
        "softether_outgoing_unicast_packets",
        "Outgoing unicast transfer in packets.",
        &["hub"]
    ).unwrap();

    static ref OUTGOING_UNICAST_BYTES: GaugeVec = register_gauge_vec!(
        "softether_outgoing_unicast_bytes",
        "Outgoing unicast transfer in bytes.",
        &["hub"]
    ).unwrap();

    static ref OUTGOING_BROADCAST_PACKETS: GaugeVec = register_gauge_vec!(
        "softether_outgoing_broadcast_packets",
        "Outgoing broadcast transfer in packets.",
        &["hub"]
    ).unwrap();

    static ref OUTGOING_BROADCAST_BYTES: GaugeVec = register_gauge_vec!(
        "softether_outgoing_broadcast_bytes",
        "Outgoing broadcast transfer in bytes.",
        &["hub"]
    ).unwrap();

    static ref INCOMING_UNICAST_PACKETS: GaugeVec = register_gauge_vec!(
        "softether_incoming_unicast_packets",
        "Incoming unicast transfer in packets.",
        &["hub"]
    ).unwrap();

    static ref INCOMING_UNICAST_BYTES: GaugeVec = register_gauge_vec!(
        "softether_incoming_unicast_bytes",
        "Incoming unicast transfer in bytes.",
        &["hub"]
    ).unwrap();

    static ref INCOMING_BROADCAST_PACKETS: GaugeVec = register_gauge_vec!(
        "softether_incoming_broadcast_packets",
        "Incoming broadcast transfer in packets.",
        &["hub"]
    ).unwrap();

    static ref INCOMING_BROADCAST_BYTES: GaugeVec = register_gauge_vec!(
        "softether_incoming_broadcast_bytes",
        "Incoming broadcast transfer in bytes.",
        &["hub"]
    ).unwrap();
}

#[derive(Debug, Deserialize)]
pub struct Config {
    listen_port: Option<u32>,
    vpncmd     : Option<String>,
    server     : Option<String>,
    hubs       : Vec<Hub>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Hub {
    name    : Option<String>,
    password: Option<String>,
}

impl Config {
    pub fn from_file( file: &str ) -> Result<Config, Box<Error>> {
        let mut f = File::open( file )?;
        let mut s = String::new();
        let _ = f.read_to_string( &mut s );
        let config: Config = toml::from_str( &s )?;
        Ok( config )
    }
}

pub struct Exporter {
}

impl Exporter {
    pub fn start ( config: Config ) -> Result<(), Box<Error>> {
        let encoder = TextEncoder::new();
        let addr    = format!( "0.0.0.0:{}", config.listen_port.unwrap_or( 9411 ) );
        let vpncmd  = config.vpncmd.unwrap_or( String::from( "vpncmd" ) );
        let server  = config.server.unwrap_or( String::from( "localhost" ) );
        let hubs    = config.hubs;

        println!( "Server started: {}", addr );

        Server::http( addr )?
            .handle( move |_: Request, mut res: Response| {

                for hub in hubs.clone() {
                    let name     = hub.name.unwrap_or( String::from( "" ) );
                    let password = hub.password.unwrap_or( String::from( "" ) );
                    let status   = match SoftEtherReader::hub_status( &vpncmd, &server, &name, &password ) {
                        Ok ( x ) => x,
                        Err( x ) => { println!( "Hub status read failed: {}", x ); return },
                    };

                    ONLINE                    .with_label_values(&[&status.name]).set( if status.online { 1.0 } else { 0.0 } );
                    SESSIONS                  .with_label_values(&[&status.name]).set( status.sessions );
                    SESSIONS_CLIENT           .with_label_values(&[&status.name]).set( status.sessions_client );
                    SESSIONS_BRIDGE           .with_label_values(&[&status.name]).set( status.sessions_bridge );
                    USERS                     .with_label_values(&[&status.name]).set( status.users );
                    GROUPS                    .with_label_values(&[&status.name]).set( status.groups );
                    MAC_TABLES                .with_label_values(&[&status.name]).set( status.mac_tables );
                    IP_TABLES                 .with_label_values(&[&status.name]).set( status.ip_tables );
                    LOGINS                    .with_label_values(&[&status.name]).set( status.logins );
                    OUTGOING_UNICAST_PACKETS  .with_label_values(&[&status.name]).set( status.outgoing_unicast_packets );
                    OUTGOING_UNICAST_BYTES    .with_label_values(&[&status.name]).set( status.outgoing_unicast_bytes );
                    OUTGOING_BROADCAST_PACKETS.with_label_values(&[&status.name]).set( status.outgoing_broadcast_packets );
                    OUTGOING_BROADCAST_BYTES  .with_label_values(&[&status.name]).set( status.outgoing_broadcast_bytes );
                    INCOMING_UNICAST_PACKETS  .with_label_values(&[&status.name]).set( status.incoming_unicast_packets );
                    INCOMING_UNICAST_BYTES    .with_label_values(&[&status.name]).set( status.incoming_unicast_bytes );
                    INCOMING_BROADCAST_PACKETS.with_label_values(&[&status.name]).set( status.incoming_broadcast_packets );
                    INCOMING_BROADCAST_BYTES  .with_label_values(&[&status.name]).set( status.incoming_broadcast_bytes );
                }

                let metric_familys = prometheus::gather();
                let mut buffer = vec![];
                encoder.encode( &metric_familys, &mut buffer ).unwrap();
                res.headers_mut().set( ContentType( encoder.format_type().parse::<Mime>().unwrap()));
                res.send( &buffer ).unwrap();
            })?;

        Ok( () )
    }
}

