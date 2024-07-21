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

// boolean

// ----------------------------------------------------------------

use crate::constants::{ONE, ONE_INT, ZERO, ZERO_INT};

// ----------------------------------------------------------------

#[derive(Debug)]
pub enum Boolean {
    True,
    False,
}

// ----------------------------------------------------------------

impl Boolean {
    pub fn to_str(&self) -> &str {
        match self {
            Boolean::True => ONE,
            Boolean::False => ZERO,
        }
    }

    pub fn to_string(&self) -> String {
        self.to_str().to_string()
    }

    pub fn to_int(&self) -> u8 {
        match self {
            Boolean::True => ONE_INT,
            Boolean::False => ZERO_INT,
        }
    }

    pub fn to_bool(&self) -> bool {
        match self {
            Boolean::True => true,
            Boolean::False => false,
        }
    }
}
