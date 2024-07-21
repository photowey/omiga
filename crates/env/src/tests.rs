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

use crate::env;

// ----------------------------------------------------------------

#[cfg(test)]
#[cfg(unix)]
mod env_unix_tests;

#[cfg(test)]
#[cfg(windows)]
mod env_windows_test;

#[cfg(test)]
mod converter_tests;

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
