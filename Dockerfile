# Use the official Rust image as a builder
FROM rust:slim-bookworm as builder

WORKDIR /usr/src/md_cat
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m md_cat
WORKDIR /home/md_cat
USER md_cat

COPY --from=builder /usr/src/md_cat/target/release/md_cat /usr/local/bin/md_cat

ENTRYPOINT ["md_cat"]
CMD ["--help"]