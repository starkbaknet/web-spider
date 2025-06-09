FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN cargo build --release || true

COPY src ./src

RUN cargo build --release

FROM ubuntu:latest

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -r -s /bin/false appuser

WORKDIR /app

COPY --from=builder /app/target/release/spider /app/spider

RUN chown appuser:appuser /app/spider

USER appuser

CMD ["./spider"]
