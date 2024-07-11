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

// env/standard

// ----------------------------------------------------------------

use std::collections::HashMap;

use omigacore::constants::DOT;

use crate::core::{
    domain::{Table, Value},
    error::ConfigError,
    table::merge_tables,
};
use crate::env::{try_load_env_variables, DynamicEnvironment, Environment};
use crate::reader::{
    registry::{ConfigReaderRegistry, ReaderRegistry},
    toml::TomlConfigReader,
    ConfigReader,
};

// ----------------------------------------------------------------

pub struct StandardEnvironment {
    ctx: Table,
    registry: Option<Box<dyn ReaderRegistry>>,
}

impl StandardEnvironment {
    #[cfg(not(feature = "tomls"))]
    pub fn new() -> Self {
        let env_table = try_load_env_variables();
        Self::mixed(
            Some(env_table),
            Some(Box::<ConfigReaderRegistry>::default()),
        )
    }

    #[cfg(feature = "tomls")]
    pub fn new() -> Self {
        let env_table = try_load_env_variables();
        let mut configurer = Self::mixed(
            Some(env_table),
            Some(Box::<ConfigReaderRegistry>::default()),
        );

        configurer.register_toml_reader();
        configurer
    }

    // ----------------------------------------------------------------

    pub fn mixed(table_opt: Option<Table>, registry: Option<Box<dyn ReaderRegistry>>) -> Self {
        if let Some(table) = table_opt {
            return Self {
                ctx: table,
                registry,
            };
        }

        let env_table = try_load_env_variables();
        Self {
            ctx: env_table,
            registry,
        }
    }

    pub fn mixed_with_env_variables(
        table_opt: Option<Table>,
        registry: Option<Box<dyn ReaderRegistry>>,
    ) -> Self {
        if let Some(table) = table_opt {
            let env_table = try_load_env_variables();
            let merged_table = merge_tables(table, env_table);
            return Self {
                ctx: merged_table,
                registry,
            };
        }

        let env_table = try_load_env_variables();
        Self {
            ctx: env_table,
            registry,
        }
    }

    // ----------------------------------------------------------------

    #[cfg(not(feature = "tomls"))]
    pub fn table(table: Table) -> Self {
        Self::mixed_with_env_variables(Some(table), Some(Box::<ConfigReaderRegistry>::default()))
    }

    #[cfg(feature = "tomls")]
    pub fn table(table: Table) -> Self {
        let mut configurer = Self::mixed_with_env_variables(
            Some(table),
            Some(Box::<ConfigReaderRegistry>::default()),
        );

        configurer.register_toml_reader();
        configurer
    }

    // ----------------------------------------------------------------

    pub fn register_table_with_env_variables(&mut self, table: Table) {
        let env_table = try_load_env_variables();
        let merged_table = merge_tables(table, env_table);

        self.ctx = merged_table;
    }

    pub fn merge_table(&mut self, table: Table) {
        self.ctx = merge_tables(self.ctx.clone(), table)
    }

    // ---------------------------------------------------------------- private

    #[cfg(feature = "tomls")]
    fn register_toml_reader(&mut self) {
        if let Some(ref mut registry) = self.registry {
            registry.register(Box::<TomlConfigReader>::default())
        }
    }
}

// ----------------------------------------------------------------

impl StandardEnvironment {
    fn set_nested_recursive(
        node_ref: &mut Table,
        keys: Vec<&str>,
        value: Value,
    ) -> Result<(), ConfigError> {
        if let Some(sentinel) = keys.first() {
            let key = (*sentinel).to_string();

            if keys.len() > 1 {
                let nested = node_ref
                    .entry(key.clone())
                    .or_insert(Value::Nested(HashMap::new()));
                return if let Value::Nested(nested_ref) = nested {
                    Self::set_nested_recursive(nested_ref, keys[1..].to_vec(), value)
                } else {
                    Err(ConfigError::NonNested)
                };
            }

            node_ref.insert(key, value);
        }

        Ok(())
    }
}

// ----------------------------------------------------------------

impl StandardEnvironment {
    fn set_nested(&mut self, keys: Vec<&str>, value: Value) -> Result<(), ConfigError> {
        if keys.is_empty() {
            return Err(ConfigError::EmptyKey);
        }

        Self::set_nested_recursive(&mut self.ctx, keys, value)?;

        Ok(())
    }

    fn get_nested(&self, keys: Vec<&str>) -> Result<&Value, ConfigError> {
        if keys.is_empty() {
            return Err(ConfigError::EmptyKey);
        }

        let mut node_ref = &self.ctx;

        for (index, sentinel) in keys.iter().enumerate() {
            let key = (*sentinel).to_string();
            match node_ref.get(&key) {
                Some(next_node) => {
                    if index == keys.len() - 1 {
                        return Ok(next_node);
                    }

                    match next_node {
                        Value::Nested(nested) => {
                            node_ref = nested;
                        }
                        _ => return Err(ConfigError::NonNested),
                    }
                }
                None => return Err(ConfigError::NotFound),
            }
        }

        Err(ConfigError::NotFound)
    }
}

// ----------------------------------------------------------------

impl Environment for StandardEnvironment {
    fn set(&mut self, key: &str, value: Value) -> Result<(), ConfigError> {
        if key.is_empty() {
            return Err(ConfigError::EmptyKey);
        }

        let keys: Vec<&str> = key.split(DOT).collect();
        self.set_nested(keys, value)
    }

    fn get(&self, key: &str) -> Result<&Value, ConfigError> {
        let keys: Vec<&str> = key.split(DOT).collect();
        self.get_nested(keys)
    }

    fn try_acquire(&self, name: &str) -> Option<&dyn ConfigReader> {
        if let Some(ref registry) = self.registry {
            registry.try_acquire(name)
        } else {
            None
        }
    }

    fn try_acquires(&self) -> Vec<&dyn ConfigReader> {
        if let Some(ref registry) = self.registry {
            return registry.try_acquires();
        }

        Vec::new()
    }
}

// ----------------------------------------------------------------

impl DynamicEnvironment for StandardEnvironment {}

// ----------------------------------------------------------------

impl Default for StandardEnvironment {
    fn default() -> Self {
        Self::new()
    }
}
