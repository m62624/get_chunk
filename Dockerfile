FROM rust:latest
LABEL org.opencontainers.image.source="https://github.com/m62624/get_chunk"

WORKDIR /usr/src/get_chunk
RUN cargo install cargo-tarpaulin

COPY . .