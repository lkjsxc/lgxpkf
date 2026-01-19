FROM rust:1.83 AS builder
WORKDIR /app
COPY Cargo.toml ./
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update \
	&& apt-get install -y --no-install-recommends libssl3 ca-certificates \
	&& rm -rf /var/lib/apt/lists/*
RUN useradd -m appuser
WORKDIR /app
COPY --from=builder /app/target/release/lgxpkf /app/lgxpkf
USER appuser
ENV BIND_ADDR=0.0.0.0:8080
CMD ["/app/lgxpkf"]
