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

// tests/env_windows_test

#![allow(unused_imports)]

// ----------------------------------------------------------------

use omigacore::collection::table::{Table, Value};

use crate::env::standard::StandardEnvironment;
use crate::env::Environment;
use crate::reader::registry::ConfigReaderRegistry;
use crate::reader::toml::TomlConfigReader;

// ----------------------------------------------------------------

#[test]
fn test_hold_on_imports() {
    let rvt = StandardEnvironment::builder()
        .with_table(Table::new())
        .with_registry(Box::new(ConfigReaderRegistry::default()))
        .with_reader(Box::new(TomlConfigReader::default()))
        .with_config("omiga".to_string())
        .with_profile("dev".to_string())
        .with_format("toml".to_string())
        .build();

    let mut environment = rvt.unwrap();
    environment
        .set("io.github.photowey", Value::String("omiga".to_string()))
        .expect("Set failed");

    assert_eq!(
        environment.get("io.github.photowey"),
        Ok(&Value::String("omiga".to_string()))
    );
}

#[test]
#[cfg(windows)]
fn test_standard_environment_builder_os_windows() {
    let rvt = StandardEnvironment::builder()
        .with_table(Table::new())
        .with_registry(Box::new(ConfigReaderRegistry::default()))
        .with_reader(Box::new(TomlConfigReader::default()))
        .with_path(".\\testdata".to_string())
        .with_paths(vec![".\\configs".to_string()])
        .with_config("omiga".to_string())
        .with_configs(vec!["application".to_string()])
        .with_profile("dev".to_string())
        .with_profiles(vec!["test".to_string()])
        .with_format("toml".to_string())
        //.with_formats(vec!["yml".to_string(), "yaml".to_string()]) // ReaderNotFound
        .with_formats(vec!["toml".to_string()])
        .with_search_path("C:\\rust\\app\\configs".to_string())
        .with_search_paths(vec!["C:\\rust\\data\\configs".to_string()])
        .build();

    assert!(rvt.is_ok());
}

// ----------------------------------------------------------------

#[test]
#[cfg(windows)]
fn test_standard_environment_os_windows() {
    let rvt = StandardEnvironment::builder()
        .with_table(Table::new())
        .with_registry(Box::new(ConfigReaderRegistry::default()))
        .with_reader(Box::new(TomlConfigReader::default()))
        .with_path(".\\testdata".to_string())
        .with_config("omiga".to_string())
        .with_profile("dev".to_string())
        .with_format("toml".to_string())
        .with_search_path("C:\\rust\\app\\configs".to_string())
        .build();

    let mut environment = rvt.unwrap();
    environment
        .set("io.github.photowey", Value::String("omiga".to_string()))
        .expect("Set failed");

    assert_eq!(
        environment.get("io.github.photowey"),
        Ok(&Value::String("omiga".to_string()))
    );
}
