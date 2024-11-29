//
use core::panic;
use lazy_static::lazy_static;
use std::env;
use tracing::debug;

lazy_static! {
    pub static ref ENV_CONFIG: EnvConfig = get_env_config();
}

pub fn get_env_var(key: &str) -> Option<String> {
    let var = match env::var(key) {
        Err(env::VarError::NotPresent) => None,
        Err(e) => panic!("{e}"),
        Ok(var) => Some(var),
    };

    debug!("env var {key}:{:?}", var);

    var
}

pub fn get_env_bool(key: &str) -> Option<bool> {
    get_env_var(key).map(|var| match var.to_lowercase().as_str() {
        "true" => true,
        "false" => false,
        "t" => true,
        "f" => false,
        "1" => true,
        "0" => false,
        str => panic!("invalid bool value {str} for {key}"),
    })
}

pub fn get_env_config() -> EnvConfig {
    EnvConfig {
        db_url: get_env_var("DATABASE_URL").expect("DATABASE_URL is required"),
        log_json: get_env_bool("LOG_JSON").unwrap_or(false),
        log_perf: get_env_bool("LOG_PERF").unwrap_or(false),
    }
}

pub struct EnvConfig {
    pub db_url: String,
    pub log_json: bool,
    pub log_perf: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_and_set_env_vars() {
        let test_key = "test_env_key";
        let test_value = "test_env_value";
        std::env::set_var(test_key, test_value);

        // query env var by test key
        let query_env_var =
            get_env_var(&test_key).expect("query env var should not empty");
        println!("query_env_var: {}", &query_env_var);
        assert_eq!(&query_env_var, test_value);

        let test_key_bool_type = "test_key_bool_type";
        let test_value_bool_value = "true";
        std::env::set_var(test_key_bool_type, test_value_bool_value);
        assert_eq!(
            true,
            get_env_bool(test_key_bool_type)
                .expect("value of bool should exist")
        );
    }

    #[test]
    fn test_not_exist_env_vars() {
        let test_env_key = "test_env_key_name";
        assert_eq!(None, get_env_var(test_env_key));
        assert_eq!(None, get_env_bool(test_env_key));
    }

    #[test]
    fn test_env_var_true_exist() {
        let key = "env_var_key";
        let value = "TRUE";
        std::env::set_var(key, value);

        assert!(get_env_bool(key).expect("query env var should be true"));
    }

    #[test]
    fn test_env_var_false_exist() {
        let key = "env_bool_var_key";
        let value = "FalSE";
        std::env::set_var(key, value);

        assert_eq!(
            false,
            get_env_bool(key).expect("query env var should be false")
        );
    }
}
