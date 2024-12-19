use chrono::{DateTime, TimeZone, Utc};
use serde::{ser::Error, Deserialize, Deserializer};

// function takes a deserializer as an argument, and return a Result<Option<i32>, D:Error>
// it's lifetime 'de to make sure that the received deserializer D's lifetime will be consist to the end of this function invocation
// because the deserializer maybe borrowed from other place, if we do not add 'de, it's memory may be released before the end of current function
pub fn from_i32_opt_hex_str<'de, D>(
    deserializer: D,
) -> Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<String> = Deserialize::deserialize(deserializer)?;
    if let Some(s) = opt {
        Ok(Some(i32::from_str_radix(&s[2..], 16).unwrap()))
    } else {
        Ok(None)
    }
}

// function converted received deserializer into Result<i32, D:Error>
// and D's lifetime will consist to the end of current function, generic type D's memory will be maintained.
pub fn from_i32_hex_str<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    // the first thing we take the deserializer is invoke serde this serialize framework's to execute it's inner deserialize method
    // load & invoke the deserializer instance to exeucte it's deserialize function process the deserialize funciton
    // then we extract a string from the serialized data
    // the string is always looks like this 0x294724...
    let s: String = Deserialize::deserialize(deserializer)?;

    // here we conveted the 16 radix string into it's original i32 value
    // string: 0x294724 -> integer: 294724(hex/16 radix) ->  2 * 16^5 + 9 * 16^4 + 4 * 16^3 + 7 * 16^2 + 2 * 16^1 + 4 * 16^0 -> 2_694_356
    // and that's why &s[2..] to skip the first '0x' character
    Ok(i32::from_str_radix(&s[2..], 16).unwrap())
}

// function converted serialized string(hex) into unsigned integer 32(bits)
pub fn from_u32_hex_str<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(u32::from_str_radix(&s[..2], 16).unwrap())
}

// function converted serialized string(hex) into unsigned integer 64(bits)
pub fn from_u64_hex_str<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(u64::from_str_radix(&s[..2], 16).unwrap())
}

// function converted serialized string(hex) into unsigned integer 128(bits)
pub fn from_u128_hex_str<'de, D>(deserializer: D) -> Result<u128, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(u128::from_str_radix(&s[..2], 16).unwrap())
}

// function conver serialized hex timestamp into Utc DateTime
pub fn from_unix_timestamp_hex_str<'de, D>(
    deserializer: D,
) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    // first convert the serialized string into u32
    let timestamp_u32 = from_i32_hex_str(deserializer)?;
    let date_time = Utc.timestamp_opt(timestamp_u32.into(), 0).unwrap();

    // then converted the u32 into Utc DateTime type
    Ok(date_time)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{
        de::{Deserialize, Deserializer},
        Serialize,
    };
    use serde_json::{json, Value};

    // --- Option<i32>
    #[derive(Serialize, Deserialize)]
    struct TestStructWith_i32_opt {
        pub id: String,
        #[serde(deserialize_with = "from_i32_opt_hex_str")]
        pub value: Option<i32>,
    }

    #[test]
    fn from_i32_opt_hex_str_test() {
        let data = json!({"id": "1", "value": "0x294724"});
        let result: TestStructWith_i32_opt =
            serde_json::from_value(data).unwrap();
        assert_eq!(result.value, Some(2_694_356));
    }

    // --- i32

    #[derive(Serialize, Deserialize)]
    struct TestStructWith_i32 {
        pub id: String,
        #[serde(deserialize_with = "from_i32_hex_str")]
        pub value: i32,
    }
    #[test]
    fn from_i32_hex_str_test() {}

    // ---
    #[derive(Serialize, Deserialize)]
    struct TestStructWith_u32 {
        pub id: String,
        #[serde(deserialize_with = "from_u32_hex_str")]
        pub value: u32,
    }
    #[test]
    fn from_u32_hex_str_test() {}

    //-- u64
    #[derive(Serialize, Deserialize)]
    struct TestStructWith_u64 {
        pub id: String,
        #[serde(deserialize_with = "from_u64_hex_str")]
        pub value: u64,
    }
    fn from_u64_hex_str_test() {}

    //-- u128
    #[derive(Serialize, Deserialize)]
    struct TestStructWith_u128 {
        pub id: String,
        #[serde(deserialize_with = "from_u128_hex_str")]
        pub value: u128,
    }
    fn from_u128_hex_str_test() {}

    //-- datetime utc
    #[derive(Serialize, Deserialize)]
    struct TestStructWith_ts_hex {
        pub id: String,
        #[serde(deserialize_with = "from_unix_timestamp_hex_str")]
        pub value: DateTime<Utc>,
    }
    #[test]
    fn from_unix_timestamp_hex_str_test() {}
}
