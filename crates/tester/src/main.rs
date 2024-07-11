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

// main

// ----------------------------------------------------------------

use tester::kv;

// ----------------------------------------------------------------

/// command-line k/v args
///
/// # Args Examples
///
/// `$ cargo run -- start --omiga.server.port=9320`
///
/// `$ cargo run -- start --omiga.application.name=helloomiga`
///
/// `$ cargo run -- start --omiga.server.port=9320 --omiga.application.name=helloomiga`
///
/// priority: 1: remote server(Unsupported now.) > 2.command-line args > 3.environment variables > 4.config files
fn main() {
    try_parse_command_line_kv_args()
}

fn try_parse_command_line_kv_args() {
    kv::try_parse_command_line_kv_args()
}
