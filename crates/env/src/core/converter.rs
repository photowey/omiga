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

// core/converter

// ----------------------------------------------------------------

use chrono::NaiveDateTime;

use omigacore::collection::table::{Array, Table, Value};

use crate::core::error::ConfigError;

// ----------------------------------------------------------------

pub struct ValueConverter;

// ----------------------------------------------------------------

impl ValueConverter {
    pub fn try_datetime(rvt: Result<&Value, ConfigError>) -> Option<&NaiveDateTime> {
        match rvt {
            Ok(v) => v.into(),
            _ => None,
        }
    }

    pub fn try_string(rvt: Result<&Value, ConfigError>) -> Option<&String> {
        match rvt {
            Ok(v) => v.into(),
            _ => None,
        }
    }

    pub fn try_str(rvt: Result<&Value, ConfigError>) -> Option<&str> {
        match rvt {
            Ok(Value::String(s)) => Some(s),
            _ => None,
        }
    }

    pub fn try_bool(rvt: Result<&Value, ConfigError>) -> Option<&bool> {
        match rvt {
            Ok(Value::Boolean(b)) => Some(b),
            _ => None,
        }
    }

    pub fn try_nested(rvt: Result<&Value, ConfigError>) -> Option<&Table> {
        match rvt {
            Ok(v) => v.into(),
            _ => None,
        }
    }

    pub fn try_array(rvt: Result<&Value, ConfigError>) -> Option<&Array> {
        match rvt {
            Ok(v) => v.into(),
            _ => None,
        }
    }

    pub fn try_int_u128(rvt: Result<&Value, ConfigError>) -> Option<&u128> {
        match rvt {
            Ok(v) => v.into(),
            _ => None,
        }
    }

    pub fn try_int_u64(rvt: Result<&Value, ConfigError>) -> Option<&u64> {
        match rvt {
            Ok(v) => v.into(),
            _ => None,
        }
    }

    pub fn try_int_u32(rvt: Result<&Value, ConfigError>) -> Option<&u32> {
        match rvt {
            Ok(v) => v.into(),
            _ => None,
        }
    }

    pub fn try_int_i128(rvt: Result<&Value, ConfigError>) -> Option<&i128> {
        match rvt {
            Ok(v) => v.into(),
            _ => None,
        }
    }

    pub fn try_int_i64(rvt: Result<&Value, ConfigError>) -> Option<&i64> {
        match rvt {
            Ok(v) => v.into(),
            _ => None,
        }
    }

    pub fn try_int_i32(rvt: Result<&Value, ConfigError>) -> Option<&i32> {
        match rvt {
            Ok(v) => v.into(),
            _ => None,
        }
    }

    pub fn try_float64(rvt: Result<&Value, ConfigError>) -> Option<&f64> {
        match rvt {
            Ok(v) => v.into(),
            _ => None,
        }
    }

    pub fn try_float32(rvt: Result<&Value, ConfigError>) -> Option<&f32> {
        match rvt {
            Ok(v) => v.into(),
            _ => None,
        }
    }

    pub fn try_none(rvt: Result<&Value, ConfigError>) -> Option<&()> {
        match rvt {
            Ok(v) => v.into(),
            _ => None,
        }
    }
}
