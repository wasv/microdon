FROM rustlang/rust:nightly-slim

RUN apt-get update && apt-get install -y pkg-config libssl-dev libpq-dev \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path . && cargo clean

EXPOSE 8000

CMD ["web"]