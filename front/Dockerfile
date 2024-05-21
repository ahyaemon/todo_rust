FROM rust:1.78 as builder

WORKDIR /

RUN rustup target add wasm32-unknown-unknown
RUN cargo install dioxus-cli

COPY assets/ ./assets/
COPY src/ ./src/
COPY Cargo.toml ./
COPY Dioxus.toml ./

RUN dx build --release

FROM nginx:latest

WORKDIR /
COPY nginx/default.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /dist/ /var/www/
