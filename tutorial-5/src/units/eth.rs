use std::{
    fmt::Display,
    ops::{Add, Sub},
};

use serde::{Deserialize, Serialize};

use super::{GWeiNewType, WeiNewType};

// Here we define the type to track an amount of ETH.
// And also provided for series of functions to convert between ETH-GWei, ETH-Wei, GWei-Wei for scenarios
// that requier more previous calculation is needed.

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EthNewType(pub f64);

impl EthNewType {
    pub const GWEI_PER_ETH: i64 = 1_000_000_000;
    pub const WEI_PER_ETH: i128 = 1_000_000_000_000_000_000;
}

impl Add for EthNewType {
    type Output = Self;
    fn add(self, EthNewType(rhs): Self) -> Self::Output {
        let EthNewType(lhs) = self;
        let result = lhs + rhs;
        EthNewType(result)
    }
}

impl Sub for EthNewType {
    type Output = Self;

    fn sub(self, EthNewType(rhs): Self) -> Self::Output {
        let EthNewType(lhs) = self;
        let result = lhs - rhs;
        EthNewType(result)
    }
}

impl Display for EthNewType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

/// NOTE: this loses prevision
/// Converted Type from Gwei into Eth
impl From<GWeiNewType> for EthNewType {
    fn from(GWeiNewType(amount): GWeiNewType) -> Self {
        EthNewType(amount as f64 / EthNewType::GWEI_PER_ETH as f64)
    }
}

/// /// NOTE: this loses prevision
impl From<WeiNewType> for EthNewType {
    fn from(WeiNewType(amount): WeiNewType) -> Self {
        EthNewType(amount as f64 / EthNewType::WEI_PER_ETH as f64)
    }
}

#[cfg(test)]
mod tests {
    use std::ops::{Add, Sub};

    use crate::units::{
        GWeiNewType, WeiNewType, GWEI_PER_ETH_F64, WEI_PER_ETH,
    };

    use super::EthNewType;

    #[test]
    fn test_create_EthNewType() {
        // create new instance, we use 0 because the struct declaration not give exact struct field name
        let item = EthNewType { 0: 93484 as f64 };

        // serialize instance
        let serialized_json = serde_json::to_string(&item).unwrap();
        // print the serialized instance
        println!("eth type json {:?}", serialized_json);

        // deserialize instance
        let deserialized_json =
            serde_json::from_str::<EthNewType>(serialized_json.as_str())
                .unwrap();
        // print the deserialized instance
        println!("eth type instance content {:?}", deserialized_json);
    }

    #[test]
    fn test_add_two_EthNewType() {
        let item1 = EthNewType {
            0: 29484.937434 as f64,
        };
        let item2 = EthNewType {
            0: 2445.284634 as f64,
        };

        let EthNewType(sum_value) = item1.add(item2);
        assert_eq!((29484.937434 as f64 + 2445.284634 as f64), sum_value)
    }

    #[test]
    fn test_sub_two_EthNewType() {
        let item1 = EthNewType {
            0: 29484.937434 as f64,
        };
        let item2 = EthNewType {
            0: 2445.284634 as f64,
        };

        let EthNewType(sub_value) = item1.sub(item2);
        assert_eq!(sub_value, item1.0 as f64 - item2.0 as f64);

        let EthNewType(sub_value) = item2.sub(item1);
        assert_eq!(sub_value, item2.0 - item1.0);
    }

    #[test]
    fn test_convert_from_GWeiNewType() {
        let item1 = GWeiNewType {
            0: 8 * GWEI_PER_ETH_F64 as i64,
        };
        let item1_eth: EthNewType = item1.into();
        let EthNewType(eth_value) = item1_eth;
        assert_eq!(item1.0 as f64 / GWEI_PER_ETH_F64, eth_value);
        println!("eth_value: {:?}, item1.0 value: {:?}", item1_eth, item1);
    }

    #[test]
    fn test_convert_from_WeiNewType() {
        let item = WeiNewType {
            0: 100 * WEI_PER_ETH,
        };

        let EthNewType(amount) = item.into();
        assert_eq!(amount as i128, item.0 / WEI_PER_ETH);
    }
}
