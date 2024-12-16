use std::collections::HashSet;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use katana_rpc::cors::HeaderValue;

/// The default maximum number of concurrent RPC connections.
pub const DEFAULT_RPC_MAX_CONNECTIONS: u32 = 100;
pub const DEFAULT_RPC_ADDR: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);
pub const DEFAULT_RPC_PORT: u16 = 5050;

/// Default maximmum page size for the `starknet_getEvents` RPC method.
pub const DEFAULT_RPC_MAX_EVENT_PAGE_SIZE: u64 = 1024;
/// Default maximmum number of keys for the `starknet_getStorageProof` RPC method.
pub const DEFAULT_RPC_MAX_PROOF_KEYS: u64 = 100;

/// List of APIs supported by Katana.
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, strum_macros::EnumString, strum_macros::Display,
)]
pub enum ApiKind {
    Starknet,
    Torii,
    Dev,
    Saya,
}

/// Configuration for the RPC server.
#[derive(Debug, Clone)]
pub struct RpcConfig {
    pub addr: IpAddr,
    pub port: u16,
    pub max_connections: u32,
    pub apis: HashSet<ApiKind>,
    pub cors_origins: Vec<HeaderValue>,
    pub max_event_page_size: Option<u64>,
    pub max_proof_keys: Option<u64>,
}

impl RpcConfig {
    /// Returns the [`SocketAddr`] for the RPC server.
    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.addr, self.port)
    }
}

impl Default for RpcConfig {
    fn default() -> Self {
        Self {
            cors_origins: Vec::new(),
            addr: DEFAULT_RPC_ADDR,
            port: DEFAULT_RPC_PORT,
            max_connections: DEFAULT_RPC_MAX_CONNECTIONS,
            apis: HashSet::from([ApiKind::Starknet]),
            max_event_page_size: Some(DEFAULT_RPC_MAX_EVENT_PAGE_SIZE),
            max_proof_keys: Some(DEFAULT_RPC_MAX_PROOF_KEYS),
        }
    }
}
