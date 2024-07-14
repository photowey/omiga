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

// integration_tests

// ----------------------------------------------------------------

use omigaapp::app::{Application, OmigaApplication};
use omigacore::constants::SIGMA_CORE_PROFILE_ACTIVES_DEFAULT;

// ----------------------------------------------------------------

#[test]
fn test_it_works() {
    println!("Hello, Omiga!");
}

// ----------------------------------------------------------------

#[test]
fn test_app_default_profile() {
    let app = OmigaApplication::builder().build();

    let profiles = app.profiles_active();
    assert_eq!(SIGMA_CORE_PROFILE_ACTIVES_DEFAULT, profiles);

    let ok = app.is_default_profile();
    assert!(ok);

    app.run()
}

#[test]
fn test_app_profile_dynamic() {
    let app = OmigaApplication::builder()
        .profile("override".to_string())
        .profiles(vec!["dev".to_string(), "dynamic".to_string()])
        .build();

    let profiles = app.profiles_active();
    assert_eq!("override,dev,dynamic", profiles);

    let ok = app.is_default_profile();
    assert!(!ok);

    app.run()
}

#[test]
fn test_app_profile_dynamic_array() {
    let app = OmigaApplication::builder()
        .profile("override".to_string())
        .profiles(vec!["dev".to_string(), "dynamic".to_string()])
        .build();

    let profiles = app.profiles_active_array();

    assert_eq!(3, profiles.len());
    assert!(profiles.iter().any(|x| x == "override"));
    assert!(profiles.iter().any(|x| x == "dev"));
    assert!(profiles.iter().any(|x| x == "dynamic"));

    let ok = app.is_default_profile();
    assert!(!ok);

    app.run()
}

#[test]
fn test_app_configs() {
    let app = OmigaApplication::builder()
        .config("omiga.toml".to_string())
        .configs(vec![
            "omiga-dev.toml".to_string(),
            "omiga-dynamic.toml".to_string(),
        ])
        .build();

    let configs = app.configs();
    assert_eq!("omiga.toml,omiga-dev.toml,omiga-dynamic.toml", configs);

    app.run()
}

#[test]
fn test_app_configs_array() {
    let app = OmigaApplication::builder()
        .config("omiga.toml".to_string())
        .configs(vec![
            "omiga-dev.toml".to_string(),
            "omiga-dynamic.toml".to_string(),
        ])
        .build();

    let configs = app.configs_array();

    assert_eq!(3, configs.len());
    assert!(configs.iter().any(|x| x == "omiga.toml"));
    assert!(configs.iter().any(|x| x == "omiga-dev.toml"));
    assert!(configs.iter().any(|x| x == "omiga-dynamic.toml"));

    app.run()
}
