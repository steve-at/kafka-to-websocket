# step 1 --building
FROM rust:1.50 as builder

# defining a user for the builder step
RUN USER=root cargo new --bin kafka_ws
WORKDIR ./kafka_ws

# copying the dependency manifest.
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# install cmake for rdkafka
RUN apt-get update \
    && apt-get install -y cmake \
    && rm -rf /var/lib/apt/lists/*
# build a dummy project with the dependencies. This will reduce build time if only the code changes but the deps stay
RUN cargo build --release
RUN rm src/*.rs

ADD . ./
# remove the dep files for the dummy project (only the ones for the project, not the ones for the dependencies
RUN rm ./target/release/deps/kafka_ws*
# compile the app
RUN cargo build --release


# step 2 -- running
FROM debian:buster-slim

# define a APP directory
ARG APP=/usr/src/app
# install packages for web stuff
RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

# setting up environment variables for the user name (no root privilegues for the container) and the timezone
ENV TZ=Etc/UTC \
    APP_USER=appuser

# create a group for the user, add the user to the group and create the usr directory
RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

# copy the compiled app to the urs directory
COPY --from=builder /kafka_ws/target/release/kafka_ws ${APP}/kafka_ws

# change the ownership to the user
RUN chown -R $APP_USER:$APP_USER ${APP}

# set the user who runs the app and change the directory
USER $APP_USER
WORKDIR ${APP}

# run the app
CMD ["./kafka_ws"]
