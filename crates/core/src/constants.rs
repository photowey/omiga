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

// constants

// ----------------------------------------------------------------

pub const SIGMA_VERSION: &str = "0.1.0";
pub const SIGMA_CORE_PROFILE_ACTIVES_DEFAULT: &str = "default";

// ----------------------------------------------------------------

// omiga.toml | omiga-dev.toml ...
pub const SIGMA_CORE_CONFIG_FILE_NAME_DEFAULT: &str = "omiga";
pub const SIGMA_CORE_CONFIG_FILE_NAME_APPLICATION_DEFAULT: &str = "application";
// toml* | yml/yaml | json | properties | ini | ...
pub const SIGMA_CORE_CONFIG_FILE_FORMAT_DEFAULT: &str = "toml";
pub const SIGMA_CORE_CONFIG_FILE_SEARCH_PATHS_DEFAULT: &str = ".,configs,resources";

// ----------------------------------------------------------------

pub const SIGMA_CONFIG_READER_TOML: &str = "TOML";
pub const SIGMA_CONFIG_READER_TOML_FORMAT: &str = "toml";

// ----------------------------------------------------------------

pub const DOT: &str = ".";
pub const COMMA: &str = ",";

// ----------------------------------------------------------------

/// 9320: A dream moment for Manchester City's forward `Agüero`.
pub const SIGMA_WEB_SERVER_PORT_DEFAULT: u32 = 9320;

// ----------------------------------------------------------------

pub const ONE_INT: u8 = 1;
pub const ZERO_INT: u8 = 0;

pub const ONE: &str = "1";
pub const ZERO: &str = "0";

pub const TRUE: &str = "true";
pub const FALSE: &str = "false";

// ----------------------------------------------------------------
