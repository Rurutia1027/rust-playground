use std::{
    fmt::Display,
    ops::{Add, Sub},
};

use super::{EthNewType, GWeiNewType, WeiNewType};
use serde::Serialize;

/// This is abstraction of USD.
/// We use the imprecise f64 here because most USD amounts we track are based on ETH amounts,
/// converted to USD, which is also imprecise.
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
#[serde(transparent)]
pub struct UsdNewtype(pub f64);

impl UsdNewtype {
    /// #[allow(dead_code)] purpose:
    /// Rust has strict rules about unused code. If you define a function, struct, or variable that is not
    /// used anywhere, the compiler issues a warning.
    /// Adding #[allow(dead_code)] to a function, struct, or module supresses this warning.
    #[allow(dead_code)]
    pub fn from_eth(eth: EthNewType, eth_price: f64) -> Self {
        let usd = eth.0 * eth_price;
        UsdNewtype(usd)
    }

    // #[allow(dead_code)]
    // pub fn from_gwei(gwei: GWeiNewType, eth_price: f64) -> Self {
    //     let eth: EthNewType = gwei.into();
    //     Self::from(eth)
    // }
}
