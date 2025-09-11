use chrono::NaiveDate;
use serde_json::json;

use crate::{field::Field, record::Record, value::Value};

#[test]
fn test_value_from_json_bool() {
    let json_value = json!(true);
    let value = Value::from(json_value);
    assert_eq!(value, Value::Bool(true));
}

#[test]
fn test_value_from_json_i8() {
    let json_value = json!(123);
    let value = Value::from(json_value);
    assert_eq!(value, Value::I8(123));
}

#[test]
fn test_value_from_json_i16() {
    let json_value = json!(258);
    let value = Value::from(json_value);
    assert_eq!(value, Value::I16(258));
}

#[test]
fn test_value_from_json_array() {
    let json_value = json!([1, "abc", true]);
    let value = Value::from(json_value);
    assert_eq!(
        value,
        Value::Collection(vec![
            Value::I8(1),
            Value::String("abc".to_string()),
            Value::Bool(true)
        ])
    );
}

#[test]
fn test_value_from_json_object() {
    let json_value = json!({
        "name": "John Doe",
        "age": 30
    });
    let value = Value::from(json_value);

    let mut expected_record = Record::new();
    expected_record
        .fields_as_mut()
        .push(Field::new_value("age", Value::I8(30)));
    expected_record.fields_as_mut().push(Field::new_value(
        "name",
        Value::String("John Doe".to_string()),
    ));

    assert_eq!(value, Value::Record(expected_record));
}

#[test]
fn test_value_from_json_null() {
    let json_value = json!(null);
    let value = Value::from(json_value);
    assert_eq!(value, Value::None);
}

#[test]
fn test_value_from_json_date() {
    let json_value = json!("2023-10-27");
    let value = Value::from(json_value);
    assert_eq!(
        value,
        Value::Date(NaiveDate::from_ymd_opt(2023, 10, 27).unwrap())
    );
}

#[test]
fn test_value_from_json_char() {
    let json_value = json!('a');
    let value = Value::from(json_value);
    assert_eq!(value, Value::Char('a'));
}

#[test]
fn test_value_from_json_f64() {
    let json_value = json!(123.45);
    let value = Value::from(json_value);
    assert_eq!(value, Value::F64(123.45));
}

#[test]
fn test_from_string() {
    let v: Value = String::from("string value").into();
    assert!(matches!(v, Value::String(_)));
}

#[test]
fn test_from_string_slice() {
    let v: Value = "string value".into();
    assert!(matches!(v, Value::String(_)));
}

#[test]
fn test_from_bool() {
    let v: Value = true.into();
    assert!(matches!(v, Value::Bool(true)));
    let v: Value = false.into();
    assert!(matches!(v, Value::Bool(false)));
}

#[test]
fn test_from_char() {
    let v: Value = 'A'.into();
    assert!(matches!(v, Value::Char('A')));
}

#[test]
fn test_from_i8() {
    let v: Value = (-42 as i8).into();
    assert!(matches!(v, Value::I8(-42)));
}

#[test]
fn test_from_i16() {
    let v: Value = (-4273 as i16).into();
    assert!(matches!(v, Value::I16(-4273)));
}

#[test]
fn test_from_i32() {
    let v: Value = (-42737342 as i32).into();
    assert!(matches!(v, Value::I32(-42737342)));
}

#[test]
fn test_from_i64() {
    let v: Value = (-4273734242737342 as i64).into();
    assert!(matches!(v, Value::I64(-4273734242737342)));
}

#[test]
fn test_from_i128() {
    let v: Value = (-4273734242737342 as i128).into();
    assert!(matches!(v, Value::I128(-4273734242737342)));
}

#[test]
fn test_from_isize() {
    let v: Value = (-42737342 as isize).into();
    assert!(matches!(v, Value::ISize(-42737342)));
}

#[test]
fn test_from_u8() {
    let v: Value = (42 as u8).into();
    assert!(matches!(v, Value::U8(42)));
}

#[test]
fn test_from_u16() {
    let v: Value = (4273 as u16).into();
    assert!(matches!(v, Value::U16(4273)));
}

#[test]
fn test_from_u32() {
    let v: Value = (42737342 as u32).into();
    assert!(matches!(v, Value::U32(42737342)));
}

#[test]
fn test_from_u64() {
    let v: Value = (4273734242737342 as u64).into();
    assert!(matches!(v, Value::U64(4273734242737342)));
}

#[test]
fn test_from_u128() {
    let v: Value = (4273734242737342 as u128).into();
    assert!(matches!(v, Value::U128(4273734242737342)));
}

#[test]
fn test_from_usize() {
    let v: Value = (42737342 as usize).into();
    assert!(matches!(v, Value::USize(42737342)));
}

#[test]
fn test_from_f32() {
    let v: Value = (4273.7342 as f32).into();
    assert!(matches!(v, Value::F32(4273.7342)));
}

#[test]
fn test_from_f64() {
    let v: Value = (42737342427373.42 as f64).into();
    assert!(matches!(v, Value::F64(42737342427373.42)));
}

#[test]
fn test_from_bytes() {
    let vec: Vec<u8> = vec![42, 73];
    let v: Value = vec.into();
    assert!(matches!(v, Value::Blob(_)));
}
