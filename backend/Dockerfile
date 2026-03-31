FROM rust:1.85-bookworm AS builder

WORKDIR /app
COPY backend/Cargo.toml backend/Cargo.lock* ./
COPY backend/src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates python3 default-jre g++ make && rm -rf /var/lib/apt/lists/*
WORKDIR /srv/app/backend
COPY --from=builder /app/target/release/realtime-detail-backend /usr/local/bin/realtime-detail-backend
COPY plugins /srv/app/plugins
RUN mkdir -p /srv/app/backend/logs
EXPOSE 8080
ENV RUST_LOG=info
CMD ["realtime-detail-backend"]
