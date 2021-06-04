# Kafka to Websocket

**Usage**
the usage is straight forward. Just start the container with the environment variables. 

When using with an AVRO schema the **USE_AVRO_SCHEMA** environment must be set.

The docker-compose.yml shows a basic usage which comes with a (basic) Kafka environment.

## Environment variables
* WS_HOST "exposed websocket address" *default = **localhost***
* WS_PORT "exposed websocket port" *default = **8080***
* KAFKA_HOST "kafka host address" *default = **localhost***
* KAFKA_HOST_PORT "kafka host port" *default = **9092***
* KAFKA_TOPIC "topic to consume" *default = **default***
* KAFKA_GROUP_ID "group id" *default = **consumer-1***
* USE_AVRO_SCHEMA "is a Avro Schema is used?" *default = **nothing** **! it only checks if a value is set so passing false will be the same as passing true**
* SCHEMA_REGISTRY "the url of the Schema registry is used" *default = **http://localhost:8081***


## Other default settings (not changable via environment)

* partition.eof = false
* session timeout = 6000ms
* auto commit = true

all other options are the default values from rdkafka
