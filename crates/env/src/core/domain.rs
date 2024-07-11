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

// core/domain

// ----------------------------------------------------------------

use std::collections::HashMap;

use chrono::NaiveDateTime;

// ----------------------------------------------------------------

pub type Table = HashMap<String, Value>;

pub type Array = Vec<Value>;

// ----------------------------------------------------------------

pub type OmigaTable = Table;
pub type OmigaValue = Value;

// ----------------------------------------------------------------

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Nested(Table),
    Array(Array),
    DateTime(NaiveDateTime),
    String(String),
    Boolean(bool),
    IntU128(u128),
    IntU64(u64),
    IntU32(u32),
    Int128(i128),
    Int64(i64),
    Int32(i32),
    Float64(f64),
    Float32(f32),
    None,
}

// ----------------------------------------------------------------

impl Value {
    pub fn as_nested_mut(&mut self) -> Option<&mut Table> {
        match self {
            Value::Nested(ref mut nested) => Some(nested),
            _ => None,
        }
    }
}

// ----------------------------------------------------------------

impl Default for Value {
    fn default() -> Self {
        Self::None
    }
}

// ----------------------------------------------------------------

impl From<Table> for Value {
    fn from(value: Table) -> Self {
        Value::Nested(value)
    }
}

// ----------------------------------------------------------------

impl From<Array> for Value {
    fn from(value: Array) -> Self {
        Value::Array(value)
    }
}

// ----------------------------------------------------------------

impl From<NaiveDateTime> for Value {
    fn from(value: NaiveDateTime) -> Self {
        Value::DateTime(value)
    }
}

// ----------------------------------------------------------------

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.to_string())
    }
}

// ----------------------------------------------------------------

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Boolean(value)
    }
}

// ----------------------------------------------------------------

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Int32(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Int64(value)
    }
}

impl From<i128> for Value {
    fn from(value: i128) -> Self {
        Value::Int128(value)
    }
}

// ----------------------------------------------------------------

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Value::IntU32(value)
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Value::IntU64(value)
    }
}

impl From<u128> for Value {
    fn from(value: u128) -> Self {
        Value::IntU128(value)
    }
}

// ----------------------------------------------------------------

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Float64(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::Float32(value)
    }
}

// ----------------------------------------------------------------

impl<'a> From<&'a Value> for Option<&'a Table> {
    fn from(value: &'a Value) -> Option<&'a Table> {
        match *value {
            Value::Nested(ref table) => Some(table),
            _ => None,
        }
    }
}

impl<'a> From<&'a Value> for Option<&'a Array> {
    fn from(value: &'a Value) -> Option<&'a Array> {
        match *value {
            Value::Array(ref array) => Some(array),
            _ => None,
        }
    }
}

impl<'a> From<&'a Value> for Option<&'a NaiveDateTime> {
    fn from(value: &'a Value) -> Option<&'a NaiveDateTime> {
        match *value {
            Value::DateTime(ref time) => Some(time),
            _ => None,
        }
    }
}

impl<'a> From<&'a Value> for Option<&'a String> {
    fn from(value: &'a Value) -> Option<&'a String> {
        match *value {
            Value::String(ref v) => Some(v),
            _ => None,
        }
    }
}

impl<'a> From<&'a Value> for Option<&'a bool> {
    fn from(value: &'a Value) -> Option<&'a bool> {
        match *value {
            Value::Boolean(ref v) => Some(v),
            _ => None,
        }
    }
}

impl<'a> From<&'a Value> for Option<&'a i128> {
    fn from(value: &'a Value) -> Option<&'a i128> {
        match *value {
            Value::Int128(ref v) => Some(v),
            _ => None,
        }
    }
}

impl<'a> From<&'a Value> for Option<&'a u128> {
    fn from(value: &'a Value) -> Option<&'a u128> {
        match *value {
            Value::IntU128(ref v) => Some(v),
            _ => None,
        }
    }
}

impl<'a> From<&'a Value> for Option<&'a i64> {
    fn from(value: &'a Value) -> Option<&'a i64> {
        match *value {
            Value::Int64(ref v) => Some(v),
            _ => None,
        }
    }
}

impl<'a> From<&'a Value> for Option<&'a u64> {
    fn from(value: &'a Value) -> Option<&'a u64> {
        match *value {
            Value::IntU64(ref v) => Some(v),
            _ => None,
        }
    }
}

impl<'a> From<&'a Value> for Option<&'a i32> {
    fn from(value: &'a Value) -> Option<&'a i32> {
        match *value {
            Value::Int32(ref v) => Some(v),
            _ => None,
        }
    }
}

impl<'a> From<&'a Value> for Option<&'a u32> {
    fn from(value: &'a Value) -> Option<&'a u32> {
        match *value {
            Value::IntU32(ref v) => Some(v),
            _ => None,
        }
    }
}

impl<'a> From<&'a Value> for Option<&'a f64> {
    fn from(value: &'a Value) -> Option<&'a f64> {
        match *value {
            Value::Float64(ref v) => Some(v),
            _ => None,
        }
    }
}

impl<'a> From<&'a Value> for Option<&'a f32> {
    fn from(value: &'a Value) -> Option<&'a f32> {
        match *value {
            Value::Float32(ref v) => Some(v),
            _ => None,
        }
    }
}

impl<'a> From<&'a Value> for Option<&'a ()> {
    fn from(value: &'a Value) -> Option<&'a ()> {
        match *value {
            Value::None => Some(&()),
            _ => None,
        }
    }
}
