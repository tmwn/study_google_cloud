# https://shaneutt.com/blog/rust-fast-small-docker-image-builds/
FROM rust:1.56.0

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src/
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/app*

COPY . .
RUN cargo build --release

ENTRYPOINT ["/app/target/release/app"]
