use anyhow::anyhow;
use std::cmp::Ordering;

use serde_json::{Value, json};
use std::num::FpCategory;

/// Is a floating point number native zero?
fn is_negative_zero(n: f64) -> bool {
    matches!(n.total_cmp(&-0.0), Ordering::Equal)
}

pub fn convex_json(value: &mut Value) {
    match value {
        Value::Number(num) => {
            if let Some(i) = num.as_i64() {
                *value = serde_json::json!({ "$integer": JsonInteger::encode(i) });
            } else if let Some(f) = num.as_f64() {
                let mut is_special = is_negative_zero(f);
                is_special |= match f.classify() {
                    FpCategory::Zero | FpCategory::Normal | FpCategory::Subnormal => false,
                    FpCategory::Infinite | FpCategory::Nan => true,
                };
                if is_special {
                    *value = json!({ "$float": JsonFloat::encode(f) });
                } else {
                    *value = json!(f);
                }
            }
        }
        Value::Array(arr) => {
            for item in arr.iter_mut() {
                convex_json(item);
            }
        }
        Value::Object(obj) => {
            for (_, item) in obj.iter_mut() {
                convex_json(item);
            }
        }
        _ => {} // Null, Bool remain as is
    }
}

/// Helper functions for encoding `Int64`s as `String`s.
pub enum JsonInteger {}

impl JsonInteger {
    /// Encode an integer as a string.
    pub fn encode(n: i64) -> String {
        base64::encode(n.to_le_bytes())
    }

    /// Decode an integer from a string.
    pub fn decode(s: String) -> anyhow::Result<i64> {
        let bytes: [u8; 8] = base64::decode(s.as_bytes())?
            .try_into()
            .map_err(|_| anyhow!("Int64 must be exactly eight bytes"))?;
        Ok(i64::from_le_bytes(bytes))
    }
}

// /// Helper functions for encoding `Bytes`s as `String`s.
// pub enum JsonBytes {}
//
// impl JsonBytes {
//     /// Encode a binary string as a string.
//     pub fn encode(bytes: &Vec<u8>) -> String {
//         base64::encode(&bytes[..])
//     }
//
//     /// Decode a binary string from a string.
//     pub fn decode(s: String) -> anyhow::Result<Vec<u8>> {
//         Ok(base64::decode(s.as_bytes())?)
//     }
// }

/// Helper functions for encoding `f64`s as `String`s.
pub enum JsonFloat {}

impl JsonFloat {
    /// Encode an `f64` as a string.
    pub fn encode(n: f64) -> String {
        base64::encode(n.to_le_bytes())
    }

    /// Decode an `f64` from a string.
    pub fn decode(s: String) -> anyhow::Result<f64> {
        let bytes: [u8; 8] = base64::decode(s.as_bytes())?
            .try_into()
            .map_err(|_| anyhow!("Float64 must be exactly eight bytes"))?;
        Ok(f64::from_le_bytes(bytes))
    }
}
