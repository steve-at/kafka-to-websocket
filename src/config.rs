use std::env;

#[derive(Debug)]
pub struct Config {
    pub address: String,
    pub port: String,
    pub topic: String,
    pub group_id: String,
    pub ws_host: String,
    pub ws_port: String,
    pub schema_registry: String,
    pub use_avro_schema: bool,
}

/// Collects the used environment variables.
pub fn handle_config() -> Config {
    let address = env::var("KAFKA_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("KAFKA_HOST_PORT").unwrap_or_else(|_| "9092".to_string());
    let topic = env::var("KAFKA_TOPIC").unwrap_or_else(|_| "default".to_string());
    let group_id = env::var("KAFKA_GROUP_ID").unwrap_or_else(|_| "consumer-1".to_string());
    // websocket environment
    let ws_host = env::var("WS_HOST").unwrap_or_else(|_| "localhost".to_string());
    let ws_port = env::var("WS_PORT").unwrap_or_else(|_| "8080".to_string());
    let schema_registry= env::var("SCHEMA_REGISTRY").unwrap_or_else(|_| "http://localhost:8081".to_string());
    let use_avro_schema = env::var("USE_AVRO_SCHEMA");
    let use_schema = match use_avro_schema {
        Ok(_) => { true }
        Err(_) => { false }
    };

    Config {
        address,
        port,
        topic,
        group_id,
        ws_host,
        ws_port,
        schema_registry,
        use_avro_schema: use_schema,
    }
}