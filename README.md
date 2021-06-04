# Kafka to Websocket

**Usage**

should be like
* start container with topics and uri of kafka, port and port for websocket
* connect to websocket


## Environment variables
* WS_HOST "exposed websocket address" *default = **localhost***
* WS_PORT "exposed websocket port" *default = **8080***
* 
* KAFKA_HOST "kafka host address" *default = **localhost***
* KAFKA_HOST_PORT "kafka host port" *default = **9092***
* KAFKA_TOPIC "topic to consume" *default = **default***
* KAFKA_GROUP_ID "group id" *default = **consumer-1***
* USE_AVRO_SCHMEA "is a Avro Schema is used?" *default = **false** **! it only checks if a value is set so passing false will be the same as passing true**
* SCHEMA_REGISTRY "the url of the Schema registry is used" *default = **http://localhost:8081***

