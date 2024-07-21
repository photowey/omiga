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

// bean

// ----------------------------------------------------------------

use std::any::Any;

// ----------------------------------------------------------------

pub type Bean = (dyn Any + Send + Sync);
pub type Boolean = bool;

// ----------------------------------------------------------------

pub enum Booleans {
    TRUE(Boolean),
    FALSE(Boolean),
}

// ----------------------------------------------------------------

impl Booleans {
    pub fn value(&self) -> Boolean {
        match self {
            Booleans::TRUE(value) => *value,
            Booleans::FALSE(value) => *value,
        }
    }

    pub fn value_of(&self, value: Boolean) -> Booleans {
        if value {
            return Booleans::TRUE(value);
        }

        return Booleans::FALSE(value);
    }
}
