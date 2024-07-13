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

use omigacore::constants::{COMMA, SIGMA_CORE_PROFILE_ACTIVES_DEFAULT};

use crate::core::kv::Kv;

// ----------------------------------------------------------------

pub trait Application {
    fn run(&self);
}

// ----------------------------------------------------------------

#[allow(dead_code)]
pub struct OmigaApplication {
    /// Config file name.
    ///
    /// * application
    /// * omiga
    /// * ...
    configs: Vec<String>,
    /// Config file profiles active.
    ///
    /// * dev
    /// * test
    /// * stage
    /// * prod
    /// * ...
    profiles: Vec<String>,
    /// Config file format.
    ///
    /// * toml
    /// * yml | yaml (Unsupported now)
    /// * json (Unsupported now)
    /// * properties (Unsupported now)
    /// * ini (Unsupported now)
    /// * ...
    formats: Vec<String>,
    /// Application cmd args for specific configs.
    ///
    /// * /opt/configs/omiga.yml
    /// * /opt/configs/omiga-dev.yml
    /// * /opt/configs/application.yml
    /// * /opt/configs/application-dev.yml
    /// * ...
    paths: Vec<String>,
    /// Search paths.
    /// * .
    /// * ./configs
    /// * ./resources
    /// * ...
    search_paths: Vec<String>,
    /// Application cmd args.
    ///
    /// * --omiga.server.port=9320
    /// * --omiga.application.name=omiga
    /// * ...
    kv: Option<Kv>,
}

impl OmigaApplication {
    pub fn builder() -> OmigaApplicationBuilder {
        OmigaApplicationBuilder::default()
    }

    // omiga start --omiga.server.port=9320
    //               ^~~~ k/v
    pub fn walk(/*tmp*/ _kv_args: Kv) -> OmigaApplicationBuilder {
        panic!("Unsupported now.")
    }

    // ----------------------------------------------------------------

    pub fn new(
        configs: Vec<String>,
        profiles: Vec<String>,
        formats: Vec<String>,
        paths: Vec<String>,
        search_paths: Vec<String>,
        kv: Option<Kv>,
    ) -> Self {
        Self {
            configs,
            profiles,
            formats,
            paths,
            search_paths,
            kv,
        }
    }

    // ----------------------------------------------------------------

    pub fn profiles_active(&self) -> String {
        self.profiles.join(COMMA)
    }

    pub fn profiles_active_array(&self) -> Vec<String> {
        self.profiles.clone()
    }

    // ----------------------------------------------------------------

    pub fn configs(&self) -> String {
        self.configs.join(COMMA)
    }

    pub fn configs_array(&self) -> Vec<String> {
        self.configs.clone()
    }

    // ----------------------------------------------------------------

    pub fn is_default_profile(&self) -> bool {
        self.profiles
            .contains(&SIGMA_CORE_PROFILE_ACTIVES_DEFAULT.to_string())
    }
}

impl Application for OmigaApplication {
    fn run(&self) {
        println!("omiga: Hello, Omiga!");
    }
}

// ----------------------------------------------------------------

pub struct OmigaApplicationBuilder {
    /// Config file name.
    ///
    /// * application
    /// * omiga
    /// * ...
    configs: Vec<String>,
    /// Config file profiles active.
    ///
    /// * dev
    /// * test
    /// * stage
    /// * prod
    /// * ...
    profiles: Vec<String>,
    /// Config file format.
    ///
    /// * toml
    /// * yml | yaml (Unsupported now)
    /// * json (Unsupported now)
    /// * properties (Unsupported now)
    /// * ini (Unsupported now)
    /// * ...
    formats: Vec<String>,
    /// Application cmd args for specific configs.
    ///
    /// * /opt/configs/omiga.yml
    /// * /opt/configs/omiga-dev.yml
    /// * /opt/configs/application.yml
    /// * /opt/configs/application-dev.yml
    /// * ...
    paths: Vec<String>,
    /// Search paths.
    /// * .
    /// * ./configs
    /// * ./resources
    /// * ...
    search_paths: Vec<String>,
    /// Application cmd args.
    ///
    /// * --omiga.server.port=9320
    /// * --omiga.application.name=omiga
    /// * ...
    kv: Option<Kv>,
}

impl OmigaApplicationBuilder {
    pub fn new() -> Self {
        Self {
            configs: Vec::new(),
            profiles: vec![SIGMA_CORE_PROFILE_ACTIVES_DEFAULT.to_string()],
            formats: Vec::new(),
            paths: Vec::new(),
            search_paths: Vec::new(),
            kv: Some(Kv::new()),
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

    pub fn format(mut self, format: String) -> Self {
        self.formats.push(format);

        self
    }

    pub fn formats(mut self, format: Vec<String>) -> Self {
        self.formats.extend(format);

        self
    }

    // ----------------------------------------------------------------

    pub fn search_path(mut self, search_path: String) -> Self {
        self.search_paths.push(search_path);

        self
    }

    pub fn search_paths(mut self, search_paths: Vec<String>) -> Self {
        self.search_paths.extend(search_paths);

        self
    }

    // ----------------------------------------------------------------

    fn is_not_default_profile(profile: &str) -> bool {
        profile != SIGMA_CORE_PROFILE_ACTIVES_DEFAULT
    }

    // ----------------------------------------------------------------

    pub fn build(self) -> OmigaApplication {
        OmigaApplication::new(
            self.configs,
            self.profiles,
            self.formats,
            self.paths,
            self.search_paths,
            self.kv,
        )
    }
}

impl Default for OmigaApplicationBuilder {
    fn default() -> Self {
        Self::new()
    }
}
