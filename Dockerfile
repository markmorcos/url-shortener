FROM rust:slim AS builder

RUN rustup target add aarch64-unknown-linux-musl

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo fetch

COPY . .
RUN cargo build --release --target aarch64-unknown-linux-musl

FROM scratch
COPY --from=builder /app/target/aarch64-unknown-linux-musl/release/url-shortener /url-shortener
COPY static /static

EXPOSE 3000
CMD ["/url-shortener"]
