use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer};

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
    // the first thing we take the deserializer is invoke serde this serialize framework's deserialize method
    // load & invoke the deserializer instance to exeucte it's deserialize function process the deserialize funciton
    // then we extract a string from the serialized data
    // the string is always looks like this 0x294724... this is a 16 radix string
    let s: String = Deserialize::deserialize(deserializer)?;

    // here we conveted the 16 radix string into it's original i32 value
    // string: 0x294724 -> integer: 294724(hex/16 radix) ->  2 * 16^5 + 9 * 16^4 + 4 * 16^3 + 7 * 16^2 + 2 * 16^1 + 4 * 16^0 -> 2_694_356
    // and that's why &s[2..] to skip the first '0x' character
    Ok(i32::from_str_radix(&s[2..], 16).unwrap())
}
