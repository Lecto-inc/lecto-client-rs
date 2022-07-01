FROM rust:1.61.0-slim-buster AS base

# ローカルのtargetディレクトリにビルドするとマウントしている時に遅くなるのでビルドディレクトリを変える
ENV CARGO_TARGET_DIR=/tmp/target \
    DEBIAN_FRONTEND=noninteractive \
    LC_CTYPE=ja_JP.utf8 \
    LANG=ja_JP.utf8

RUN apt-get update \
  && apt-get install -y -q \
     ca-certificates \
     locales \
     gnupg \
     apt-transport-https\
     libssl-dev \
     pkg-config \
     curl \
     build-essential \
     git \
     wget \
     cmake \
  && echo "ja_JP UTF-8" > /etc/locale.gen \
  && locale-gen \
  && echo "install rust tools" \
  && rustup component add rustfmt \
  && cargo install cargo-watch cargo-make \
  && chmod go-w /usr/local/cargo /usr/local/cargo/bin

RUN USER=root cargo new --lib app
WORKDIR /app

COPY ./Cargo.toml Cargo.toml
COPY ./Cargo.lock Cargo.lock

# Development
FROM base AS dev

RUN cargo build --color never \
  && rm src/*.rs \
  && find $CARGO_TARGET_DIR/ -name "liblecto_client*" -delete \
  && find $CARGO_TARGET_DIR/ -name "lecto_client*" -prune -exec rm -rf {} +

COPY . /app
RUN cargo build

CMD ["cargo", "run"]

# Build binaries for production
FROM base AS build

ENV RUSTFLAGS="-C debuginfo=1"
RUN cargo build \
  --release --color never

COPY . /app
RUN cargo build \
  --release --color never \
  --bin lecto-client

# Production
FROM debian:buster-slim AS prd

RUN apt-get update \
  && apt-get install -y -q \
     ca-certificates \
     locales \
     gnupg \
     libssl-dev \
  && echo "ja_JP UTF-8" > /etc/locale.gen \
  && locale-gen

RUN mkdir -p /app/bin
WORKDIR /app

COPY --from=build /tmp/target/release/lecto-client /app/bin

CMD ["/app/bin/lecto-client"]
