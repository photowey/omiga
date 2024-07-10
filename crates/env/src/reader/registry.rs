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

// reader/registry

// ------------------------------------------------------------

use std::collections::HashMap;

use crate::reader::ConfigReader;

// ------------------------------------------------------------

#[allow(dead_code)]
pub trait ReaderRegistry {
    fn register(&mut self, reader: Box<dyn ConfigReader>);
    fn try_acquire(&self, suffix: &str) -> Option<&dyn ConfigReader>;
    fn try_acquires(&self) -> Vec<&dyn ConfigReader>;
}

// ------------------------------------------------------------

#[allow(dead_code)]
pub struct ConfigReaderRegistry {
    readers: HashMap</*suffix*/ String, Box<dyn ConfigReader>>,
}

impl ConfigReaderRegistry {
    pub fn new() -> Self {
        Self {
            readers: HashMap::new(),
        }
    }
}

impl Default for ConfigReaderRegistry {
    fn default() -> Self {
        ConfigReaderRegistry::new()
    }
}

// ----------------------------------------------------------------

impl ReaderRegistry for ConfigReaderRegistry {
    fn register(&mut self, reader: Box<dyn ConfigReader>) {
        self.readers.insert(reader.suffix(), reader);
    }

    fn try_acquire(&self, suffix: &str) -> Option<&dyn ConfigReader> {
        self.readers.get(suffix).map(|r| r.as_ref())
    }

    fn try_acquires(&self) -> Vec<&dyn ConfigReader> {
        self.readers
            .values()
            .map(|r| r.as_ref() as &dyn ConfigReader)
            .collect()
    }
}
