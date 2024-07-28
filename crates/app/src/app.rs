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

// omiga_app/app

// ----------------------------------------------------------------

use std::collections::HashSet;

use omiga_core::collection::hashset::hashset_join;
use omiga_core::constants::{COMMA, SIGMA_CORE_PROFILE_ACTIVES_DEFAULT};
use omiga_core::model::kv::Kv;

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
    configs: HashSet<String>,
    /// Config file profiles active.
    ///
    /// * dev
    /// * test
    /// * stage
    /// * prod
    /// * ...
    profiles: HashSet<String>,
    /// Config file format.
    ///
    /// * toml
    /// * yml | yaml (Unsupported now)
    /// * json (Unsupported now)
    /// * properties (Unsupported now)
    /// * ini (Unsupported now)
    /// * ...
    formats: HashSet<String>,
    /// Application cmd args for specific configs.
    ///
    /// * /opt/configs/omiga.yml
    /// * /opt/configs/omiga-dev.yml
    /// * /opt/configs/application.yml
    /// * /opt/configs/application-dev.yml
    /// * ...
    paths: HashSet<String>,
    /// Search paths.
    /// * .
    /// * ./configs
    /// * ./resources
    /// * ...
    search_paths: HashSet<String>,
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
        configs: HashSet<String>,
        profiles: HashSet<String>,
        formats: HashSet<String>,
        paths: HashSet<String>,
        search_paths: HashSet<String>,
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
        hashset_join(&self.profiles, COMMA)
    }

    pub fn profiles_active_array(&self) -> HashSet<String> {
        self.profiles.clone()
    }

    // ----------------------------------------------------------------

    pub fn configs(&self) -> String {
        hashset_join(&self.configs, COMMA)
    }

    pub fn configs_array(&self) -> HashSet<String> {
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
    configs: HashSet<String>,
    /// Config file profiles active.
    ///
    /// * dev
    /// * test
    /// * stage
    /// * prod
    /// * ...
    profiles: HashSet<String>,
    /// Config file format.
    ///
    /// * toml
    /// * yml | yaml (Unsupported now)
    /// * json (Unsupported now)
    /// * properties (Unsupported now)
    /// * ini (Unsupported now)
    /// * ...
    formats: HashSet<String>,
    /// Application cmd args for specific configs.
    ///
    /// * /opt/configs/omiga.yml
    /// * /opt/configs/omiga-dev.yml
    /// * /opt/configs/application.yml
    /// * /opt/configs/application-dev.yml
    /// * ...
    paths: HashSet<String>,
    /// Search paths.
    /// * .
    /// * ./configs
    /// * ./resources
    /// * ...
    search_paths: HashSet<String>,
    /// Application cmd args.
    ///
    /// * --omiga.server.port=9320
    /// * --omiga.application.name=omiga
    /// * ...
    kv: Option<Kv>,
}

impl OmigaApplicationBuilder {
    pub fn new() -> Self {
        let mut default_profiles = HashSet::new();
        default_profiles.insert(SIGMA_CORE_PROFILE_ACTIVES_DEFAULT.to_string());

        Self {
            configs: HashSet::new(),
            profiles: default_profiles,
            formats: HashSet::new(),
            paths: HashSet::new(),
            search_paths: HashSet::new(),
            kv: Some(Kv::new()),
        }
    }

    // ----------------------------------------------------------------

    pub fn config(mut self, config: String) -> Self {
        self.configs.insert(config);

        self
    }

    pub fn configs(mut self, configs: Vec<String>) -> Self {
        self.configs.extend(configs);

        self
    }

    // ----------------------------------------------------------------

    pub fn profile(mut self, profile: String) -> Self {
        self.profiles.retain(|p| Self::is_not_default_profile(p));
        self.profiles.insert(profile);

        self
    }

    pub fn profiles(mut self, profiles: Vec<String>) -> Self {
        self.profiles.extend(profiles);

        self
    }

    // ----------------------------------------------------------------

    pub fn format(mut self, format: String) -> Self {
        self.formats.insert(format);

        self
    }

    pub fn formats(mut self, format: Vec<String>) -> Self {
        self.formats.extend(format);

        self
    }

    // ----------------------------------------------------------------

    pub fn search_path(mut self, search_path: String) -> Self {
        self.search_paths.insert(search_path);

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
