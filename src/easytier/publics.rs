use crate::controller::Room;

pub type PublicServers = Vec<String>;

pub fn fetch_public_nodes(_: &Room, mut external_nodes: PublicServers) -> PublicServers {
    external_nodes.extend_from_slice(&[
    "tcp://cgk1.clusters.zeabur.com:22171",
        "tcp://public2.easytier.cn:54321",
        "tcp://tcp.ap-northeast-1.clawcloudrun.com:45146",
        "https://etnode.zkitefly.eu.org/node2",
    ].map(|s| s.into()));

    external_nodes
}
