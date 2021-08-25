FROM rust:latest AS builder

COPY Cargo.toml ./
COPY src ./src

RUN cargo build --release


FROM archlinux:latest
COPY --from=builder /target/release/model-api model-api
CMD ["./model-api"]
