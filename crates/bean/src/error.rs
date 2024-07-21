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

// ----------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub enum BeanError {
    CircularDependency(String),
    NotFound(String),
    CastFailed(String),
}

impl fmt::Display for BeanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BeanError::CircularDependency(message) => {
                write!(f, "Omiga: circular dependency error, message:[{}]", message)
            }
            BeanError::NotFound(message) => {
                write!(f, "Omiga: bean not found error, message:[{}]", message)
            }
            BeanError::CastFailed(message) => {
                write!(
                    f,
                    "Omiga: component cast to `Bean` error, message:[{}]",
                    message
                )
            }
        }
    }
}

impl Error for BeanError {}
