FROM rust:1.88 AS builder
WORKDIR /usr/src/ghcr
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
LABEL org.opencontainers.image.title="ghcr" \
      org.opencontainers.image.description="A CLI tool to build and publish Docker images to GHCR" \
      org.opencontainers.image.authors="Md Sulaiman <dev.sulaiman@icloud.com>"

# Add a non-root user
RUN useradd -m appuser
COPY --from=builder /usr/local/cargo/bin/ghcr /usr/local/bin/ghcr
USER appuser

HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
  CMD ["ghcr", "--version"] || exit 1

ENTRYPOINT ["ghcr"]