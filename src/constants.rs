//Mainnet
pub const PREFIX: [u8; 8] = [67, 78, 84, 82, 80, 82, 84, 89];
pub const ENCHANCED_SEND_ID: [u8; 1] = [2];
pub const CLASSIC_SEND_ID: [u8; 1] = [0];
pub const SWEEP: [u8; 1] = [4];
pub const DEX_ORDER: [u8; 1] = [10];
pub const BTC_PAY: [u8; 1] = [11];
pub const BASE58_VERSION_BYTE: [u8; 1] = [0];
pub const BASE58_P2SH_VERSION_BYTE: [u8; 1] = [5];
pub const BECH_32_VERSION_BYTE: [u8; 1] = [128];
pub const NETWORK: bitcoin::Network = bitcoin::Network::Bitcoin;
