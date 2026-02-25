FROM rust:1.93-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:trixie-slim AS prod
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/foodbasket-backend .

EXPOSE 3000
CMD ["./foodbasket-backend"]
