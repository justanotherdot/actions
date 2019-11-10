FROM rust:1.39-stretch AS builder
WORKDIR /build
COPY . .
RUN cargo build --release

FROM debian:stretch
COPY --from=builder /build/target/release/actions /usr/bin/local/actions
RUN /usr/bin/local/actions
