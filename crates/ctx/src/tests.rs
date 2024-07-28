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

use omiga_bean::factory::BeanFactory;

use crate::ctx::standard::StandardApplicationContext;
use crate::ctx::ApplicationContext;

// ----------------------------------------------------------------

#[derive(Debug, Clone)]
struct HelloService {
    value: i32,
}

impl HelloService {
    pub fn new(value: i32) -> Self {
        Self { value }
    }

    pub fn say_hello(&self) -> i32 {
        self.value
    }
}

// ----------------------------------------------------------------

#[test]
fn test_ctx_register() {
    let ctx = StandardApplicationContext::new();
    ctx.register("hello_service", HelloService::new(10086));
}

#[test]
fn test_ctx_get() {
    let ctx = StandardApplicationContext::new();
    ctx.register("hello_service", HelloService::new(10086));
    let hello_service = ctx.get::<HelloService>("hello_service").unwrap();
    assert_eq!(10086, hello_service.say_hello());
}

// ----------------------------------------------------------------

#[test]
fn test_ctx_register_initializing() {
    let ctx = StandardApplicationContext::new();

    let rvt = ctx.register_initializing("hello_service");
    assert!(rvt.is_none());

    let rvt_1 = ctx.predicate_initializing("hello_service");
    assert!(rvt_1.is_some());
}

#[test]
fn test_ctx_register_initialized() {
    let ctx = StandardApplicationContext::new();

    let rvt = ctx.register_initializing("hello_service");
    assert!(rvt.is_none());

    let rvt2 = ctx.register_initialized("hello_service");
    assert!(rvt2.is_some());

    let rvt_1 = ctx.predicate_initializing("hello_service");
    assert!(rvt_1.is_none());
}

#[test]
fn test_ctx_predicate_initializing() {
    let ctx = StandardApplicationContext::new();

    let rvt_1 = ctx.register_initializing("hello_service");
    assert!(rvt_1.is_none());

    let rvt_1_1 = ctx.predicate_initializing("hello_service");
    assert!(rvt_1_1.is_some());

    let rvt_2 = ctx.register_initialized("hello_service");
    assert!(rvt_2.is_some());

    let rvt_2_1 = ctx.predicate_initializing("hello_service");
    assert!(rvt_2_1.is_none());
}

#[test]
fn test_ctx_register_bean_with_initialized() {
    let ctx = StandardApplicationContext::new();

    let rvt_1 = ctx.register_initializing("hello_service");
    assert!(rvt_1.is_none());

    let rvt_1_1 = ctx.predicate_initializing("hello_service");
    assert!(rvt_1_1.is_some());

    ctx.register("hello_service", HelloService::new(10086));

    let rvt_2 = ctx.predicate_initializing("hello_service");
    assert!(rvt_2.is_none());
}

#[test]
fn test_ctx_predicate_initialized() {
    let ctx = StandardApplicationContext::new();

    let rvt_1 = ctx.register_initializing("hello_service");
    assert!(rvt_1.is_none());

    let rvt_1_1 = ctx.predicate_initializing("hello_service");
    assert!(rvt_1_1.is_some());

    ctx.register("hello_service", HelloService::new(10086));

    let rvt_2 = ctx.predicate_initializing("hello_service");
    assert!(rvt_2.is_none());

    let rvt_3 = ctx.predicate_initialized("hello_service");
    assert!(rvt_3.is_some());
}
