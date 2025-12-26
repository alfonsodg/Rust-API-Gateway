# Multi-stage build for minimal production image
FROM rust:1.75-slim as builder

WORKDIR /app
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -r -s /bin/false rustway

WORKDIR /app

# Copy binary from builder stage
COPY --from=builder /app/target/release/main /usr/local/bin/rustway
COPY --from=builder /app/gateway.yaml /app/
COPY --from=builder /app/api_keys.yaml /app/

# Set ownership
RUN chown -R rustway:rustway /app

USER rustway

EXPOSE 8081

CMD ["rustway", "--config", "/app/gateway.yaml"]
