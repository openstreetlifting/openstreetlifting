FROM rust:1.100.0-slim-bookworm AS chef
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*
RUN cargo install cargo-chef --locked --version 0.1.77
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
ARG BIN
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --locked --recipe-path recipe.json --bin "${BIN}"
COPY . .
RUN cargo build --release --locked --bin "${BIN}"

FROM gcr.io/distroless/cc-debian12:nonroot
WORKDIR /app
ARG BIN
COPY --from=builder /app/target/release/${BIN} /app/server
COPY --from=builder /app/data /app/data
EXPOSE 8080
ENTRYPOINT ["/app/server"]
