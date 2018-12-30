FROM rust:1.31.1-slim-stretch AS builder

WORKDIR /usr/src/myapp
COPY . .

RUN cargo build --release

FROM ubuntu:18.04
WORKDIR /
COPY --from=builder /usr/src/myapp/target/release/testproj .
ENTRYPOINT ["/testproj"]