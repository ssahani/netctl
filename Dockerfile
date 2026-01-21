# Multi-stage Dockerfile for static netctl binary

FROM rust:1.75-alpine AS builder

RUN apk add --no-cache musl-dev pkgconfig dbus-dev linux-headers

WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

ENV RUSTFLAGS="-C target-feature=+crt-static"
RUN cargo build --release --target x86_64-unknown-linux-musl

RUN strip target/x86_64-unknown-linux-musl/release/netctl

FROM scratch
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/netctl /netctl

ENTRYPOINT ["/netctl"]
CMD ["--help"]
