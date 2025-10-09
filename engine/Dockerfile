FROM rust:1.73 as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
# build
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/fmeda_engine /usr/local/bin/fmeda_engine
EXPOSE 8081
ENV RUST_LOG=info
CMD ["/usr/local/bin/fmeda_engine"]
