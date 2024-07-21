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

#![allow(dead_code)]

// clock/datetime

// ----------------------------------------------------------------

use chrono::{Local, NaiveDateTime, Utc};

// ----------------------------------------------------------------

pub fn now() -> NaiveDateTime {
    now_local()
}

// ----------------------------------------------------------------

pub fn now_utc() -> NaiveDateTime {
    let dt = Utc::now();
    dt.naive_local()
}

pub fn now_local() -> NaiveDateTime {
    let dt = Local::now();
    dt.naive_local()
}

// ----------------------------------------------------------------

pub fn timestamp_seconds_local(dt: &NaiveDateTime) -> i64 {
    dt.and_local_timezone(Local).unwrap().timestamp()
}

pub fn timestamp_seconds_utc(dt: &NaiveDateTime) -> i64 {
    dt.and_local_timezone(Utc).unwrap().timestamp()
}

// ----------------------------------------------------------------

pub fn timestamp_millis_local(dt: &NaiveDateTime) -> i64 {
    dt.and_local_timezone(Local).unwrap().timestamp_millis()
}

pub fn timestamp_millis_utc(dt: &NaiveDateTime) -> i64 {
    dt.and_local_timezone(Utc).unwrap().timestamp_millis()
}

// ----------------------------------------------------------------
