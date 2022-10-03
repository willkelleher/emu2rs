use serde::{de::Error, Deserialize, Deserializer};

// These are some hacky hex deserializers because I couldn't get others to work.

pub fn from_hex<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    u32::from_str_radix(&s[2..], 16).map_err(D::Error::custom)
}

pub fn from_hex_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(s)
}

pub fn from_hex_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(s.eq("Y"))
}