FROM rust:latest AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev libpq-dev \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release --no-default-features --features web --bin web

FROM debian:stable-slim
COPY --from=builder /usr/src/app/target/release/web /usr/local/bin/

RUN apt-get update && apt-get install -y libssl1.1 libpq5 ca-certificates \
 && rm -rf /var/lib/apt/lists/*

USER 1000
EXPOSE 8080

CMD ["web"]
