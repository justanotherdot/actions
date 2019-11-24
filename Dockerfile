FROM rust:1.39-stretch AS builder
WORKDIR /build
COPY . .
RUN rustup install nightly
RUN cargo +nightly build --release

FROM debian:stretch
COPY --from=builder /build/target/release/actions /usr/local/bin/actions
