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

// tests/env_tests

// ----------------------------------------------------------------

use omigacore::collection::table::Table;

use crate::env::standard::StandardEnvironment;
use crate::reader::registry::ConfigReaderRegistry;
use crate::reader::toml::TomlConfigReader;

// ----------------------------------------------------------------

#[test]
#[cfg(windows)]
fn test_standard_environment_builder() {
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
