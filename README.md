#Kafka to Websocket

**TODO:**
*[X] Messages are forwarded
*[X] Remove hardcoded values
*[X] Split into models
*[ ] dockerize

**Usage**

should be like
* start container with topics and uri of kafka, port and port for websocket
* connect to websocket


# Environment variables
* WS_HOST "exposed websocket address" *default = **localhost***
* WS_PORT "exposed websocket port" *default = **8080***

* KAFKA_HOST "kafka host address" *default = **localhost***
* KAFKA_HOST_PORT "kafka host port" *default = **9092***
* KAFKA_TOPIC "topic to consume" *default = **default***
* KAFKA_GROUP_ID "group id" *default = **consumer-1***

