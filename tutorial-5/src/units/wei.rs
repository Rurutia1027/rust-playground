//! Ethereum wei unit type and associated fns
//! A 1e-18th of an ether(ETH).

use std::{
    fmt::Display,
    num::ParseFloatError,
    ops::{Add, Sub},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use super::{EthNewType, GWeiNewType, WEI_PER_ETH};

pub type WeiF64 = f64;

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct WeiNewType(pub i128);

impl WeiNewType {
    pub fn from_eth(eth: i128) -> Self {
        Self(eth * WEI_PER_ETH)
    }
}
