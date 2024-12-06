mod eth;
mod gwei;
mod usd;
mod wei;

pub use eth::EthNewType;
pub use gwei::GWeiNewType;
pub use wei::WeiNewType;

// 1 ETH = 10 ^9 Gwei
pub const GWEI_PER_ETH_F64: f64 = 1_000_000_000_f64;

// 1 ETH = 10 ^ 18 Wei
pub const WEI_PER_ETH: i128 = 1_000_000_000_000_000_000;
