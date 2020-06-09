use serde_json::Value;
use xi_term::components::Configuration;

#[test]
fn config_test_simple() {
    let mut config = Configuration::default();

    assert_eq!(Value::Bool(false), config.get_default("test-key", Value::Bool(false)));

    config.insert_value("test-key", Value::Bool(true));

    assert_eq!(Value::Bool(true), config.get_default("test-key", Value::Bool(false)));
}
