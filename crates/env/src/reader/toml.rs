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

// reader/toml

// ------------------------------------------------------------

use chrono::NaiveDateTime;
use toml::de::Error;
use toml::value::{Date, Datetime, Time};
use toml::Value;

use omiga_core::collection::table::{OmigaTable, OmigaValue};
use omiga_core::constants::{SIGMA_CONFIG_READER_TOML, SIGMA_CONFIG_READER_TOML_FORMAT};

use crate::core::error::FileError;
use crate::reader::ConfigReader;

// ------------------------------------------------------------

pub struct TomlConfigReader {
    name: String,
    suffix: String,
}

impl TomlConfigReader {
    fn new() -> Self {
        Self {
            name: SIGMA_CONFIG_READER_TOML.to_string(),
            suffix: SIGMA_CONFIG_READER_TOML_FORMAT.to_string(),
        }
    }
}

// ----------------------------------------------------------------

impl Default for TomlConfigReader {
    fn default() -> Self {
        Self::new()
    }
}

// ----------------------------------------------------------------

impl ConfigReader for TomlConfigReader {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn suffix(&self) -> String {
        self.suffix.clone()
    }

    fn supports(&self, suffix: &str) -> bool {
        self.suffix.eq(suffix)
    }

    fn read_from_str(&self, data: &str) -> Result<OmigaTable, FileError> {
        let mut ctx: OmigaTable = OmigaTable::new();

        let parsed_rvt: Result<Value, Error> = toml::from_str(data);
        match parsed_rvt {
            Ok(value) => {
                if let Value::Table(table) = value {
                    for (key, value) in table {
                        ctx.insert(key, toml_value_to_sigma_value(value));
                    }

                    return Ok(ctx);
                }

                Err(FileError::IncorrectFormat(
                    SIGMA_CONFIG_READER_TOML.to_string(),
                ))
            }
            Err(err) => Err(FileError::ParseFailed(
                SIGMA_CONFIG_READER_TOML.to_string(),
                err.message().to_string(),
            )),
        }
    }
}

// ----------------------------------------------------------------

pub fn toml_value_to_sigma_value(value: Value) -> OmigaValue {
    match value {
        Value::String(s) => OmigaValue::String(s),
        Value::Integer(i) => OmigaValue::Int64(i),
        Value::Float(f) => OmigaValue::Float64(f),
        Value::Boolean(b) => OmigaValue::Boolean(b),
        Value::Datetime(datetime) => {
            OmigaValue::DateTime(toml_datetime_to_chrono_naive_datetime(datetime).unwrap())
        }
        Value::Array(arr) => {
            OmigaValue::Array(arr.into_iter().map(toml_value_to_sigma_value).collect())
        }
        Value::Table(table) => OmigaValue::Nested(
            table
                .into_iter()
                .map(|(k, v)| (k, toml_value_to_sigma_value(v)))
                .collect(),
        ),
    }
}

pub fn toml_datetime_to_chrono_naive_datetime(datetime: Datetime) -> Option<NaiveDateTime> {
    match (datetime.date, datetime.time, datetime.offset) {
        (Some(date), Some(time), _) => Some(toml_date_time_to_chrono_naive_datetime(date, time)),
        (Some(date), None, None) => Some(toml_date_to_chrono_naive_datetime(date)),
        (None, Some(time), None) => Some(toml_time_to_chrono_naive_datetime(time)),
        _ => None,
    }
}

pub fn toml_date_to_chrono_naive_datetime(date: Date) -> NaiveDateTime {
    NaiveDateTime::new(
        chrono::NaiveDate::from_ymd_opt(date.year as i32, date.month as u32, date.day as u32)
            .unwrap(),
        chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
    )
}

pub fn toml_date_time_to_chrono_naive_datetime(date: Date, time: Time) -> NaiveDateTime {
    NaiveDateTime::new(
        chrono::NaiveDate::from_ymd_opt(date.year as i32, date.month as u32, date.day as u32)
            .unwrap(),
        chrono::NaiveTime::from_hms_opt(time.hour as u32, time.minute as u32, time.second as u32)
            .unwrap(),
    )
}

pub fn toml_time_to_chrono_naive_datetime(time: Time) -> NaiveDateTime {
    NaiveDateTime::new(
        chrono::NaiveDate::from_ymd_opt(0, 0, 0).unwrap(),
        chrono::NaiveTime::from_hms_opt(time.hour as u32, time.minute as u32, time.second as u32)
            .unwrap(),
    )
}
