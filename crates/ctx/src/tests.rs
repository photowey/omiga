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

use omigabean::factory::BeanFactory;

use crate::ctx::standard::StandardApplicationContext;

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
fn test_ctx() {
    let ctx = StandardApplicationContext::new();
    ctx.register("hello_service", HelloService::new(10086));
    let hello_service = ctx.get::<HelloService>("hello_service").unwrap();
    assert_eq!(10086, hello_service.say_hello());
}
