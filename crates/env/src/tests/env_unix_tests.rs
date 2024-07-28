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

// tests/env_unix_tests

#![allow(unused_imports)]

// ----------------------------------------------------------------

use std::path::Path;

use omiga_core::collection::table::{Table, Value};

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

    let file_path_relative = Path::new("omiga.toml");
    assert!(file_path_relative.is_relative());
}

#[test]
#[cfg(unix)]
fn test_standard_environment_builder_os_unix() {
    let rvt = StandardEnvironment::builder()
        .with_table(Table::new())
        .with_registry(Box::new(ConfigReaderRegistry::default()))
        .with_reader(Box::new(TomlConfigReader::default()))
        .with_path("./testdata".to_string())
        .with_paths(vec!["./configs".to_string()])
        .with_config("omiga".to_string())
        .with_configs(vec!["application".to_string()])
        .with_profile("dev".to_string())
        .with_profiles(vec!["test".to_string()])
        .with_format("toml".to_string())
        .with_formats(vec!["toml".to_string()])
        .with_formats(vec!["toml".to_string(), "toml".to_string()])
        .with_search_path("/opt/app/configs".to_string())
        .with_search_paths(vec!["/data/configs".to_string()])
        .build();

    assert!(rvt.is_ok());
}

// ----------------------------------------------------------------

#[test]
#[cfg(unix)]
fn test_standard_environment_os_unix() {
    let rvt = StandardEnvironment::builder()
        .with_table(Table::new())
        .with_registry(Box::new(ConfigReaderRegistry::default()))
        .with_reader(Box::new(TomlConfigReader::default()))
        .with_path("./testdata".to_string())
        .with_config("omiga".to_string())
        .with_profile("dev".to_string())
        .with_format("toml".to_string())
        .with_search_path("/opt/app/configs".to_string())
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

// ----------------------------------------------------------------

#[test]
#[cfg(unix)]
fn test_file_path_os_unix() {
    let path = "/opt/ppp/configs/omiga";
    let file_path = Path::new(&path);
    let parent_path = file_path.parent().unwrap();
    let file_stem = file_path.file_stem().unwrap();

    assert_eq!("/opt/ppp/configs", parent_path.to_str().unwrap());
    assert_eq!("omiga", file_stem.to_str().unwrap());
    assert_eq!("omiga", file_stem.to_string_lossy())
}

// ----------------------------------------------------------------

#[test]
#[cfg(unix)]
fn test_file_os_unix() {
    let path = "/opt/ppp/configs/omiga.toml";
    let file_path = Path::new(&path);
    let parent_path = file_path.parent().unwrap();

    assert_eq!("/opt/ppp/configs", parent_path.to_str().unwrap());
    assert_eq!("omiga.toml", file_path.file_name().unwrap());
}

// ----------------------------------------------------------------

#[test]
#[cfg(unix)]
fn test_path_os_unix() {
    let file_path_absolute = Path::new("/opt/ppp/configs/omiga.toml");
    let file_path_relative = Path::new("omiga.toml");

    assert!(file_path_absolute.is_absolute());
    assert!(file_path_relative.is_relative());
}
