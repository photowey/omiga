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

use std::path::Path;

use crate::env;

// ----------------------------------------------------------------

#[cfg(test)]
#[cfg(unix)]
mod env_unix_tests;
#[cfg(test)]
#[cfg(windows)]
mod env_windows_test;

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

#[test]
#[cfg(windows)]
fn test_file_path_os_windows() {
    let path = "C:\\rust\\dev\\configs\\omiga";
    let file_path = Path::new(&path);
    let parent_path = file_path.parent().unwrap();
    let file_stem = file_path.file_stem().unwrap();

    assert_eq!("C:\\rust\\dev\\configs", parent_path.to_str().unwrap());
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

#[test]
#[cfg(windows)]
fn test_file_os_windows() {
    let path = "C:\\rust\\dev\\configs\\omiga.toml";
    let file_path = Path::new(&path);
    let parent_path = file_path.parent().unwrap();

    assert_eq!("C:\\rust\\dev\\configs", parent_path.to_str().unwrap());
    assert_eq!("omiga.toml", file_path.file_name().unwrap());
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

#[test]
#[cfg(windows)]
fn test_path_os_windows() {
    let file_path_absolute = Path::new("C:\\rust\\dev\\configs\\omiga.toml");
    let file_path_relative = Path::new("omiga.toml");

    assert!(file_path_absolute.is_absolute());
    assert!(file_path_relative.is_relative());
}

// ----------------------------------------------------------------

#[test]
fn test_is_default_profile_active() {
    assert!(env::is_default_profile("default"));
    assert!(!env::is_default_profile("dev"))
}

#[test]
fn test_is_not_default_profile_active() {
    assert!(env::is_not_default_profile("dev"));
    assert!(!env::is_not_default_profile("default"))
}
