use csv;
use std::error::Error;
use std::process::Command;

pub struct SoftEtherReader {
}

impl SoftEtherReader {
    pub fn hub_status( vpncmd: &str, server: &str, hub: &str, password: &str ) -> Result<HubStatus, Box<Error>> {
        let output = Command::new( vpncmd )
            .arg( server )
            .arg( "/SERVER" )
            .arg( format!( "/HUB:{}", hub ) )
            .arg( format!( "/PASSWORD:{}", password ) )
            .arg( "/CSV" )
            .arg( "/CMD" )
            .arg( "StatusGet" )
            .output()?;

        let mut rdr = csv::Reader::from_reader( output.stdout.as_slice() );
        let mut status = HubStatus::new();

        for entry in rdr.records() {
            let entry = entry?;
            let key = entry.get( 0 ).unwrap_or( "" );
            let val = entry.get( 1 ).unwrap_or( "" );
            match key.as_ref() {
                "仮想 HUB 名"                     => status.name                       = String::from( val ),
                "状態"                            => status.online                     = if val == "オンライン" { true } else { false },
                "SecureNAT 機能"                  => status.secure_nat                 = if val == "無効" { false } else { true },
                "セッション数"                    => status.sessions                   = val.parse()?,
                "セッション数 (クライアント)"     => status.sessions_client            = val.parse()?,
                "セッション数 (ブリッジ)"         => status.sessions_bridge            = val.parse()?,
                "アクセスリスト数"                => status.access_lists               = val.parse()?,
                "ユーザー数"                      => status.users                      = val.parse()?,
                "グループ数"                      => status.groups                     = val.parse()?,
                "MAC テーブル数"                  => status.mac_tables                 = val.parse()?,
                "IP テーブル数"                   => status.ip_tables                  = val.parse()?,
                "ログイン回数"                    => status.logins                     = val.parse()?,
                "送信ユニキャストパケット数"      => status.outgoing_unicast_packets   = SoftEtherReader::decode_packets( val )?,
                "送信ユニキャスト合計サイズ"      => status.outgoing_unicast_bytes     = SoftEtherReader::decode_bytes( val )?,
                "送信ブロードキャストパケット数"  => status.outgoing_broadcast_packets = SoftEtherReader::decode_packets( val )?,
                "送信ブロードキャスト合計サイズ"  => status.outgoing_broadcast_bytes   = SoftEtherReader::decode_bytes( val )?,
                "受信ユニキャストパケット数"      => status.incoming_unicast_packets   = SoftEtherReader::decode_packets( val )?,
                "受信ユニキャスト合計サイズ"      => status.incoming_unicast_bytes     = SoftEtherReader::decode_bytes( val )?,
                "受信ブロードキャストパケット数"  => status.incoming_broadcast_packets = SoftEtherReader::decode_packets( val )?,
                "受信ブロードキャスト合計サイズ"  => status.incoming_broadcast_bytes   = SoftEtherReader::decode_bytes( val )?,
                "Virtual Hub Name"                => status.name                       = String::from( val ),
                "Status"                          => status.online                     = if val == "Online" { true } else { false },
                "SecureNAT"                       => status.secure_nat                 = if val == "Disabled" { false } else { true },
                "Sessions"                        => status.sessions                   = val.parse()?,
                "Sessions (Client)"               => status.sessions_client            = val.parse()?,
                "Sessions (Bridge)"               => status.sessions_bridge            = val.parse()?,
                "Access Lists"                    => status.access_lists               = val.parse()?,
                "Users"                           => status.users                      = val.parse()?,
                "Groups"                          => status.groups                     = val.parse()?,
                "MAC Tables"                      => status.mac_tables                 = val.parse()?,
                "IP Tables"                       => status.ip_tables                  = val.parse()?,
                "Num Logins"                      => status.logins                     = val.parse()?,
                "Outgoing Unicast Packets"        => status.outgoing_unicast_packets   = SoftEtherReader::decode_packets( val )?,
                "Outgoing Unicast Total Size"     => status.outgoing_unicast_bytes     = SoftEtherReader::decode_bytes( val )?,
                "Outgoing Broadcast Packets"      => status.outgoing_broadcast_packets = SoftEtherReader::decode_packets( val )?,
                "Outgoing Broadcast Total Size"   => status.outgoing_broadcast_bytes   = SoftEtherReader::decode_bytes( val )?,
                "Incoming Unicast Packets"        => status.incoming_unicast_packets   = SoftEtherReader::decode_packets( val )?,
                "Incoming Unicast Total Size"     => status.incoming_unicast_bytes     = SoftEtherReader::decode_bytes( val )?,
                "Incoming Broadcast Packets"      => status.incoming_broadcast_packets = SoftEtherReader::decode_packets( val )?,
                "Incoming Broadcast Total Size"   => status.incoming_broadcast_bytes   = SoftEtherReader::decode_bytes( val )?,
                "虚拟 HUB 名称"                   => status.name                       = String::from( val ),
                "状态"                            => status.online                     = if val == "在线" { true } else { false },
                "SecureNAT 机能"                  => status.secure_nat                 = if val == "无效" { false } else { true },
                "会话数"                          => status.sessions                   = val.parse()?,
                "会话数 (客户端)"                 => status.sessions_client            = val.parse()?,
                "会话数 (网桥)"                   => status.sessions_bridge            = val.parse()?,
                "访问列表"                        => status.access_lists               = val.parse()?,
                "用户数"                          => status.users                      = val.parse()?,
                "组数"                            => status.groups                     = val.parse()?,
                "MAC 表数"                        => status.mac_tables                 = val.parse()?,
                "IP 表数"                         => status.ip_tables                  = val.parse()?,
                "登录次数"                        => status.logins                     = val.parse()?,
                "发送单播数据包"                  => status.outgoing_unicast_packets   = SoftEtherReader::decode_packets( val )?,
                "发送单播总量"                    => status.outgoing_unicast_bytes     = SoftEtherReader::decode_bytes( val )?,
                "发送广播数据包"                  => status.outgoing_broadcast_packets = SoftEtherReader::decode_packets( val )?,
                "发送广播总量"                    => status.outgoing_broadcast_bytes   = SoftEtherReader::decode_bytes( val )?,
                "接收单播数据包"                  => status.incoming_unicast_packets   = SoftEtherReader::decode_packets( val )?,
                "接收单播总量"                    => status.incoming_unicast_bytes     = SoftEtherReader::decode_bytes( val )?,
                "接收广播数据包"                  => status.incoming_broadcast_packets = SoftEtherReader::decode_packets( val )?,
                "接收广播总量"                    => status.incoming_broadcast_bytes   = SoftEtherReader::decode_bytes( val )?,
                _                                 => (),
            }
        }
        Ok( status )
    }

    fn decode_packets( src: &str ) -> Result<f64, Box<Error>> {
        let ret = String::from( src ).replace( ",", "" )
                                     .replace( " パケット", "" )
                                     .replace( " packets", "" )
                                     .replace( " 数据包", "" ).parse()?;
        Ok( ret )
    }

    fn decode_bytes( src: &str ) -> Result<f64, Box<Error>> {
        let ret = String::from( src ).replace( ",", "" )
                                     .replace( " バイト", "" )
                                     .replace( " bytes", "" )
                                     .replace( " 字节", "" ).parse()?;
        Ok( ret )
    }
}

#[derive(Debug)]
pub struct HubStatus {
    pub name                      : String,
    pub online                    : bool  ,
    pub secure_nat                : bool  ,
    pub sessions                  : f64   ,
    pub sessions_client           : f64   ,
    pub sessions_bridge           : f64   ,
    pub access_lists              : f64   ,
    pub users                     : f64   ,
    pub groups                    : f64   ,
    pub mac_tables                : f64   ,
    pub ip_tables                 : f64   ,
    pub logins                    : f64   ,
    pub outgoing_unicast_packets  : f64   ,
    pub outgoing_unicast_bytes    : f64   ,
    pub outgoing_broadcast_packets: f64   ,
    pub outgoing_broadcast_bytes  : f64   ,
    pub incoming_unicast_packets  : f64   ,
    pub incoming_unicast_bytes    : f64   ,
    pub incoming_broadcast_packets: f64   ,
    pub incoming_broadcast_bytes  : f64   ,
}

impl HubStatus {
    pub fn new() -> HubStatus {
        HubStatus {
            name                      : String::from( "" ),
            online                    : false ,
            secure_nat                : false ,
            sessions                  : 0.0   ,
            sessions_client           : 0.0   ,
            sessions_bridge           : 0.0   ,
            access_lists              : 0.0   ,
            users                     : 0.0   ,
            groups                    : 0.0   ,
            mac_tables                : 0.0   ,
            ip_tables                 : 0.0   ,
            logins                    : 0.0   ,
            outgoing_unicast_packets  : 0.0   ,
            outgoing_unicast_bytes    : 0.0   ,
            outgoing_broadcast_packets: 0.0   ,
            outgoing_broadcast_bytes  : 0.0   ,
            incoming_unicast_packets  : 0.0   ,
            incoming_unicast_bytes    : 0.0   ,
            incoming_broadcast_packets: 0.0   ,
            incoming_broadcast_bytes  : 0.0   ,
        }
    }
}


