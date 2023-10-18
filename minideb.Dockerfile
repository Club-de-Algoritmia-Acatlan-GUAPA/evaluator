#FROM debian:bookworm-slim as base
FROM bitnami/minideb:latest as base
#FROM ubuntu:18.04 AS base

#RUN apt-get -y update && apt-get install -y \

RUN install_packages \
    autoconf \
    bison \
    flex \
    gcc \
    g++ \
    git \
    libprotobuf-dev \
    libnl-route-3-dev \
    libtool \
    make \
    pkg-config \
    protobuf-compiler \
    python3 \
    curl \
    default-jre \
    default-jdk \
    vim \
    ca-certificates \
    &&  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

COPY ./evaluator/nsjail /nsjail

FROM base AS maker
RUN cd /nsjail && make && mv /nsjail/nsjail /bin && rm -rf -- /nsjail

FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY ./evaluator /app/evaluator
COPY ./primitypes /app/primitypes
WORKDIR /app/evaluator
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/evaluator/recipe.json /app/evaluator/recipe.json

WORKDIR /app/evaluator
COPY ./primitypes /app/primitypes
RUN cargo chef cook --release --recipe-path recipe.json

COPY ./evaluator /app/evaluator
RUN cargo build --release --bin evaluator


FROM bitnami/minideb:latest as end
RUN install_packages \
    gcc \
    g++ \
    python3 \
    default-jre \
    default-jdk

COPY --from=maker /lib/aarch64-linux-gnu/libprotobuf.so.32 /lib/aarch64-linux-gnu/libprotobuf.so.32 
COPY --from=maker /lib/aarch64-linux-gnu/libnl-route-3.so.200 /lib/aarch64-linux-gnu/libnl-route-3.so.200
COPY --from=maker /lib/aarch64-linux-gnu/libnl-3.so.200  /lib/aarch64-linux-gnu/libnl-3.so.200  
COPY --from=maker /lib/aarch64-linux-gnu/libstdc++.so.6 /lib/aarch64-linux-gnu/libstdc++.so.6 
COPY --from=maker /lib/aarch64-linux-gnu/libc.so.6  /lib/aarch64-linux-gnu/libc.so.6  
COPY --from=maker /lib/aarch64-linux-gnu/libz.so.1 /lib/aarch64-linux-gnu/libz.so.1
COPY --from=maker /lib/ld-linux-aarch64.so.1 /lib/ld-linux-aarch64.so.1 
COPY --from=maker /lib/aarch64-linux-gnu/libgcc_s.so.1 /lib/aarch64-linux-gnu/libgcc_s.so.1 
COPY --from=maker /lib/aarch64-linux-gnu/libm.so.6 /lib/aarch64-linux-gnu/libm.so.6 

ENV IS_PROD=true
ENV CONFIGURATION_DIRECTORY="/app/evaluator/configuration"
ENV CONFIGURATION_FILE="prod.yml"

COPY --from=builder /app/evaluator/target/release/evaluator /app/evaluator/evaluator
COPY --from=maker /bin/nsjail /bin/nsjail

COPY ./evaluator/playground /app/evaluator/playground
COPY ./evaluator/resources /app/evaluator/resources

WORKDIR /app/evaluator
ENTRYPOINT ["/app/evaluator/evaluator"]
