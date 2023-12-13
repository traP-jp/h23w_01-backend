FROM rust:bullseye AS builder

WORKDIR /app
COPY . .
RUN touch Rocket.toml
RUN cargo build

FROM debian:bullseye

RUN apt-get -y update \
    && apt-get -y install build-essential libssl-dev ca-certificates \
    && update-ca-certificates --fresh
WORKDIR /app
COPY --from=builder /app/target/debug/handler ./main

# TODO: migration

CMD ["./main"]
