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

// core/table

// ----------------------------------------------------------------

use std::mem;

use crate::core::domain::{Table, Value};

// ----------------------------------------------------------------

pub fn merge_tables(mut dst: Table, src: Table) -> Table {
    for (key, src_value) in src {
        let dst_value = dst.get_mut(&key).map(mem::take);

        match (dst_value, src_value) {
            (Some(Value::Nested(mut dst_nested)), Value::Nested(src_nested)) => {
                dst_nested = merge_tables(mem::take(&mut dst_nested), src_nested);
                dst.insert(key, Value::Nested(dst_nested));
            }
            (Some(Value::Array(mut dst_array)), Value::Array(src_array)) => {
                dst_array.extend(src_array);
                dst.insert(key, Value::Array(dst_array));
            }
            (_, other_value) => {
                dst.insert(key, other_value);
            }
        }
    }

    dst
}
