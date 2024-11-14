FROM clux/muslrust:stable AS base
USER root
RUN groupadd -g 10001 -r dockergrp && useradd -r -g dockergrp -u 10001 dockeruser
RUN cargo install cargo-chef
WORKDIR /app

FROM base AS chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM base as builder
COPY --from=chef /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM docker:dind
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/docker-registry-actions /app/
RUN chmod +x /app/docker-registry-actions
ENTRYPOINT ["/app/docker-registry-actions"]