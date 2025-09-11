use chrono::{Local, NaiveDate};
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
    let json_value = json!(-123);
    let value = Value::from(json_value);
    assert_eq!(value, Value::I8(-123));
}

#[test]
fn test_value_from_json_i16() {
    let json_value = json!(-258);
    let value = Value::from(json_value);
    assert_eq!(value, Value::I16(-258));
}

#[test]
fn test_value_from_json_i32() {
    let json_value = json!(-42258);
    let value = Value::from(json_value);
    assert_eq!(value, Value::I32(-42258));
}

#[test]
fn test_value_from_json_isize() {
    let json_value = json!(-9223372036854775808 as i64);
    let value = Value::from(json_value);
    assert_eq!(value, Value::ISize(-9223372036854775808));
}

#[test]
fn test_value_from_json_array() {
    let json_value = json!([-11, "abc", true]);
    let value = Value::from(json_value);
    assert_eq!(
        value,
        Value::Collection(vec![
            Value::I8(-11),
            Value::String("abc".to_string()),
            Value::Bool(true)
        ])
    );
}

#[test]
fn test_value_from_json_u8() {
    let json_value = json!(223 as u8);
    let value = Value::from(json_value);
    assert_eq!(value, Value::U8(223));
}

#[test]
fn test_value_from_json_u16() {
    let json_value = json!(258 as u16);
    let value = Value::from(json_value);
    assert_eq!(value, Value::U16(258));
}

#[test]
fn test_value_from_json_u32() {
    let json_value = json!(7342258);
    let value = Value::from(json_value);
    assert_eq!(value, Value::U32(7342258));
}

#[test]
fn test_value_from_json_usize() {
    let json_value = json!(9223372036854775808 as usize);
    let value = Value::from(json_value);
    assert_eq!(value, Value::USize(9223372036854775808));
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
        .push(Field::new_value("age", Value::U8(30)));
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

#[test]
fn test_from_date() {
    let now = Local::now().naive_local().date();
    let v: Value = now.into();
    assert!(matches!(v, Value::Date(_)));
}

#[test]
fn test_from_collection() {
    let vec: Vec<Value> = vec![
        Value::I8(42),
        Value::F32(73.0),
        Value::String(String::from("string")),
    ];
    let v: Value = vec.into();
    assert!(matches!(v, Value::Collection(_)));
}

#[test]
fn test_from_record() {
    let r = Record::new();
    let v: Value = r.into();
    assert!(matches!(v, Value::Record(_)));
}
