FROM rust:1.67 AS builder
WORKDIR /usr/src/tamako
COPY . .
RUN cargo install cargo-binstall
# hadolint ignore=DL3059
RUN cargo binstall -y sqlx-cli
RUN --mount=type=cache,target=/usr/local/cargo,from=rust:1.67,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    cargo build --release && mv ./target/release/tamako ./tamako

FROM debian:bullseye-slim
# hadolint ignore=DL3008
RUN apt-get update && apt-get install -y --no-install-recommends libssl-dev pkg-config ca-certificates && apt-get clean && rm -rf /var/lib/apt/lists/*
WORKDIR /app
# RUN mkdir -p /app/data
COPY --from=builder /usr/src/tamako/tamako /usr/src/tamako/migrations/ /app/
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx
CMD [ "/app/tamako" ]