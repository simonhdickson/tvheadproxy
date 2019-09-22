FROM ekidd/rust-musl-builder:latest as cargo-build

WORKDIR /home/rust/src

ADD . ./

RUN sudo chown -R rust:rust .

RUN cargo build --release

FROM alpine:latest

RUN addgroup -g 1000 tvheadproxy

RUN adduser -D -s /bin/sh -u 1000 -G tvheadproxy tvheadproxy

WORKDIR /home/tvheadproxy/bin/

COPY --from=cargo-build /home/rust/src/target/x86_64-unknown-linux-musl/release/tvheadproxy tvheadproxy

RUN chown tvheadproxy:tvheadproxy tvheadproxy

USER tvheadproxy

EXPOSE 5004

ENTRYPOINT ["./tvheadproxy"]
