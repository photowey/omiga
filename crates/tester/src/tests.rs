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

// tests

// ----------------------------------------------------------------

use std::collections::HashSet;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use std::{env, fs};

// ----------------------------------------------------------------

#[derive(Debug)]
struct Config {
    search_paths: Vec<String>,
    configs: Vec<String>,
    formats: Vec<String>,
    profiles: Vec<String>,
    paths: HashSet<String>,
}

impl Config {
    fn new(
        search_paths: Vec<String>,
        configs: Vec<String>,
        formats: Vec<String>,
        profiles: Vec<String>,
        paths: HashSet<String>,
    ) -> Self {
        Config {
            search_paths,
            configs,
            formats,
            profiles,
            paths,
        }
    }

    fn build(&mut self) {
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
                    Err(_) => absolute_path.to_string_lossy().to_string(),
                }
            })
            .collect();
    }
}
// ----------------------------------------------------------------

#[test]
#[cfg(windows)]
fn test_files_os_windows() {
    let search_paths = vec![
        String::from("."),
        String::from("C:\\rust\\dev\\configs"),
        String::from("C:\\rust\\data\\configs"),
    ];
    let configs = vec![String::from("application"), String::from("omiga")];
    let formats = vec![String::from("toml"), String::from("yml")];
    let profiles = vec![String::from("dev"), String::from("test")];
    let initial_paths = HashSet::new();

    let mut config = Config::new(search_paths, configs, formats, profiles, initial_paths);
    config.build();

    for path in &config.paths {
        println!("{}", path);
    }
}

#[test]
#[cfg(unix)]
fn test_files_os_unix() {
    let search_paths = vec![
        String::from("."),
        String::from("/opt/configs"),
        String::from("/data/configs"),
    ];
    let configs = vec![String::from("application"), String::from("omiga")];
    let formats = vec![String::from("toml"), String::from("yml")];
    let profiles = vec![String::from("dev"), String::from("test")];
    let initial_paths = HashSet::new();

    let mut config = Config::new(search_paths, configs, formats, profiles, initial_paths);
    config.build();

    for path in &config.paths {
        println!("{}", path);
    }
}
