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

// reader

// ----------------------------------------------------------------

use std::fs;
use std::path::PathBuf;

use crate::core::domain::Table;
use crate::core::error::ReadError;

pub mod registry;

#[cfg(feature = "tomls")]
pub mod toml;

// ----------------------------------------------------------------

pub trait ConfigReader {
    fn name(&self) -> String;
    fn suffix(&self) -> String;
    fn supports(&self, suffix: &str) -> bool;

    fn read_from_str(&self, data: &str) -> Result<Table, ReadError>;

    fn read_from_path(&self, path: &str) -> Result<Table, ReadError> {
        let canon = PathBuf::from(path)
            .canonicalize()
            .map_err(|_| ReadError::InvalidPath(path.to_string()))?;
        let content =
            fs::read_to_string(canon).map_err(|_| ReadError::ReadFailed(path.to_string()))?;
        self.read_from_str(&content)
    }
}
