# Build website
FROM node:20-slim AS website-builder
WORKDIR /app/website
COPY website/package*.json ./
RUN npm install
COPY website .
RUN npm run build

FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin clams-tech-server

# Copy built static files from website
COPY --from=website-builder /app/static ./static

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime

# Install SQLite and libcurl
RUN apt-get update && apt-get install -y \
    libsqlite3-0 \
    libcurl4 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/clams-tech-server /usr/local/bin
COPY --from=builder /app/static ./static
ENTRYPOINT ["/usr/local/bin/clams-tech-server"]
