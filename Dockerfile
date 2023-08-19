FROM rust:1-slim-bookworm
LABEL org.opencontainers.image.source="https://github.com/m62624/get_chunk"
RUN cargo install get_chunk
ENTRYPOINT ["get_chunk"]