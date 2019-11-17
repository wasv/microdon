FROM rustlang/rust:nightly-slim

RUN apt-get update && apt-get install -y default-libmysqlclient-dev \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

CMD ["main"]
