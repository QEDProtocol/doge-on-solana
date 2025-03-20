use doge_light_client::chain_state::QEDDogeChainStateCore;

pub const QDOGE_BRIDGE_REQUIRED_CONFIRMATIONS: usize = 4;
pub const QDOGE_BRIDGE_BLOCK_HASH_CACHE_SIZE: usize = 32;
pub const QDOGE_BRIDGE_BLOCK_TREE_HEIGHT: usize = 32;

pub type QEDDogeChainState = QEDDogeChainStateCore<
    QDOGE_BRIDGE_BLOCK_HASH_CACHE_SIZE,
    QDOGE_BRIDGE_REQUIRED_CONFIRMATIONS,
    QDOGE_BRIDGE_BLOCK_TREE_HEIGHT,
>;
