FROM rust:1.78 AS builder

WORKDIR /

COPY src/ ./src/
COPY Cargo.toml ./
COPY .env ./

RUN cargo build --release

FROM debian:bookworm-slim

COPY --from=builder /target/release/back /back
COPY --from=builder /.env /.env

EXPOSE 18080

CMD ["/back"]
