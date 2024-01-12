FROM rust:latest as builder

WORKDIR /usr/src

RUN USER=root cargo new ideno-server
RUN apt update && apt install -y pkg-config musl-tools musl-dev libssl-dev


COPY Cargo.toml /usr/src/ideno-server/

WORKDIR /usr/src/ideno-server

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl --release

COPY src /usr/src/ideno-server/src/

RUN touch /usr/src/ideno-server/src/main.rs
RUN cargo build --target x86_64-unknown-linux-musl --release


FROM alpine:3.16.0 AS runtime
USER root

RUN apk --no-cache add ca-certificates gcompat \
  && update-ca-certificates
COPY --from=builder /usr/src/ideno-server/target/x86_64-unknown-linux-musl/release/ideno-server /usr/bin
RUN chmod 700 /usr/bin/ideno-server
CMD ["ideno-server"]
