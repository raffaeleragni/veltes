FROM docker.io/lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS reciper
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=reciper /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

FROM docker.io/rust:1.78-slim as runtime
COPY --from=builder /app/target/release/veltes /usr/local/bin
ENTRYPOINT ["/usr/local/bin/veltes"]
