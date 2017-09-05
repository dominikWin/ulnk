extern crate config;

fn main() {
    let mut settings: config::Config = config::Config::default();
    settings.merge(config::File::with_name("ulnk.toml")).unwrap();
    println!("In: {}", settings.get_str("name").expect("name not in config file"));
}
