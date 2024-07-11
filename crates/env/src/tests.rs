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
use std::time::{SystemTime, UNIX_EPOCH};

use crate::core::domain::{Table, Value};
use crate::core::table::merge_tables;

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
    let seed = current_time_millis();
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

fn current_time_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("SystemTime before UNIX EPOCH!")
        .as_millis()
}
