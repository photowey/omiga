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

use std::collections::{HashMap, HashSet};
use std::io::ErrorKind;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use std::{env, fs};

use omigacore::collection::table::{Table, Value};
use omigacore::constants::{
    DOT, SIGMA_CONFIG_READER_TOML_FORMAT, SIGMA_CORE_CONFIG_FILE_FORMAT_DEFAULT,
    SIGMA_CORE_CONFIG_FILE_NAME_APPLICATION_DEFAULT, SIGMA_CORE_CONFIG_FILE_NAME_DEFAULT,
    SIGMA_CORE_PROFILE_ACTIVES_DEFAULT,
};

use crate::core::error::FileError;
use crate::core::{error::ConfigError, merger::merge_tables};
use crate::env::{is_default_profile, DynamicEnvironment, Environment};
use crate::reader::{
    registry::{ConfigReaderRegistry, ReaderRegistry},
    toml::TomlConfigReader,
    ConfigReader,
};

// ----------------------------------------------------------------

pub struct StandardEnvironment {
    ctx: Table,
    registry: Box<dyn ReaderRegistry>,
}

// ----------------------------------------------------------------

impl StandardEnvironment {
    pub fn builder() -> StandardEnvironmentBuilder {
        StandardEnvironmentBuilder::default()
    }
}

// ----------------------------------------------------------------

impl StandardEnvironment {
    #[cfg(not(feature = "tomls"))]
    pub fn new(table_opt: Option<Table>, registry: Box<dyn ReaderRegistry>) -> Self {
        let configure = Self::new_opt(table_opt, registry);

        configure
    }

    #[cfg(feature = "tomls")]
    pub fn new(table_opt: Option<Table>, registry: Box<dyn ReaderRegistry>) -> Self {
        let mut configure = Self::new_opt(table_opt, registry);
        configure.register_toml_reader();

        configure
    }

    fn new_opt(table_opt: Option<Table>, registry: Box<dyn ReaderRegistry>) -> Self {
        let mut merged_table = Table::new();

        //let env_table = try_load_env_variables();
        let env_table = Table::new();
        merged_table = merge_tables(merged_table, env_table);

        if let Some(table) = table_opt {
            merged_table = merge_tables(merged_table, table);
        }

        Self::mixed(Some(merged_table), registry)
    }

    // ----------------------------------------------------------------

    pub fn table(mut self, table: Table) -> Self {
        self.merge_table(table);

        self
    }

    // ----------------------------------------------------------------

    pub fn merge_table(&mut self, table: Table) {
        self.ctx = merge_tables(self.ctx.clone(), table)
    }

    // ---------------------------------------------------------------- private

    fn mixed(table_opt: Option<Table>, registry: Box<dyn ReaderRegistry>) -> Self {
        if let Some(table) = table_opt {
            return Self {
                ctx: table,
                registry,
            };
        }

        Self {
            ctx: Table::new(),
            registry,
        }
    }

    #[cfg(feature = "tomls")]
    fn register_toml_reader(&mut self) {
        self.registry.register(Box::<TomlConfigReader>::default())
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
        self.registry.try_acquire(name)
    }

    fn try_acquires(&self) -> Vec<&dyn ConfigReader> {
        self.registry.try_acquires()
    }
}

// ----------------------------------------------------------------

impl DynamicEnvironment for StandardEnvironment {}

// ----------------------------------------------------------------

pub struct StandardEnvironmentBuilder {
    /// Init config table.
    table: Option<Table>,
    /// Config reader registry.
    registry: Option<Box<dyn ReaderRegistry>>,
    /// Config file paths.
    ///
    /// * /opt/app/configs/omiga.yml
    /// * /opt/app/configs/application.yml
    /// * omiga.yml -> ./omiga.yml
    /// * application.yml -> ./application.yml
    /// * ...
    paths: HashSet<String>,
    /// Config file name.
    ///
    /// * omiga
    /// * application
    /// * ...
    configs: HashSet<String>,
    /// Config file profiles active.
    ///
    /// * dev
    /// * test
    /// * stage
    /// * prod
    /// * ...
    profiles: HashSet<String>,
    /// Config file format.
    ///
    /// * toml
    /// * yml | yaml (Unsupported now)
    /// * json (Unsupported now)
    /// * properties (Unsupported now)
    /// * ini (Unsupported now)
    /// * ...
    formats: HashSet<String>,
    /// Config file search paths.
    /// * .
    /// * ./configs
    /// * ./resources
    /// * ...
    search_paths: HashSet<String>,
}

impl StandardEnvironmentBuilder {
    pub fn new() -> Self {
        let mut default_profiles = HashSet::new();
        default_profiles.insert(SIGMA_CORE_PROFILE_ACTIVES_DEFAULT.to_string());

        Self {
            table: None,
            registry: None,
            paths: HashSet::new(),
            configs: HashSet::new(),
            profiles: default_profiles,
            formats: HashSet::new(),
            search_paths: HashSet::new(),
        }
    }

    pub fn with_table(mut self, table: Table) -> Self {
        self.table = Some(table);

        self
    }

    #[cfg(not(feature = "tomls"))]
    pub fn with_registry(mut self, registry: Box<dyn ReaderRegistry>) -> Self {
        self.registry = Some(registry);

        self
    }

    #[cfg(feature = "tomls")]
    pub fn with_registry(mut self, registry: Box<dyn ReaderRegistry>) -> Self {
        self.registry = Some(registry);

        match self
            .registry
            .as_ref()
            .unwrap()
            .try_acquire(SIGMA_CONFIG_READER_TOML_FORMAT)
        {
            Some(_) => {}
            None => {
                self.registry
                    .as_mut()
                    .unwrap()
                    .register(Box::<TomlConfigReader>::default());
            }
        }

        self
    }

    pub fn with_reader(mut self, reader: Box<dyn ConfigReader>) -> Self {
        match self
            .registry
            .as_ref()
            .unwrap()
            .try_acquire(reader.as_ref().suffix().as_str())
        {
            Some(_) => {}
            None => self.registry.as_mut().unwrap().register(reader),
        }

        self
    }

    pub fn with_path(mut self, path: String) -> Self {
        self.paths.insert(path);

        self
    }

    pub fn with_paths(mut self, paths: Vec<String>) -> Self {
        self.paths.extend(paths);

        self
    }

    pub fn with_config(mut self, config: String) -> Self {
        self.configs.insert(config);

        self
    }

    pub fn with_configs(mut self, configs: Vec<String>) -> Self {
        self.configs.extend(configs);

        self
    }

    pub fn with_profile(mut self, profile: String) -> Self {
        self.profiles.insert(profile);

        self
    }

    pub fn with_profiles(mut self, profiles: Vec<String>) -> Self {
        self.profiles.extend(profiles);

        self
    }

    pub fn with_format(mut self, format: String) -> Self {
        self.formats.insert(format);

        self
    }

    pub fn with_formats(mut self, formats: Vec<String>) -> Self {
        self.formats.extend(formats);

        self
    }

    pub fn with_search_path(mut self, search_path: String) -> Self {
        self.search_paths.insert(search_path);

        self
    }

    pub fn with_search_paths(mut self, search_paths: Vec<String>) -> Self {
        self.search_paths.extend(search_paths);

        self
    }
}

impl StandardEnvironmentBuilder {
    pub fn build(&mut self) -> Result<StandardEnvironment, FileError> {
        self.try_populate_defaults();
        self.try_merge_paths();

        self.on_build()
    }

    // ----------------------------------------------------------------

    fn try_populate_defaults(&mut self) {
        if self.configs.is_empty() {
            self.configs
                .insert(SIGMA_CORE_CONFIG_FILE_NAME_DEFAULT.to_string());
            self.configs
                .insert(SIGMA_CORE_CONFIG_FILE_NAME_APPLICATION_DEFAULT.to_string());
        }

        if self.formats.is_empty() {
            self.formats
                .insert(SIGMA_CORE_CONFIG_FILE_FORMAT_DEFAULT.to_string());
        }

        match self.registry {
            Some(_) => {}
            None => {
                // By default.
                let mut registry = ConfigReaderRegistry::default();
                registry.register(Box::<TomlConfigReader>::default());

                self.registry = Some(Box::new(registry));
            }
        }
    }

    // ----------------------------------------------------------------

    fn try_merge_paths(&mut self) {
        let current_dir = env::current_dir().unwrap();
        let separator = MAIN_SEPARATOR.to_string();

        let mut new_paths = HashSet::new();

        for search_path in &self.search_paths {
            let base_path = if Path::new(search_path).is_absolute() {
                PathBuf::from(search_path)
            } else {
                current_dir.join(search_path)
            };

            for config in &self.configs {
                for format in &self.formats {
                    for profile in &self.profiles {
                        if is_default_profile(profile) {
                            continue;
                        }
                        let path = format!(
                            "{}{}{}-{}.{}",
                            base_path.to_string_lossy(),
                            separator,
                            config,
                            profile,
                            format
                        );
                        new_paths.insert(path);
                    }
                    let path = format!(
                        "{}{}{}.{}",
                        base_path.to_string_lossy(),
                        separator,
                        config,
                        format
                    );
                    new_paths.insert(path);
                }
            }
        }

        self.paths.extend(new_paths);
        self.paths = self
            .paths
            .iter()
            .map(|path| {
                let absolute_path = if Path::new(path).is_absolute() {
                    PathBuf::from(path)
                } else {
                    current_dir.join(path)
                };
                match fs::canonicalize(&absolute_path) {
                    Ok(clean_path) => clean_path.to_string_lossy().to_string(),
                    Err(err) => {
                        if err.kind() != ErrorKind::NotFound {
                            // log.warn
                            return ErrorKind::NotFound.to_string();
                        }

                        // log.error
                        ErrorKind::InvalidInput.to_string()
                    }
                }
            })
            .filter(|x| {
                !(x == &ErrorKind::NotFound.to_string()
                    || x == &ErrorKind::InvalidInput.to_string())
            })
            .collect();
    }

    // ----------------------------------------------------------------

    fn take_registry(&mut self) -> Option<Box<dyn ReaderRegistry>> {
        self.registry.take()
    }

    // ----------------------------------------------------------------

    fn on_build(&mut self) -> Result<StandardEnvironment, FileError> {
        let mut merged_table = Table::new();
        for path in self.paths.iter() {
            // e.g.: /opt/app/configs/omiga.toml
            let config_file_path = Path::new(&path);
            if let Some(extension) = config_file_path
                .to_str()
                .and_then(|name| Path::new(name).extension())
            {
                let format = extension.to_string_lossy();

                if let Some(reader) = self.registry.as_ref().unwrap().try_acquire(format.as_ref()) {
                    match self.try_read_config_file(config_file_path, reader) {
                        Ok(table_ok) => {
                            merged_table = merge_tables(merged_table, table_ok);
                        }
                        Err(failed) => {
                            return Err(failed);
                        }
                    }
                } else {
                    // /opt/app/configs/omiga.${UnknownSuffix}
                    return Err(FileError::ReaderNotFound(format.as_ref().to_string()));
                }
            } else {
                // /opt/app/configs/omiga
                return Err(FileError::InvalidPath(format!(
                    "{}",
                    config_file_path.to_string_lossy()
                )));
            }
        }

        if let Some(registry) = self.take_registry() {
            return Ok(StandardEnvironment::new(Some(merged_table), registry));
        }

        Ok(StandardEnvironment::new(
            Some(merged_table),
            Box::new(ConfigReaderRegistry::default()),
        ))
    }

    // ----------------------------------------------------------------

    fn try_read_config_file(
        &self,
        file_path: &Path,
        reader: &dyn ConfigReader,
    ) -> Result<Table, FileError> {
        match reader.read_from_path(file_path.to_str().unwrap()) {
            Ok(table_ok) => {
                return Ok(table_ok);
            }
            Err(failed) => {
                match failed {
                    FileError::FileNotFound(_) => {
                        // log.warn
                        Ok(Table::new())
                    }
                    _ => {
                        return Err(failed);
                    }
                }
            }
        }
    }
}

// ----------------------------------------------------------------

impl Default for StandardEnvironmentBuilder {
    fn default() -> Self {
        StandardEnvironmentBuilder::new()
    }
}
