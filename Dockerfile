FROM rust:1.86 as builder
WORKDIR /usr/src/ghcr
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim@sha256:1d2b8b5e1e3c3d8b3b3c2a1f2e1d8e1b2c3a4e5b6f7a8b9c0d1e2f3a4b5c6d7e8
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