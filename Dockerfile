#docker build --platform linux/amd64 -t kore-http:amd64 --target amd64 .
#docker build --platform linux/arm/v7 -t kore-http:arm --target arm .
#docker build --platform linux/arm64 -t kore-http:arm64 --target arm64 .

# Etapa de compilación para AMD64
FROM rust:1.78-slim-buster AS builder-amd64
WORKDIR /app
RUN rustup target add amd64-unknown-linux-gnu
RUN rustup target add wasm32-unknown-unknown
COPY . .
RUN cargo build --target amd64-unknown-linux-gnu --release

# Etapa de compilación para ARM
FROM rust:1.78-slim-buster AS builder-arm
WORKDIR /app
RUN rustup target add armv7-unknown-linux-gnueabihf
RUN rustup target add wasm32-unknown-unknown
COPY . .
RUN cargo build --target armv7-unknown-linux-gnueabihf --release

# Etapa de compilación para ARM64
FROM rust:1.78-slim-buster AS builder-arm64
WORKDIR /app
RUN rustup target add aarch64-unknown-linux-gnu
RUN rustup target add wasm32-unknown-unknown
COPY . .
RUN cargo build --target aarch64-unknown-linux-gnu --release

# Imagen final para AMD64
FROM gcr.io/distroless/cc-debian12 AS amd64
COPY --from=builder-amd64 /app/target/amd64-unknown-linux-gnu/release/kore-http /usr/local/bin/kore-http
CMD ["kore-http"]

# Imagen final para ARM
FROM gcr.io/distroless/cc-debian12 AS arm
COPY --from=builder-arm /app/target/armv7-unknown-linux-gnueabihf/release/kore-http /usr/local/bin/kore-http
CMD ["kore-http"]

# Imagen final para ARM64
FROM gcr.io/distroless/cc-debian12 AS arm64
COPY --from=builder-arm64 /app/target/aarch64-unknown-linux-gnu/release/kore-http /usr/local/bin/kore-http
CMD ["kore-http"]