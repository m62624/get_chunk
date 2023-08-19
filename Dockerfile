FROM alpine:latest
LABEL org.opencontainers.image.source="https://github.com/m62624/get_chunk"
RUN apk update && \
    apk add --no-cache bash 
COPY ./target/x86_64-unknown-linux-musl/release/get_chunk /usr/bin/get_chunk