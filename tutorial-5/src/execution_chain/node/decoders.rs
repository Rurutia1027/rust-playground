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
    i32::from_str_radix(&s[2..], 16).map_err(|e| {
        serde::de::Error::custom(format!("Failed to parse hex string: {}", e))
    })
}

// function converted serialized string(hex) into unsigned integer 32(bits)
pub fn from_u32_hex_str<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;

    if !s.starts_with("0x") {
        return Err(serde::de::Error::custom(format!(
            "Expect hex string {} starts with '0x' prefix",
            s
        )));
    }

    u32::from_str_radix(&s[2..], 16).map_err(|e| {
        serde::de::Error::custom(format!("Failed to parse hex string: {}", e))
    })
}

// function converted serialized string(hex) into unsigned integer 64(bits)
pub fn from_u64_hex_str<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;

    if !s.starts_with("0x") {
        return Err(serde::de::Error::custom(format!(
            "Expect hex string {} starts with '0x' prefix",
            s
        )));
    }

    u64::from_str_radix(&s[2..], 16).map_err(|e| {
        serde::de::Error::custom(format!("Failed to parse hex string: {}", e))
    })
}

// function converted serialized string(hex) into unsigned integer 128(bits)
pub fn from_u128_hex_str<'de, D>(deserializer: D) -> Result<u128, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;

    if !s.starts_with("0x") {
        return Err(serde::de::Error::custom(format!(
            "Expect hex string {} starts with '0x' prefix",
            s
        )));
    }
    u128::from_str_radix(&s[2..], 16).map_err(|e| {
        serde::de::Error::custom(format!("Failed to parse hex string: {}", e))
    })
}

// function conver serialized hex timestamp into Utc DateTime
pub fn from_unix_timestamp_hex_str<'de, D>(
    deserializer: D,
) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    // first convert the serialized string into u32
    let timestamp_u32 = from_i32_hex_str(deserializer).map_err(|e| {
        serde::de::Error::custom(format!(
            "Invalid hex string for timestapm: {}",
            e
        ))
    })?;

    Utc.timestamp_opt(timestamp_u32.into(), 0)
        .single()
        .ok_or_else(|| {
            serde::de::Error::custom("Failed to convert timestamp to DateTime")
        })
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
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
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
        assert_eq!(
            result.value,
            Some(i32::from_str_radix("294724", 16).unwrap())
        );

        let obj = TestStructWith_i32_opt {
            id: "1".to_string(),
            // 0x20 -> 1 * 16 ^ 0 + 2 * 16 ^ 1
            //value: Some(1 * 16 ^ 0 + 2 * 16 ^ 1 as i32),
            value: Some(i32::from_str_radix("20", 16).unwrap()),
        };

        let json_value = json!({"id": "1", "value": "0x20"});
        let json_obj =
            serde_json::from_value::<TestStructWith_i32_opt>(json_value)
                .unwrap();
        assert_eq!(json_obj, obj);
    }

    // --- i32

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestStructWith_i32 {
        pub id: String,
        #[serde(deserialize_with = "from_i32_hex_str")]
        pub value: i32,
    }
    #[test]
    fn from_i32_hex_str_test() {
        let data = json!({"id":"2", "value": "0x294724"});
        let result: TestStructWith_i32 = serde_json::from_value(data).unwrap();
        assert_eq!(result.value, i32::from_str_radix("294724", 16).unwrap());

        let obj = TestStructWith_i32 {
            id: "2".to_string(),
            value: i32::from_str_radix("294724", 16).unwrap(),
        };
        let json_value = json!({"id":"2", "value": "0x294724"});
        let json_obj: TestStructWith_i32 =
            serde_json::from_value::<TestStructWith_i32>(json_value).unwrap();
        assert_eq!(json_obj, obj);
    }

    // --- u32
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestStructWith_u32 {
        pub id: String,
        #[serde(deserialize_with = "from_u32_hex_str")]
        pub value: u32,
    }
    #[test]
    fn from_u32_hex_str_test() {
        // u32: min: 0x00000000, max: 0xFFFFFFFF

        let data = json!({"id": "3", "value": "0x00000000"});
        let result: TestStructWith_u32 = serde_json::from_value(data).unwrap();
        assert_eq!(result.value, u32::from_str_radix("0", 16).unwrap());

        let json_value = json!({"id":"4", "value": "0xFFFFFFFF"});
        let json_obj =
            serde_json::from_value::<TestStructWith_u32>(json_value).unwrap();
        let obj = TestStructWith_u32 {
            id: "4".to_string(),
            value: u32::from_str_radix("FFFFFFFF", 16).unwrap(),
        };
        assert_eq!(json_obj, obj);
    }

    //-- u64
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestStructWith_u64 {
        pub id: String,
        #[serde(deserialize_with = "from_u64_hex_str")]
        pub value: u64,
    }
    fn from_u64_hex_str_test() {
        // u64, min: 0x0, max: 0xFFFFFFFFFFFFFFFF
        let data = json!({"id": "5", "value": "0xFFFFFFFFFFFFFFFF"});
        let result: TestStructWith_u64 = serde_json::from_value(data).unwrap();
        assert_eq!(
            result.value,
            u64::from_str_radix("FFFFFFFFFFFFFFFF", 16).unwrap()
        );

        let json_value = json!({"id":"6", "value": "0x0"});
        let json_obj =
            serde_json::from_value::<TestStructWith_u64>(json_value).unwrap();
        let obj = TestStructWith_u64 {
            id: "6".to_string(),
            value: u64::from_str_radix("0", 16).unwrap(),
        };
        assert_eq!(obj, json_obj);
    }

    //-- u128
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestStructWith_u128 {
        pub id: String,
        #[serde(deserialize_with = "from_u128_hex_str")]
        pub value: u128,
    }
    fn from_u128_hex_str_test() {
        // u128, min:0x0  max: 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
        let data =
            json!({"id": "7", "value": "0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"});
        let result: TestStructWith_u128 = serde_json::from_value(data).unwrap();
        assert_eq!(
            result.value,
            u128::from_str_radix("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF", 16)
                .unwrap()
        );

        let obj = TestStructWith_u128 {
            id: "7".to_string(),
            value: u128::from_str_radix("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF", 16)
                .unwrap(),
        };

        assert_eq!(obj, result);
    }

    //-- datetime utc
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestStructWith_ts_hex {
        pub id: String,
        #[serde(deserialize_with = "from_unix_timestamp_hex_str")]
        pub value: DateTime<Utc>,
    }
    #[test]
    fn from_unix_timestamp_hex_str_test() {
        let data = json!({"id": "8", "value": "0x5f5b0d9d"});
        let result = serde_json::from_value::<TestStructWith_ts_hex>(data);
        let date_time = result.unwrap().value;

        let invalid_data = json!({"id": "8", "value": "0xFFFFFFFFF"});
        let result =
            serde_json::from_value::<TestStructWith_ts_hex>(invalid_data);
        assert!(result.is_err());
    }
}
