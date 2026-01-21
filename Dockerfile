# Stage 1: Build
FROM rust:1.84-alpine AS builder

RUN apk add --no-cache musl-dev pkgconfig openssl-dev

WORKDIR /app

COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

COPY src ./src
RUN touch src/main.rs && cargo build --release

# Stage 2: Runtime
FROM alpine:3.21

RUN apk add --no-cache ca-certificates libgcc \
    && addgroup -S appgroup \
    && adduser -S appuser -G appgroup \
    && mkdir -p /app/data \
    && chown -R appuser:appgroup /app

WORKDIR /app

COPY --from=builder /app/target/release/reversal /app/reversal

USER appuser

ENV HOST=0.0.0.0
ENV PORT=3000
ENV DB_TYPE=sqlite

VOLUME ["/app/data"]

HEALTHCHECK --interval=1m --timeout=30s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:3000/ || exit 1

EXPOSE 3000

CMD ["/app/reversal"]
