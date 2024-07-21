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

// factory

// ----------------------------------------------------------------

use std::any::Any;
use std::sync::Arc;

use crate::aware::Aware;
use crate::error::BeanError;

// ----------------------------------------------------------------

pub trait BeanFactory {
    fn register<T: 'static + Any + Send + Sync + Clone>(&self, name: &str, component: T);
    fn get<T: 'static + Any + Send + Sync + Clone>(&self, name: &str) -> Result<Arc<T>, BeanError>;
}

// ----------------------------------------------------------------

pub trait BeanFactoryAware: Aware {}
