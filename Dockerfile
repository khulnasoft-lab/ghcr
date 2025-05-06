FROM rust:1.86 as builder
WORKDIR /usr/src/ghcr
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim@sha256:fdd75562fdcde1039c2480a1ea1cd2cf03b18b6e4cb551cabb03bde66ade8a5d
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