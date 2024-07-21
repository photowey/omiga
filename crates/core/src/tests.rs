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

// tests

// ----------------------------------------------------------------

use std::f32::consts::PI;

use chrono::{Local, Utc};

use crate::clock::datetime;
use crate::clock::timestamp::now;
use crate::collection::merger::merge_tables;
use crate::collection::table::{Table, Value};

#[cfg(test)]
mod helper_tests;

// ----------------------------------------------------------------

#[test]
fn test_table_merge_tables() {
    let mut table_a: Table = Table::new();

    table_a.insert(
        "io".to_string(),
        Value::Nested({
            let mut inner_table = Table::new();
            inner_table.insert(
                "github".to_string(),
                Value::Array(vec![Value::Int32(1), Value::Int32(3)]),
            );
            inner_table
        }),
    );
    table_a.insert("replaced".to_string(), Value::Float64(PI as f64));

    let mut table_b: Table = Table::new();

    // 1
    table_b.insert(
        "io".to_string(),
        Value::Nested({
            let mut inner_table = Table::new();
            inner_table.insert(
                "github".to_string(),
                Value::Array(vec![Value::Int32(2), Value::Int32(4)]),
            );
            inner_table
        }),
    );
    // 2
    table_b.insert(
        "hello".to_string(),
        Value::Nested({
            let mut inner_table = Table::new();
            inner_table.insert(
                "world".to_string(),
                Value::Array(vec![Value::Int32(2), Value::Int32(4)]),
            );
            inner_table
        }),
    );

    // 3
    let seed = now();
    table_b.insert("replaced".to_string(), Value::IntU128(seed));

    // 4: merge
    let merged_table = merge_tables(table_a, table_b);

    let mut table_sentinel: Table = Table::new();
    table_sentinel.insert(
        "io".to_string(),
        Value::Nested({
            let mut inner_table = Table::new();
            inner_table.insert(
                "github".to_string(),
                Value::Array(vec![
                    Value::Int32(1),
                    Value::Int32(3),
                    Value::Int32(2),
                    Value::Int32(4),
                ]),
            );
            inner_table
        }),
    );
    table_sentinel.insert(
        "hello".to_string(),
        Value::Nested({
            let mut inner_table = Table::new();
            inner_table.insert(
                "world".to_string(),
                Value::Array(vec![Value::Int32(2), Value::Int32(4)]),
            );
            inner_table
        }),
    );
    // replaced
    table_sentinel.insert("replaced".to_string(), Value::IntU128(seed));

    assert_eq!(merged_table, table_sentinel);
}

// ----------------------------------------------------------------

#[test]
fn test_clock_timestamp() {
    // 2024-07-21 00:00:00 GMT+08:00
    // 1721491200000
    let base = 1721491200000u128;
    let now_millis = now();

    assert!(now_millis > base)
}

#[test]
fn test_clock_datetime_local() {
    // 2024-07-21 00:00:00 GMT+08:00
    // 1721491200000
    let base = 1721491200000i64;

    // now
    // 2024-07-21 17:50:00 GMT+08:00
    // 1721555400000
    let now = datetime::now();
    let now_millis = now.and_local_timezone(Local).unwrap().timestamp_millis();

    assert!(now_millis > base)
}

#[test]
fn test_clock_datetime_utc() {
    // 2024-07-21 00:00:00 GMT+08:00
    // 1721491200000
    let base = 1721491200000i64;

    // now
    // 2024-07-21 17:50:00 GMT+08:00
    // 1721555400000
    let now = datetime::now_utc();
    let now_millis = now.and_local_timezone(Utc).unwrap().timestamp_millis();

    assert!(now_millis > base)
}

// ----------------------------------------------------------------

#[test]
fn test_clock_datetime_2_timestamp_millis_local() {
    // 2024-07-21 00:00:00 GMT+08:00
    // 1721491200000
    let base = 1721491200000i64;

    // now
    // 2024-07-21 17:50:00 GMT+08:00
    // 1721555400000
    let now = datetime::now();
    let now_millis = datetime::timestamp_millis_local(&now);

    assert!(now_millis > base)
}

#[test]
fn test_clock_datetime_2_timestamp_millis_utc() {
    // 2024-07-21 00:00:00 GMT+08:00
    // 1721491200000
    let base = 1721491200000i64;

    // now
    // 2024-07-21 17:50:00 GMT+08:00
    // 1721555400000
    let now = datetime::now();
    let now_millis = datetime::timestamp_millis_utc(&now);

    assert!(now_millis > base);
}

// ----------------------------------------------------------------

#[test]
fn test_clock_datetime_2_timestamp_seconds_local() {
    // 2024-07-21 00:00:00 GMT+08:00
    // 1721491200000
    let base = 1721491200000i64;

    // now
    // 2024-07-21 17:50:00 GMT+08:00
    // 1721555400000
    let now = datetime::now();
    let now_millis = datetime::timestamp_seconds_local(&now);

    assert!(now_millis > (base / 1_000))
}

#[test]
fn test_clock_datetime_2_timestamp_seconds_utc() {
    // 2024-07-21 00:00:00 GMT+08:00
    // 1721491200000
    let base = 1721491200000i64;

    // now
    // 2024-07-21 17:50:00 GMT+08:00
    // 1721555400000
    let now = datetime::now();
    let now_millis = datetime::timestamp_seconds_utc(&now);

    assert!(now_millis > (base / 1_000))
}
