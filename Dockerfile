FROM rust:1.83 AS builder
WORKDIR /app
COPY Cargo.toml ./
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
COPY src ./src
RUN find src -type f -exec touch {} + \
	&& cargo build --release

FROM gcr.io/distroless/cc-debian12:nonroot
WORKDIR /app
COPY --from=builder /app/target/release/lgxpkf /app/lgxpkf
ENV BIND_ADDR=0.0.0.0:8080
CMD ["/app/lgxpkf"]
