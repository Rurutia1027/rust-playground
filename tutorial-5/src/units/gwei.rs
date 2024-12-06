use std::{
    fmt,
    num::ParseIntError,
    ops::{Add, Div, Sub},
    result,
    str::FromStr,
};

use serde::{de, de::Visitor, Deserialize, Serialize};

use super::{EthNewType, WeiNewType};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GWeiNewType(pub i64);

impl fmt::Display for GWeiNewType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl GWeiNewType {
    pub const WEI_PER_GWEI: u32 = 1_000_000_000;
}

impl Add<GWeiNewType> for GWeiNewType {
    type Output = Self;
    fn add(self, GWeiNewType(rhs): Self) -> Self::Output {
        let GWeiNewType(lhs) = self;
        let result = lhs
            .checked_add(rhs)
            .expect("caused overflow in gwei addition");
        GWeiNewType(result)
    }
}

impl Sub<GWeiNewType> for GWeiNewType {
    type Output = Self;
    fn sub(self, GWeiNewType(rhs): Self) -> Self::Output {
        let GWeiNewType(lhs) = self;
        let result = lhs
            .checked_sub(rhs)
            .expect("caused underflow in gwei substraction");
        GWeiNewType(result)
    }
}

impl Div<GWeiNewType> for GWeiNewType {
    type Output = Self;
    fn div(self, GWeiNewType(lhs): Self) -> Self::Output {
        let GWeiNewType(rhs) = self;
        GWeiNewType(lhs / rhs)
    }
}
