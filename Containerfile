FROM rust:1.88-slim-bullseye AS builder
WORKDIR /src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY proto/Cargo.toml proto/Cargo.toml
RUN mkdir src/ && echo "fn main() {println!(\"failed to build\")}" > src/main.rs
RUN mkdir -p proto/src/ && echo "" > proto/src/lib.rs
RUN cargo build --release
RUN rm -f target/release/deps/eprofiler*
COPY . .
RUN cargo build --locked --release
RUN mkdir -p build-out/
RUN cp target/release/eprofiler-tui build-out/

FROM debian:bullseye-slim AS runner
WORKDIR /app
COPY --from=builder /src/build-out/eprofiler-tui .
USER 1000:1000
ENTRYPOINT ["./eprofiler-tui"]
