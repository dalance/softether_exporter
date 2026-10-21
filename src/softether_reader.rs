use csv;
use std::error::Error;
use std::fmt;
use std::io::Write;
use std::process::{Command, Stdio};

#[derive(Debug)]
pub struct SoftEtherError {
    msg: String,
}

impl fmt::Display for SoftEtherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for SoftEtherError {
    fn description(&self) -> &str {
        &self.msg
    }
}

pub struct SoftEtherReader;

impl SoftEtherReader {
    pub fn hub_status(
        vpncmd: &str,
        server: &str,
        hub: &str,
        password: &str,
    ) -> Result<HubStatus, Box<dyn Error>> {
        let mut child = Command::new(vpncmd)
            .arg(server)
            .arg("/SERVER")
            .arg(format!("/ADMINHUB:{}", hub))
            .arg(format!("/PASSWORD:{}", password))
            .arg("/CSV")
            .arg("/CMD")
            .arg("StatusGet")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        {
            let stdin = child.stdin.as_mut().unwrap();
            // Input Ctrl-D to interrupt password prompt
            stdin.write_all(&[4])?;
        }

        let output = child.wait_with_output()?;

        if !output.status.success() {
            let msg = String::from_utf8_lossy(output.stdout.as_slice());
            return Err(Box::new(SoftEtherError {
                msg: String::from(format!("vpncmd failed ( {} )", msg)),
            }));
        }

        SoftEtherReader::decode_hub_status(&output.stdout)
    }

    pub fn hub_sessions(
        vpncmd: &str,
        server: &str,
        hub: &str,
        password: &str,
    ) -> Result<Vec<HubSession>, Box<dyn Error>> {
        let mut child = Command::new(vpncmd)
            .arg(server)
            .arg("/SERVER")
            .arg(format!("/ADMINHUB:{}", hub))
            .arg(format!("/PASSWORD:{}", password))
            .arg("/CSV")
            .arg("/CMD")
            .arg("SessionList")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        {
            let stdin = child.stdin.as_mut().unwrap();
            // Input Ctrl-D to interrupt password prompt
            stdin.write_all(&[4])?;
        }

        let output = child.wait_with_output()?;

        if !output.status.success() {
            let msg = String::from_utf8_lossy(output.stdout.as_slice());
            return Err(Box::new(SoftEtherError {
                msg: String::from(format!("vpncmd failed ( {} )", msg)),
            }));
        }

        SoftEtherReader::decode_hub_sessions(&output.stdout)
    }

    fn decode_hub_status(src: &[u8]) -> Result<HubStatus, Box<dyn Error>> {
        let mut rdr = csv::Reader::from_reader(src);
        let mut status = HubStatus::new();

        for entry in rdr.records() {
            let entry = entry?;
            let key = entry.get(0).unwrap_or("");
            let val = entry.get(1).unwrap_or("");
            match key.as_ref() {
                "仮想 HUB 名" => status.name = String::from(val),
                "状態" => {
                    status.online = if val == "オンライン" {
                        true
                    } else {
                        false
                    }
                }
                "SecureNAT 機能" => {
                    status.secure_nat = if val == "無効" { false } else { true }
                }
                "セッション数" => status.sessions = val.parse()?,
                "セッション数 (クライアント)" => {
                    status.sessions_client = val.parse()?
                }
                "セッション数 (ブリッジ)" => status.sessions_bridge = val.parse()?,
                "アクセスリスト数" => status.access_lists = val.parse()?,
                "ユーザー数" => status.users = val.parse()?,
                "グループ数" => status.groups = val.parse()?,
                "MAC テーブル数" => status.mac_tables = val.parse()?,
                "IP テーブル数" => status.ip_tables = val.parse()?,
                "ログイン回数" => status.logins = val.parse()?,
                "送信ユニキャストパケット数" => {
                    status.outgoing_unicast_packets = SoftEtherReader::decode_packets(val)?
                }
                "送信ユニキャスト合計サイズ" => {
                    status.outgoing_unicast_bytes = SoftEtherReader::decode_bytes(val)?
                }
                "送信ブロードキャストパケット数" => {
                    status.outgoing_broadcast_packets = SoftEtherReader::decode_packets(val)?
                }
                "送信ブロードキャスト合計サイズ" => {
                    status.outgoing_broadcast_bytes = SoftEtherReader::decode_bytes(val)?
                }
                "受信ユニキャストパケット数" => {
                    status.incoming_unicast_packets = SoftEtherReader::decode_packets(val)?
                }
                "受信ユニキャスト合計サイズ" => {
                    status.incoming_unicast_bytes = SoftEtherReader::decode_bytes(val)?
                }
                "受信ブロードキャストパケット数" => {
                    status.incoming_broadcast_packets = SoftEtherReader::decode_packets(val)?
                }
                "受信ブロードキャスト合計サイズ" => {
                    status.incoming_broadcast_bytes = SoftEtherReader::decode_bytes(val)?
                }
                "Virtual Hub Name" => status.name = String::from(val),
                "Status" => status.online = if val == "Online" { true } else { false },
                "SecureNAT" => status.secure_nat = if val == "Disabled" { false } else { true },
                "Sessions" => status.sessions = val.parse()?,
                "Sessions (Client)" => status.sessions_client = val.parse()?,
                "Sessions (Bridge)" => status.sessions_bridge = val.parse()?,
                "Access Lists" => status.access_lists = val.parse()?,
                "Users" => status.users = val.parse()?,
                "Groups" => status.groups = val.parse()?,
                "MAC Tables" => status.mac_tables = val.parse()?,
                "IP Tables" => status.ip_tables = val.parse()?,
                "Num Logins" => status.logins = val.parse()?,
                "Outgoing Unicast Packets" => {
                    status.outgoing_unicast_packets = SoftEtherReader::decode_packets(val)?
                }
                "Outgoing Unicast Total Size" => {
                    status.outgoing_unicast_bytes = SoftEtherReader::decode_bytes(val)?
                }
                "Outgoing Broadcast Packets" => {
                    status.outgoing_broadcast_packets = SoftEtherReader::decode_packets(val)?
                }
                "Outgoing Broadcast Total Size" => {
                    status.outgoing_broadcast_bytes = SoftEtherReader::decode_bytes(val)?
                }
                "Incoming Unicast Packets" => {
                    status.incoming_unicast_packets = SoftEtherReader::decode_packets(val)?
                }
                "Incoming Unicast Total Size" => {
                    status.incoming_unicast_bytes = SoftEtherReader::decode_bytes(val)?
                }
                "Incoming Broadcast Packets" => {
                    status.incoming_broadcast_packets = SoftEtherReader::decode_packets(val)?
                }
                "Incoming Broadcast Total Size" => {
                    status.incoming_broadcast_bytes = SoftEtherReader::decode_bytes(val)?
                }
                "虚拟 HUB 名称" => status.name = String::from(val),
                "状态" => status.online = if val == "在线" { true } else { false },
                "SecureNAT 机能" => {
                    status.secure_nat = if val == "无效" { false } else { true }
                }
                "会话数" => status.sessions = val.parse()?,
                "会话数 (客户端)" => status.sessions_client = val.parse()?,
                "会话数 (网桥)" => status.sessions_bridge = val.parse()?,
                "访问列表" => status.access_lists = val.parse()?,
                "用户数" => status.users = val.parse()?,
                "组数" => status.groups = val.parse()?,
                "MAC 表数" => status.mac_tables = val.parse()?,
                "IP 表数" => status.ip_tables = val.parse()?,
                "登录次数" => status.logins = val.parse()?,
                "发送单播数据包" => {
                    status.outgoing_unicast_packets = SoftEtherReader::decode_packets(val)?
                }
                "发送单播总量" => {
                    status.outgoing_unicast_bytes = SoftEtherReader::decode_bytes(val)?
                }
                "发送广播数据包" => {
                    status.outgoing_broadcast_packets = SoftEtherReader::decode_packets(val)?
                }
                "发送广播总量" => {
                    status.outgoing_broadcast_bytes = SoftEtherReader::decode_bytes(val)?
                }
                "接收单播数据包" => {
                    status.incoming_unicast_packets = SoftEtherReader::decode_packets(val)?
                }
                "接收单播总量" => {
                    status.incoming_unicast_bytes = SoftEtherReader::decode_bytes(val)?
                }
                "接收广播数据包" => {
                    status.incoming_broadcast_packets = SoftEtherReader::decode_packets(val)?
                }
                "接收广播总量" => {
                    status.incoming_broadcast_bytes = SoftEtherReader::decode_bytes(val)?
                }
                _ => (),
            }
        }
        Ok(status)
    }

    fn decode_hub_sessions(src: &[u8]) -> Result<Vec<HubSession>, Box<dyn Error>> {
        let mut rdr = csv::Reader::from_reader(src);
        let mut sessions = Vec::new();

        for entry in rdr.records() {
            let entry = entry?;
            let name = entry.get(0).unwrap_or("");
            let vlan_id = entry.get(1).unwrap_or("");
            let location = entry.get(2).unwrap_or("");
            let user = entry.get(3).unwrap_or("");
            let source = entry.get(4).unwrap_or("");
            let connections = entry.get(5).unwrap_or("");
            let transfer_bytes = entry.get(6).unwrap_or("");
            let transfer_packets = entry.get(7).unwrap_or("");

            let connections = SoftEtherReader::decode_connections(connections)?;
            let transfer_bytes = SoftEtherReader::decode_bytes(transfer_bytes)?;
            let transfer_packets = SoftEtherReader::decode_bytes(transfer_packets)?;

            let session = HubSession {
                name: String::from(name),
                vlan_id: String::from(vlan_id),
                location: String::from(location),
                user: String::from(user),
                source: String::from(source),
                connections,
                transfer_bytes,
                transfer_packets,
            };

            sessions.push(session);
        }

        Ok(sessions)
    }

    fn decode_packets(src: &str) -> Result<f64, Box<dyn Error>> {
        let ret = String::from(src)
            .replace(",", "")
            .replace(" パケット", "")
            .replace(" packets", "")
            .replace(" 数据包", "")
            .parse()?;
        Ok(ret)
    }

    fn decode_bytes(src: &str) -> Result<f64, Box<dyn Error>> {
        let ret = String::from(src)
            .replace(",", "")
            .replace(" バイト", "")
            .replace(" bytes", "")
            .replace(" 字节", "")
            .parse()?;
        Ok(ret)
    }

    fn decode_connections(src: &str) -> Result<(f64, f64), Box<dyn Error>> {
        if !src.contains('/') {
            Ok((0.0, 0.0))
        } else {
            let src: Vec<_> = src.split('/').collect();
            let ret0: f64 = src[0].trim().parse()?;
            let ret1: f64 = src[1].trim().parse()?;
            Ok((ret0, ret1))
        }
    }
}

#[derive(Debug)]
pub struct HubStatus {
    pub name: String,
    pub online: bool,
    pub secure_nat: bool,
    pub sessions: f64,
    pub sessions_client: f64,
    pub sessions_bridge: f64,
    pub access_lists: f64,
    pub users: f64,
    pub groups: f64,
    pub mac_tables: f64,
    pub ip_tables: f64,
    pub logins: f64,
    pub outgoing_unicast_packets: f64,
    pub outgoing_unicast_bytes: f64,
    pub outgoing_broadcast_packets: f64,
    pub outgoing_broadcast_bytes: f64,
    pub incoming_unicast_packets: f64,
    pub incoming_unicast_bytes: f64,
    pub incoming_broadcast_packets: f64,
    pub incoming_broadcast_bytes: f64,
}

impl HubStatus {
    pub fn new() -> HubStatus {
        HubStatus {
            name: String::from(""),
            online: false,
            secure_nat: false,
            sessions: 0.0,
            sessions_client: 0.0,
            sessions_bridge: 0.0,
            access_lists: 0.0,
            users: 0.0,
            groups: 0.0,
            mac_tables: 0.0,
            ip_tables: 0.0,
            logins: 0.0,
            outgoing_unicast_packets: 0.0,
            outgoing_unicast_bytes: 0.0,
            outgoing_broadcast_packets: 0.0,
            outgoing_broadcast_bytes: 0.0,
            incoming_unicast_packets: 0.0,
            incoming_unicast_bytes: 0.0,
            incoming_broadcast_packets: 0.0,
            incoming_broadcast_bytes: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct HubSession {
    pub name: String,
    pub vlan_id: String,
    pub location: String,
    pub user: String,
    pub source: String,
    pub connections: (f64, f64),
    pub transfer_bytes: f64,
    pub transfer_packets: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hub_status() {
        let src = r#"項目,値
仮想 HUB 名,DEFAULT
状態,オンライン
種類,スタンドアロン
SecureNAT 機能,無効
セッション数,4
セッション数 (クライアント),3
セッション数 (ブリッジ),0
アクセスリスト数,0
ユーザー数,1
グループ数,0
MAC テーブル数,134
IP テーブル数,211
ログイン回数,18965
最終ログイン日時,2020-04-08 09:25:49
最終通信日時,2020-04-08 11:31:43
作成日時,2018-01-16 10:04:05
送信ユニキャストパケット数,"7,262,679,895 パケット"
送信ユニキャスト合計サイズ,"4,153,388,417,848 バイト"
送信ブロードキャストパケット数,"1,756,889,863 パケット"
送信ブロードキャスト合計サイズ,"256,781,466,202 バイト"
受信ユニキャストパケット数,"8,840,585,104 パケット"
受信ユニキャスト合計サイズ,"4,676,951,155,757 バイト"
受信ブロードキャストパケット数,"976,264,699 パケット"
受信ブロードキャスト合計サイズ,"138,170,046,309 バイト""#;

        let status = SoftEtherReader::decode_hub_status(src.as_bytes()).unwrap();
        assert_eq!(status.name, String::from("DEFAULT"));
        assert_eq!(status.online, true);
        assert_eq!(status.secure_nat, false);
        assert_eq!(status.sessions, 4.0);
        assert_eq!(status.sessions_client, 3.0);
        assert_eq!(status.sessions_bridge, 0.0);
        assert_eq!(status.access_lists, 0.0);
        assert_eq!(status.users, 1.0);
        assert_eq!(status.groups, 0.0);
        assert_eq!(status.mac_tables, 134.0);
        assert_eq!(status.ip_tables, 211.0);
        assert_eq!(status.logins, 18965.0);
        assert_eq!(status.outgoing_unicast_packets, 7262679895.0);
        assert_eq!(status.outgoing_unicast_bytes, 4153388417848.0);
        assert_eq!(status.outgoing_broadcast_packets, 1756889863.0);
        assert_eq!(status.outgoing_broadcast_bytes, 256781466202.0);
        assert_eq!(status.incoming_unicast_packets, 8840585104.0);
        assert_eq!(status.incoming_unicast_bytes, 4676951155757.0);
        assert_eq!(status.incoming_broadcast_packets, 976264699.0);
        assert_eq!(status.incoming_broadcast_bytes, 138170046309.0);
    }

    #[test]
    fn test_hub_session() {
        let src = r#"セッション名,VLAN ID,場所,ユーザー名,接続元ホスト名,TCP コネクション,転送バイト数,転送パケット数
SID-LOCALBRIDGE-1,－,ローカルセッション,Local Bridge,Ethernet ブリッジ,なし,"294,035,917,956","1,380,393,323"
SID-XXXX-1047,－,ローカルセッション,xxxx,xxx.example.com,2 / 2,"82,691,861","322,784""#;

        let sessions = SoftEtherReader::decode_hub_sessions(src.as_bytes()).unwrap();
        assert_eq!(sessions[0].name, String::from("SID-LOCALBRIDGE-1"));
        assert_eq!(sessions[0].vlan_id, String::from("－"));
        assert_eq!(sessions[0].location, String::from("ローカルセッション"));
        assert_eq!(sessions[0].user, String::from("Local Bridge"));
        assert_eq!(sessions[0].source, String::from("Ethernet ブリッジ"));
        assert_eq!(sessions[0].connections, (0.0, 0.0));
        assert_eq!(sessions[0].transfer_bytes, 294035917956.0);
        assert_eq!(sessions[0].transfer_packets, 1380393323.0);
        assert_eq!(sessions[1].name, String::from("SID-XXXX-1047"));
        assert_eq!(sessions[1].vlan_id, String::from("－"));
        assert_eq!(sessions[1].location, String::from("ローカルセッション"));
        assert_eq!(sessions[1].user, String::from("xxxx"));
        assert_eq!(sessions[1].source, String::from("xxx.example.com"));
        assert_eq!(sessions[1].connections, (2.0, 2.0));
        assert_eq!(sessions[1].transfer_bytes, 82691861.0);
        assert_eq!(sessions[1].transfer_packets, 322784.0);
    }
}
