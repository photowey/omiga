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

// error

// ----------------------------------------------------------------

use std::error::Error;
use std::fmt;

#[allow(dead_code)] // tmp
#[derive(Debug, PartialEq)]
pub enum OmigaError {
    Runtime(String),
    IO(String),
    Database(String),
    Business(String),
    Unknown(String),
}

impl fmt::Display for OmigaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OmigaError::Runtime(message) => {
                write!(f, "Omiga: runtime error, message:[{}]", message)
            }
            OmigaError::IO(message) => write!(f, "Omiga: I/O error, message:[{}]", message),
            OmigaError::Database(message) => {
                write!(f, "Omiga: database error, message:[{}]", message)
            }
            OmigaError::Business(message) => {
                write!(f, "Omiga: business error, message:[{}]", message)
            }
            OmigaError::Unknown(message) => {
                write!(f, "Omiga: unknown error, message:[{}]", message)
            }
        }
    }
}

impl Error for OmigaError {}
