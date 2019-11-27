/**
 * Copyright (c) 2019 Pankaj Chaudhary
 *
 * This source code is licensed under the MIT License found in
 * the LICENSE.md file in the root directory of this source tree.
 */

#[cfg(test)]
use fetch_configuration::configuration;

static CONFIG_FILE: &str = "config.json";
static INVALID_CONFIG_FILE_PATH: &str = "configg.txt";

#[test]
fn test_variable_value_success() {
    if let Ok(configuration) = configuration(CONFIG_FILE) {
        assert_eq!(
            configuration
                .get(&"NAME".to_string())
                .expect("Key not present"),
            "Configuration"
        );
    }
}

#[test]
fn test_variable_value_failure() {
    if let Err(error) = configuration(INVALID_CONFIG_FILE_PATH) {
        assert_eq!(error, "No such file or directory (os error 2)");
    }
}
