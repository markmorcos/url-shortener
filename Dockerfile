FROM rust:slim AS builder

# Static musl build so the binary runs in `FROM scratch` (no libc there).
# Each arch builds on its own native runner, so target the host's own arch —
# uname -m ("x86_64" / "aarch64") matches the rust musl triple prefix exactly.
# (Hardcoding aarch64 broke the amd64 runner, which then cross-linked with ld.)
RUN rustup target add "$(uname -m)-unknown-linux-musl"

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo fetch

COPY . .
RUN MUSL_TARGET="$(uname -m)-unknown-linux-musl" && \
    cargo build --release --target "$MUSL_TARGET" && \
    cp "target/$MUSL_TARGET/release/url-shortener" /url-shortener

FROM scratch

WORKDIR /app
COPY --from=builder /url-shortener ./url-shortener
COPY static ./static

EXPOSE 3000

CMD ["./url-shortener"]
