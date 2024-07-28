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

// environment

// ----------------------------------------------------------------

use std::env;

use omiga_core::collection::table::{Table, Value};
use omiga_core::constants::SIGMA_CORE_PROFILE_ACTIVES_DEFAULT;

use crate::core::error::ConfigError;
use crate::reader::ConfigReader;

pub mod standard;

// ----------------------------------------------------------------

pub trait Environment {
    fn set(&mut self, key: &str, value: Value) -> Result<(), ConfigError>;
    fn get(&self, key: &str) -> Result<&Value, ConfigError>;

    fn try_acquire(&self, suffix: &str) -> Option<&dyn ConfigReader>;
    fn try_acquires(&self) -> Vec<&dyn ConfigReader>;
}

// ----------------------------------------------------------------

pub trait DynamicEnvironment: Environment {
    fn set_t<T>(&mut self, k: &str, v: T) -> Result<(), ConfigError>
    where
        T: Into<Value>,
    {
        self.set(k, v.into())
    }
}

// ----------------------------------------------------------------

pub fn try_load_env_variables() -> Table {
    let vars: Vec<(String, String)> = env::vars().collect();
    let mut table = Table::new();

    for (key, value) in vars {
        table.insert(key, Value::String(value));
    }

    table
}

pub fn is_not_default_profile(profile: &str) -> bool {
    !is_default_profile(profile)
}

pub fn is_default_profile(profile: &str) -> bool {
    profile == SIGMA_CORE_PROFILE_ACTIVES_DEFAULT
}
