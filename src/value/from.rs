use chrono::{NaiveDate, NaiveDateTime};

use super::Value;
use crate::record::Record;
use serde_json::Value as JsonValue;

// Implement Into<Value> for supported types
impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value.to_string())
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.to_string())
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<char> for Value {
    fn from(value: char) -> Self {
        Value::Char(value)
    }
}

impl From<i8> for Value {
    fn from(value: i8) -> Self {
        Value::I8(value)
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Value::I16(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::I32(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::I64(value)
    }
}

impl From<i128> for Value {
    fn from(value: i128) -> Self {
        Value::I128(value)
    }
}

impl From<isize> for Value {
    fn from(value: isize) -> Self {
        Value::ISize(value)
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Value::U8(value)
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Value::U16(value)
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Value::U32(value)
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Value::U64(value)
    }
}

impl From<u128> for Value {
    fn from(value: u128) -> Self {
        Value::U128(value)
    }
}

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        Value::USize(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::F32(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::F64(value)
    }
}

impl From<Vec<u8>> for Value {
    fn from(value: Vec<u8>) -> Self {
        Value::Blob(value)
    }
}

impl From<NaiveDate> for Value {
    fn from(value: NaiveDate) -> Self {
        Value::Date(value)
    }
}

impl From<NaiveDateTime> for Value {
    fn from(value: NaiveDateTime) -> Self {
        Value::Timestamp(value)
    }
}

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Value::Collection(value)
    }
}

impl From<Record> for Value {
    fn from(value: Record) -> Self {
        Value::Record(value)
    }
}

impl From<JsonValue> for Value {
    fn from(json_value: JsonValue) -> Self {
        match json_value {
            JsonValue::Bool(b) => Value::Bool(b),
            JsonValue::Number(n) => {
                if let Some(u) = n.as_u64() {
                    match u8::try_from(u) {
                        Ok(val) => Value::U8(val),
                        Err(_) => match u16::try_from(u) {
                            Ok(val) => Value::U16(val),
                            Err(_) => match u32::try_from(u) {
                                Ok(val) => Value::U32(val),
                                Err(_) => match u64::try_from(u) {
                                    Ok(val) => Value::U64(val),
                                    Err(_) => match u128::try_from(u) {
                                        Ok(val) => Value::U128(val),
                                        Err(_) => Value::None,
                                    },
                                },
                            },
                        },
                    }
                } else if let Some(i) = n.as_i64() {
                    match i8::try_from(i) {
                        Ok(val) => Value::I8(val),
                        Err(_) => match i16::try_from(i) {
                            Ok(val) => Value::I16(val),
                            Err(_) => match i32::try_from(i) {
                                Ok(val) => Value::I32(val),
                                Err(_) => match i64::try_from(i) {
                                    Ok(val) => Value::I64(val),
                                    Err(_) => match i128::try_from(i) {
                                        Ok(val) => Value::I128(val),
                                        Err(_) => Value::None,
                                    },
                                },
                            },
                        },
                    }
                } else if let Some(f) = n.as_f64() {
                    Value::F64(f)
                } else {
                    Value::None
                }
            }
            JsonValue::String(s) => {
                if s.len() == 1 {
                    Value::Char(s.chars().next().unwrap_or_default())
                } else if let Ok(date) = NaiveDate::parse_from_str(&s, "%Y-%m-%d") {
                    Value::Date(date)
                } else if let Ok(timestamp) = NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S")
                {
                    Value::Timestamp(timestamp)
                } else {
                    Value::String(s)
                }
            }
            JsonValue::Array(arr) => {
                let collection: Vec<Value> = arr.into_iter().map(Value::from).collect();
                Value::Collection(collection)
            }
            JsonValue::Object(_) => Value::Record(Record::from(json_value)),
            JsonValue::Null => Value::None,
        }
    }
}

#[cfg(test)]
mod tests;
