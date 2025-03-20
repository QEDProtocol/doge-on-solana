use doge_light_client::{chain_state::QEDDogeChainStateCore, constants as doge_net_constants, init_params::InitBlockDataIBC};



// start parameters
pub const QDOGE_BRIDGE_REQUIRED_CONFIRMATIONS: usize = 4;
pub const QDOGE_BRIDGE_BLOCK_HASH_CACHE_SIZE: usize = 32;
pub const QDOGE_BRIDGE_BLOCK_TREE_HEIGHT: usize = 32;
// end parameters

// start impls

const Q_BLOCK_DATA_TRACKER_LEN: usize = (4 + 2) + (104)*QDOGE_BRIDGE_BLOCK_HASH_CACHE_SIZE;
const Q_BLOCK_TREE_TRACKER_LEN: usize = 8 + (32*3)*QDOGE_BRIDGE_BLOCK_TREE_HEIGHT;
pub const Q_IBC_INNER_STATE_LEN: usize = Q_BLOCK_DATA_TRACKER_LEN + Q_BLOCK_TREE_TRACKER_LEN;
// end impls


pub type QInitBlockDataIBC = InitBlockDataIBC<QDOGE_BRIDGE_BLOCK_HASH_CACHE_SIZE, QDOGE_BRIDGE_BLOCK_TREE_HEIGHT>; 

pub type QEDDogeChainState = QEDDogeChainStateCore<
    QDOGE_BRIDGE_BLOCK_HASH_CACHE_SIZE,
    QDOGE_BRIDGE_REQUIRED_CONFIRMATIONS,
    QDOGE_BRIDGE_BLOCK_TREE_HEIGHT,
>;


#[cfg(all(feature = "doge_regtest", not(feature = "doge_mainnet"), not(feature = "doge_testnet")))]
pub type QDogeNetworkConfig = doge_net_constants::DogeRegTestConfig;

#[cfg(all(feature = "doge_testnet", not(feature = "doge_mainnet"), not(feature = "doge_regtest")))]
pub type QDogeNetworkConfig = doge_net_constants::DogeTestNetConfig;

#[cfg(all(feature = "doge_mainnet", not(feature = "doge_testnet"), not(feature = "doge_regtest")))]
pub type QDogeNetworkConfig = doge_net_constants::DogeMainNetConfig;