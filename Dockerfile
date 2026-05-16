FROM rust:1.78-slim AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY contracts ./contracts
COPY backend ./backend
RUN cargo build -p halt-engine-backend --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/backend /usr/local/bin/backend
CMD ["backend"]
