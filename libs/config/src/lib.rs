pub use dotenv::{from_path, vars};
use error::Error;

#[allow(dead_code)]
pub fn is_dev() -> bool {
    get_default("IS_DEV", "false") == *"true"
}

#[allow(dead_code)]
fn print_env() {
    let env_vars = std::env::vars();
    for (key, value) in env_vars {
        println!("{key} = {value:?}");
    }
}

/// Initialize global configurations from dotenv
pub async fn initialize() {
    env();
    dotenv(None);
    args();
    println!("Configuration loaded and ready.");
}

/// When the dotenv file is on a different location, we will load it here
pub async fn initialize_with_config(path: String) {
    env();
    dotenv(Some(path));
    args();
    println!("Configuration loaded and ready.");
}

/// Set and replace existing key if found
pub fn set(key: &str, value: &str) {
    let temp = get(key);

    if temp.is_ok() {
        std::env::remove_var(key);
    }

    std::env::set_var(key, value)
}

/// Set the value only if previous values doesn't exist on its place
pub fn set_if_not_empty(key: &str, value: &str) {
    let temp = get(key);

    if temp.is_err() {
        std::env::set_var(key, value)
    }
}

/// Get configuration variable
pub fn get(key: &str) -> Result<String, Error> {
    let key = String::from(key);

    std::env::var(key).map_err(|_e| Error::NotFound)
}

/// Get mutliple keys at once back
pub fn get_multiple(keys: Vec<&str>) -> Result<Vec<String>, Error> {
    let mut collected = vec![];

    for key in keys.iter() {
        let value = get(key)?;

        collected.push(value);
    }

    Ok(collected)
}

/// Get value from config or pass the default value
pub fn get_default(key: &str, default: &str) -> String {
    get(key).unwrap_or_else(|_| String::from(default))
}

/// Get mutliple keys with set default values
pub fn get_multiple_default(keys: Vec<(&str, &str)>) -> Vec<String> {
    let mut collected = vec![];

    for (key, default) in keys.iter() {
        collected.push(get_default(key, default));
    }

    collected
}

/// Load values from dotenv and attach it to values
pub fn dotenv(path: Option<String>) {
    let vars: Vec<(String, String)> = match path {
        Some(p) => {
            match from_path(&p) {
                Ok(_) => (),
                Err(e) => panic!("Couldn't load the dotenv config at '{p}', error: {e}"),
            }

            vars().collect()
        }
        None => vars().collect(),
    };

    for (key, value) in vars.iter() {
        set(key, value);
    }
}

/// Load values from shell environment
pub fn env() {
    let vars: Vec<(String, String)> = std::env::vars().collect();

    for (key, value) in vars.iter() {
        set(key, value);
    }
}

/// Load values from command line arguments passed
pub fn args() {
    let args: Vec<String> = std::env::args().collect();

    for arg in args.iter() {
        let mut items = arg.split('=');

        if let Some(key) = items.next() {
            let value = items.next().unwrap_or("true");
            set(key, value);
        }
    }
}
