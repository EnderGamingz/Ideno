FROM rust:latest as builder

WORKDIR /usr/src

RUN USER=root cargo new ideno-server
RUN apt update && apt install -y pkg-config musl-tools musl-dev libssl-dev


COPY Cargo.toml /usr/src/ideno-server/

WORKDIR /usr/src/ideno-server

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl --release

COPY src /usr/src/ideno-server/src/
COPY migrations /usr/src/ideno-server/migrations
RUN mkdir -p /usr/src/ideno-server/.temp

COPY .env.building /usr/src/ideno-server/.env

RUN touch /usr/src/ideno-server/src/main.rs
RUN cargo build --target x86_64-unknown-linux-musl --release
RUN cargo install sqlx-cli
RUN cargo sqlx database create
RUN cargo sqlx migrate run


FROM alpine:3.16.0 AS runtime
USER root

RUN apk --no-cache add ca-certificates gcompat \
  && update-ca-certificates
COPY --from=builder /usr/src/ideno-server/target/x86_64-unknown-linux-musl/release/ideno-server /usr/src/ideno/ideno-server
COPY --from=builder /usr/src/ideno-server/.temp /usr/src/ideno/.temp
RUN chmod 700 /usr/src/ideno/ideno-server
CMD ["./usr/src/ideno/ideno-server"]
