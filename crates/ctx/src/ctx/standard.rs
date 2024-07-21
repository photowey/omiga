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

#![allow(dead_code)]

// standard

// ----------------------------------------------------------------

use std::any::Any;
use std::sync::{Arc, Mutex};

use dashmap::DashMap;

use omigabean::bean::{Bean, Boolean};
use omigabean::error::BeanError;
use omigabean::factory::BeanFactory;

use crate::ctx::ApplicationContext;

// ----------------------------------------------------------------

pub struct StandardApplicationContext {
    ctx: DashMap<String, Arc<Mutex<Bean>>>,
    initializing: DashMap<String, Boolean>,
}

// ----------------------------------------------------------------

impl StandardApplicationContext {
    pub fn new() -> Self {
        Self {
            ctx: DashMap::new(),
            initializing: DashMap::new(),
        }
    }
}

// ----------------------------------------------------------------

impl ApplicationContext for StandardApplicationContext {}

impl BeanFactory for StandardApplicationContext {
    fn register<T: 'static + Any + Send + Sync + Clone>(&self, name: &str, bean: T) {
        self.ctx
            .insert(name.to_string(), Arc::new(Mutex::new(bean)));
    }

    fn get<T: 'static + Any + Send + Sync + Clone>(&self, name: &str) -> Result<Arc<T>, BeanError> {
        if self.initializing.contains_key(name) {
            return Err(BeanError::CircularDependency(name.to_string()));
        }

        if let Some(bean) = self.ctx.get(name) {
            let lock = bean
                .lock()
                .map_err(|_| BeanError::CastFailed(name.to_string()))?;
            let downcasted = lock
                .downcast_ref::<T>()
                .ok_or_else(|| BeanError::CastFailed(name.to_string()))?;

            return Ok(Arc::new(downcasted.clone()));
        }

        Err(BeanError::NotFound(name.to_string()))
    }
}
