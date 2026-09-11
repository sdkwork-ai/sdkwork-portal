FROM rust:1-bookworm AS builder
WORKDIR /src
COPY . .
RUN cargo build --release -p sdkwork-api-portal-standalone-gateway

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /src/target/release/sdkwork-api-portal-standalone-gateway /usr/local/bin/sdkwork-api-portal-standalone-gateway
ENV PORTAL_API_BIND=0.0.0.0:18091
EXPOSE 18091
CMD ["sdkwork-api-portal-standalone-gateway"]
