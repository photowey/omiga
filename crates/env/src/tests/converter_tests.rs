/*
 * Copyright © 2024 the original author or authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// tests/converter_tests

// ----------------------------------------------------------------

use std::f32::consts::PI;

use chrono::NaiveDateTime;
use chronounit::formatter::pattern::DateTimePattern;

use omigacore::clock::timestamp;
use omigacore::collection::table::{Array, Table, Value};

use crate::core::converter::ValueConverter;
use crate::core::error::FileError;
use crate::env::standard::StandardEnvironment;
use crate::env::Environment;
use crate::reader::registry::ConfigReaderRegistry;
use crate::reader::toml::TomlConfigReader;

// ----------------------------------------------------------------

#[test]
#[rustfmt::skip]
fn test_get_converter_nested() {
    let rvt = new_standard_env();
    let mut environment = rvt.unwrap();

    let mut nested = Table::new();
    nested.insert("Hello".to_string(), Value::String("Rust".to_string()));

    environment.set("io.github.photowey.nested", Value::Nested(nested)).unwrap();

    let rvt_nested = environment.get("io.github.photowey.nested");

    if let Some(into_value) = ValueConverter::try_nested(rvt_nested) {
        match into_value.get("Hello") {
            Some(v) => {
                assert_eq!(*v, Value::String("Rust".to_string()));
            }
            _ => {}
        }
    } else {
        panic!("failed to convert the value to Table")
    }
}

#[test]
#[rustfmt::skip]
fn test_get_converter_array() {
    let rvt = new_standard_env();
    let mut environment = rvt.unwrap();

    let now = 1710265983u32;
    let mut array = Array::new();
    array.push(Value::String("Rust".to_string()));
    array.push(Value::IntU32(now));

    environment.set("io.github.photowey.array", Value::Array(array)).unwrap();

    let rvt_array = environment.get("io.github.photowey.array");

    let mut image = Array::new();
    image.push(Value::String("Rust".to_string()));
    image.push(Value::IntU32(now));

    if let Some(into_value) = ValueConverter::try_array(rvt_array) {
        assert!(assert_array_equals(into_value, &image));
    } else {
        panic!("failed to convert the value to Table")
    }
}

fn assert_array_equals(array: &Array, vec: &Array) -> bool {
    array.iter().zip(vec.iter()).all(|(a, b)| a == b)
}

#[test]
#[rustfmt::skip]
fn test_get_converter_date_time() {
    let rvt = new_standard_env();
    let mut environment = rvt.unwrap();

    let now = NaiveDateTime::parse_from_str("2024-07-21 18:15:00", DateTimePattern::YYYY_MM_DD_HH_MM_SS).unwrap();
    environment.set("io.github.photowey.environment.Time", now.into()).unwrap();

    let rvt_time = environment.get("io.github.photowey.environment.Time");

    // match
    match rvt_time {
        Ok(v) => {
            match v {
                Value::DateTime(ref time) => {
                    assert_eq!(*time, now);
                }
                _ => {}
            }
        }
        _ => {}
    }

    // converter
    if let Some(into_value) = ValueConverter::try_datetime(rvt_time) {
        assert_eq!(*into_value, now);
    } else {
        panic!("failed to convert the value to NaiveDateTime")
    }
}

#[test]
#[rustfmt::skip]
fn test_get_converter_string() {
    let rvt = new_standard_env();
    let mut environment = rvt.unwrap();

    environment.set("io.github.photowey.str", String::from("Rust").into()).unwrap();
    let rvt_string = environment.get("io.github.photowey.str");

    if let Some(into_value) = ValueConverter::try_string(rvt_string) {
        assert_eq!(*into_value, String::from("Rust"));
    } else {
        panic!("failed to convert the value to String")
    }
}

#[test]
#[rustfmt::skip]
fn test_get_converter_str() {
    let rvt = new_standard_env();
    let mut environment = rvt.unwrap();

    environment.set("io.github.photowey.str", "Rust".into()).unwrap();
    let rvt_str = environment.get("io.github.photowey.str");

    if let Some(into_value) = ValueConverter::try_str(rvt_str) {
        assert_eq!(into_value, "Rust");
    } else {
        panic!("failed to convert the value to &str")
    }
}

#[test]
#[rustfmt::skip]
fn test_get_converter_bool() {
    let rvt = new_standard_env();
    let mut environment = rvt.unwrap();

    environment.set("io.github.photowey.bool", false.into()).unwrap();
    let rvt_bool = environment.get("io.github.photowey.bool");

    if let Some(into_value) = ValueConverter::try_bool(rvt_bool) {
        assert_eq!(*into_value, false);
    } else {
        panic!("failed to convert the value to false")
    }
}

#[test]
#[rustfmt::skip]
fn test_get_converter_u128() {
    let rvt = new_standard_env();
    let mut environment = rvt.unwrap();

    let now = timestamp::now();
    environment.set("io.github.photowey.u128", now.into()).unwrap();
    let rvt_u128 = environment.get("io.github.photowey.u128");

    if let Some(into_value) = ValueConverter::try_int_u128(rvt_u128) {
        assert_eq!(*into_value, now);
    } else {
        panic!("failed to convert the value to u128")
    }
}

#[test]
#[rustfmt::skip]
fn test_get_converter_u64() {
    let rvt = new_standard_env();
    let mut environment = rvt.unwrap();

    let now = timestamp::now() as u64;
    environment.set("io.github.photowey.u64", now.into()).unwrap();
    let rvt_u64 = environment.get("io.github.photowey.u64");

    if let Some(into_value) = ValueConverter::try_int_u64(rvt_u64) {
        assert_eq!(*into_value, now);
    } else {
        panic!("failed to convert the value to u64")
    }
}

#[test]
#[rustfmt::skip]
fn test_get_converter_u32() {
    let rvt = new_standard_env();
    let mut environment = rvt.unwrap();

    let rvt = 1710265983u32;
    environment.set("io.github.photowey.u32", rvt.into()).unwrap();
    let rvt_u32 = environment.get("io.github.photowey.u32");

    if let Some(into_value) = ValueConverter::try_int_u32(rvt_u32) {
        assert_eq!(*into_value, rvt);
    } else {
        panic!("failed to convert the value to u32")
    }
}

#[test]
#[rustfmt::skip]
fn test_get_converter_i128() {
    let rvt = new_standard_env();
    let mut environment = rvt.unwrap();

    let now = timestamp::now() as i128;
    environment.set("io.github.photowey.i128", now.into()).unwrap();
    let rvt_i128 = environment.get("io.github.photowey.i128");

    if let Some(into_value) = ValueConverter::try_int_i128(rvt_i128) {
        assert_eq!(*into_value, now);
    } else {
        panic!("failed to convert the value to i128")
    }
}

#[test]
#[rustfmt::skip]
fn test_get_converter_i64() {
    let rvt = new_standard_env();
    let mut environment = rvt.unwrap();

    let now = timestamp::now() as i64;
    environment.set("io.github.photowey.i64", now.into()).unwrap();
    let rvt_i64 = environment.get("io.github.photowey.i64");

    if let Some(into_value) = ValueConverter::try_int_i64(rvt_i64) {
        assert_eq!(*into_value, now);
    } else {
        panic!("failed to convert the value to i64")
    }
}

#[test]
#[rustfmt::skip]
fn test_get_converter_i32() {
    let rvt = new_standard_env();
    let mut environment = rvt.unwrap();

    let rvt = 1710265983i32;
    environment.set("io.github.photowey.i32", rvt.into()).unwrap();
    let rvt_i32 = environment.get("io.github.photowey.i32");

    if let Some(into_value) = ValueConverter::try_int_i32(rvt_i32) {
        assert_eq!(*into_value, rvt);
    } else {
        panic!("failed to convert the value to i32")
    }
}

#[test]
#[rustfmt::skip]
fn test_get_converter_f64() {
    let rvt = new_standard_env();
    let mut environment = rvt.unwrap();

    let rvt = PI as f64;
    environment.set("io.github.photowey.f64", rvt.into()).unwrap();
    let rvt_f64 = environment.get("io.github.photowey.f64");

    if let Some(into_value) = ValueConverter::try_float64(rvt_f64) {
        assert_eq!(*into_value, rvt);
    } else {
        panic!("failed to convert the value to f64")
    }
}

#[test]
#[rustfmt::skip]
fn test_get_converter_f32() {
    let rvt = new_standard_env();
    let mut environment = rvt.unwrap();

    let rvt = PI;
    environment.set("io.github.photowey.f32", rvt.into()).unwrap();
    let rvt_f32 = environment.get("io.github.photowey.f32");

    if let Some(into_value) = ValueConverter::try_float32(rvt_f32) {
        assert_eq!(*into_value, rvt);
    } else {
        panic!("failed to convert the value to f32")
    }
}

#[test]
#[rustfmt::skip]
fn test_get_converter_none() {
    let rvt = new_standard_env();
    let mut environment = rvt.unwrap();

    let none = Value::None;
    environment.set("io.github.photowey.none", none).unwrap();
    let rvt_none = environment.get("io.github.photowey.none");

    if let Some(into_value) = ValueConverter::try_none(rvt_none) {
        assert_eq!(*into_value, ());
    } else {
        panic!("failed to convert the value to none")
    }
}

// ----------------------------------------------------------------

fn new_standard_env() -> Result<StandardEnvironment, FileError> {
    StandardEnvironment::builder()
        .with_table(Table::new())
        .with_registry(Box::new(ConfigReaderRegistry::default()))
        .with_reader(Box::new(TomlConfigReader::default()))
        .with_config("omiga".to_string())
        .with_profile("dev".to_string())
        .with_format("toml".to_string())
        .build()
}
