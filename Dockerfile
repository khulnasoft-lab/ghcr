FROM rust:1.86 as builder
WORKDIR /usr/src/ghcr
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/ghcr /usr/local/bin/ghcr
ENTRYPOINT ["ghcr"]