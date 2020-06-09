use serde_json::Value;

use std::collections::HashMap;

pub type ConfigResult<T> = Result<T, ConfigError>;

#[derive(Debug)]
pub enum ConfigError {
    KeyExists(String),
    UnknownKey(String)
}

#[derive(Default)]
pub struct Configuration {
    data: HashMap<String, Value>
}

impl Configuration {

    pub fn insert_value(&mut self, key: &str, value: Value) {
        self.data.insert(key.into(), value);
    }

    pub fn remove_value(&mut self, key: &str) -> ConfigResult<()> {
        if self.data.contains_key(key) {
            self.data.remove(key);
            Ok(())
        } else {
            Err(ConfigError::UnknownKey(key.into()))
        }
    }

    pub fn get(&self, key: &str) -> ConfigResult<Value> {
        if self.data.contains_key(key) {
            Ok(self.data.get(key).unwrap().clone())
        } else {
            Err(ConfigError::UnknownKey(key.into()))
        }
    }

    pub fn get_default(&self, key: &str, value: Value) -> Value {
        if self.data.contains_key(key) {
            self.data.get(key).unwrap().clone()
        } else {
            value
        }
    }

    pub fn get_from_value_default<T: serde::de::DeserializeOwned>(&self, key: &str, value: T) -> T {
        if let Some(val) = self.data.get(key) {
            match serde_json::from_value::<T>(val.clone()) {
                Ok(fin) => fin,
                Err(err) => {
                    warn!("Failed to parse json value: {}", err);
                    value
                }
            }
        } else {
            value
        }
    }
}
