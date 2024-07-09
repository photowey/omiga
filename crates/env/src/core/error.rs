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

// core/error

// ----------------------------------------------------------------

use std::error::Error;
use std::fmt;

// ----------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub enum ConfigError {
    EmptyKey,
    NonNested,
    NotFound,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigError::EmptyKey => write!(f, "Omiga: key can't be empty"),
            ConfigError::NonNested => {
                write!(
                    f,
                    "Omiga: attempted to set/get a nested value on a non-nested node"
                )
            }
            ConfigError::NotFound => write!(f, "Omiga: not found"),
        }
    }
}

impl Error for ConfigError {}

// ----------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub enum ReadError {
    InvalidPath(String),
    InvalidFile(String),
    ReaderNotFound(String),
    ReadFailed(String),
    IncorrectFormat(String),
    ParseFailed(String, String),
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReadError::InvalidPath(path) => write!(f, "Omiga: invalid path:[{}]", path),
            ReadError::InvalidFile(file) => write!(f, "Omiga: invalid config file type:[{}]", file),
            ReadError::ReaderNotFound(suffix) => {
                write!(f, "Omiga: reader not found, suffix: [{}]", suffix)
            }
            ReadError::ReadFailed(path) => {
                write!(f, "Omiga: failed to read config file, path:[{}]", path)
            }
            ReadError::IncorrectFormat(format) => write!(
                f,
                "Omiga: incorrect [{}] format, missing table data.",
                format
            ),
            ReadError::ParseFailed(format, message) => write!(
                f,
                "Omiga: failed to parse [{}] config file, message: [{}]",
                format, message
            ),
        }
    }
}

impl Error for ReadError {}
