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

// core/kv

// ----------------------------------------------------------------

use std::collections::HashMap;

// ----------------------------------------------------------------

/// `Omiga` server command k/v args if present.
pub struct Kv {
    pub ctx: Option<HashMap<String, String>>,
}

impl Kv {
    pub fn new() -> Self {
        Self {
            ctx: Some(HashMap::new()),
        }
    }
}

impl Default for Kv {
    fn default() -> Self {
        Self::new()
    }
}
