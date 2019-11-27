extern crate fetch_configuration;
use fetch_configuration::configuration;

fn main() {
        let config = configuration("config.json").unwrap();
        let time = config.get(&"TIMER".to_string()) .expect("Key not present");
        print!("{}", time);
}
