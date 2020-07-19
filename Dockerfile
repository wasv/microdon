FROM rust:latest

RUN apt-get update && apt-get install -y pkg-config libssl-dev libpq-dev \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path . --no-default-features --bin web --features web && cargo clean

EXPOSE 8088

CMD ["web"]
