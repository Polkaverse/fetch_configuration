# fetch_configuration 
<p align="left">
  <a href="https://travis-ci.org/pankajchaudhary5/fetch_configuration">
    <img alt="Build Status" src="https://travis-ci.org/PankajChaudhary5/fetch_configuration.svg?branch=master">
  </a>
  <a href="https://crates.io/crates/fetch_configuration">
    <img alt="Latest version" src="https://img.shields.io/crates/v/fetch_configuration.svg">
  </a>
  <img alt="Stability stable" src="https://img.shields.io/badge/stability-stable-green.svg">
</p>

A Library to fetch the configurations form the config file.

## Use
Add dependency in Cargo.toml
```sh
[dependencies]
fetch_configuration = "0.1.0"
```
Example to use fetch configuration crate
```sh
extern crate fetch_configuration;
use fetch_configuration::configuration;

fn main() {
        let config = configuration("config.json").unwrap();
        let time = config.get(&"TIMER".to_string()) .expect("Key not present");
        print!("{}", time);
}
```
## Contributing

If you want to contribute to this crate please take a look to [Click Me](https://github.com/PankajChaudhary5/fetch_configuration).
