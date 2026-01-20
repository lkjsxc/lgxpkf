FROM node:20-bullseye-slim AS web
WORKDIR /web
COPY package.json package-lock.json tsconfig.json ./
COPY scripts ./scripts
COPY src/web/ts ./src/web/ts
RUN npm ci
RUN npm run build:web

FROM rust:1.83 AS builder
WORKDIR /app
COPY Cargo.toml ./
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
COPY src ./src
COPY --from=web /web/src/web/assets /app/src/web/assets
RUN find src -type f -exec touch {} + \
	&& cargo build --release

FROM gcr.io/distroless/cc-debian12:nonroot
WORKDIR /app
COPY --from=builder /app/target/release/lgxpkf /app/lgxpkf
COPY db/migrations /app/db/migrations
ENV BIND_ADDR=0.0.0.0:8080
ENV MIGRATIONS_PATH=/app/db/migrations
CMD ["/app/lgxpkf"]
