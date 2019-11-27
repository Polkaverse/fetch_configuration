/**
 * Copyright (c) 2019 Pankaj Chaudhary
 *
 * This source code is licensed under the MIT License found in
 * the LICENSE.md file in the root directory of this source tree.
 */

use std::collections::HashMap;
use std::fs::read_to_string;

use json;

/// This function retrieve the value from the config file.
///
/// #Arguments
///
/// *'config_file_path' - A string literal which can store the path of config file.
///
/// #Return
///
/// Return the Hashmap of Config elements or the error given by the compiler.
pub fn configuration(config_file_path: &str) -> Result<HashMap<String, String>, String> {
    let config_path = read_to_string(config_file_path);
    let mut configurations = HashMap::new();
    match config_path {
        Ok(path) => {
            let data = json::parse(path.as_ref());
            match data {
                Ok(values) => {
                    for (key, value) in values.entries() {
                        configurations.insert(key.to_string(), value.as_str().unwrap().to_string());
                    }
                    Ok(configurations)
                }
                Err(error) => Err(error.to_string()),
            }
        }
        Err(error) => Err(error.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use crate::configuration;

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
}
