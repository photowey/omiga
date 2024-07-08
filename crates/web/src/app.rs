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

// omigaweb/app

// ----------------------------------------------------------------

use omigacore::constants::SIGMA_CORE_PROFILE_ACTIVES;

// default

// ----------------------------------------------------------------

pub trait Application {
    fn run(&self);
}

// ----------------------------------------------------------------

pub struct OmigaApplication {
    configs: Vec<String>,
    profiles: Vec<String>,
}

impl OmigaApplication {
    pub fn builder() -> OmigaApplicationBuilder {
        OmigaApplicationBuilder::default()
    }

    // ----------------------------------------------------------------

    pub fn new(configs: Vec<String>, profiles: Vec<String>) -> Self {
        Self { configs, profiles }
    }

    // ----------------------------------------------------------------

    pub fn profiles_active(&self) -> String {
        self.profiles.join(",")
    }

    pub fn profiles_active_array(&self) -> Vec<String> {
        self.profiles.clone()
    }

    // ----------------------------------------------------------------

    pub fn configs(&self) -> String {
        self.configs.join(",")
    }

    pub fn configs_array(&self) -> Vec<String> {
        self.configs.clone()
    }

    // ----------------------------------------------------------------

    pub fn is_default_profile(&self) -> bool {
        self.profiles
            .contains(&SIGMA_CORE_PROFILE_ACTIVES.to_string())
    }
}

impl Application for OmigaApplication {
    fn run(&self) {
        println!("omiga: Hello, Omiga!");
    }
}

// ----------------------------------------------------------------

pub struct OmigaApplicationBuilder {
    configs: Vec<String>,
    profiles: Vec<String>,
}

impl OmigaApplicationBuilder {
    pub fn new() -> Self {
        Self {
            configs: Vec::new(),
            profiles: vec![SIGMA_CORE_PROFILE_ACTIVES.to_string()],
        }
    }

    // ----------------------------------------------------------------

    pub fn config(mut self, config: String) -> Self {
        self.configs.push(config);

        self
    }

    pub fn configs(mut self, configs: Vec<String>) -> Self {
        self.configs.extend(configs);

        self
    }

    // ----------------------------------------------------------------

    pub fn profile(mut self, profile: String) -> Self {
        self.profiles.retain(|p| Self::is_not_default_profile(p));
        self.profiles.push(profile);

        self
    }

    pub fn profiles(mut self, profiles: Vec<String>) -> Self {
        self.profiles.extend(profiles);

        self
    }

    // ----------------------------------------------------------------

    fn is_not_default_profile(profile: &str) -> bool {
        profile != SIGMA_CORE_PROFILE_ACTIVES
    }

    // ----------------------------------------------------------------

    pub fn build(self) -> OmigaApplication {
        OmigaApplication::new(self.configs, self.profiles)
    }
}

impl Default for OmigaApplicationBuilder {
    fn default() -> Self {
        Self::new()
    }
}
