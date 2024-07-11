FROM rust:1.79 as builder
ARG APP_NAME

WORKDIR /usr/src/${APP_NAME}
COPY src src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY Rocket.toml Rocket.toml

RUN cargo install --locked --path .

FROM ubuntu:24.04
ARG APP_NAME
ARG DEPENDENCIES
RUN apt-get update && apt-get install -y ${DEPENDENCIES} && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/${APP_NAME} /usr/local/bin/app
CMD ["app"]
