FROM rust:latest as cargo-build

RUN apt-get update

RUN apt-get install musl-tools -y

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/tvheadproxy

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/tvheadproxy*

COPY src src

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:latest

RUN addgroup -g 1000 tvheadproxy

RUN adduser -D -s /bin/sh -u 1000 -G tvheadproxy tvheadproxy

WORKDIR /home/tvheadproxy/bin/

COPY --from=cargo-build /usr/src/tvheadproxy/target/x86_64-unknown-linux-musl/release/tvheadproxy .

RUN chown tvheadproxy:tvheadproxy tvheadproxy

USER tvheadproxy

ENTRYPOINT ["./tvheadproxy"]
