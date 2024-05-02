FROM ubuntu:latest as base

RUN apt-get update
RUN apt-get install -y \
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
    ca-certificates
    #&&  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

COPY ./evaluator/nsjail /nsjail

FROM base AS maker
RUN cd /nsjail && make && mv /nsjail/nsjail /bin && rm -rf -- /nsjail

FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
#RUN rustup target add x86_64-unknown-linux-gnu
RUN apt-get update -y
RUN apt-get install -y openssl ca-certificates \
 pkg-config \
gcc-x86-64-linux-gnu
#gcc-multilib
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
RUN cargo chef cook --recipe-path recipe.json

COPY ./evaluator /app/evaluator
RUN cargo build --bin evaluator


FROM ubuntu:latest as end
RUN apt-get update
RUN apt-get install -y \
    gcc \
    g++ \
    python3 \
    default-jre \
    default-jdk

#COPY --from=maker /lib/x86_64-linux-gnu/libprotobuf.so.32 /lib/x86_64-linux-gnu/libprotobuf.so.32 
#COPY --from=maker /lib/x86_64-linux-gnu/libprotobuf.so.23 /lib/x86_64-linux-gnu/libprotobuf.so.23 
#COPY --from=maker /lib/x86_64-linux-gnu/libnl-route-3.so.200 /lib/x86_64-linux-gnu/libnl-route-3.so.200
#COPY --from=maker /lib/x86_64-linux-gnu/libnl-3.so.200  /lib/x86_64-linux-gnu/libnl-3.so.200  
#COPY --from=maker /lib/x86_64-linux-gnu/libstdc++.so.6 /lib/x86_64-linux-gnu/libstdc++.so.6 
#COPY --from=maker /lib/x86_64-linux-gnu/libc.so.6  /lib/x86_64-linux-gnu/libc.so.6  
#COPY --from=maker /lib/x86_64-linux-gnu/libz.so.1 /lib/x86_64-linux-gnu/libz.so.1
#COPY --from=maker /lib64/ld-linux-x86-64.so.2 /lib64/ld-linux-x86-64.so.2 
#COPY --from=maker /lib/x86_64-linux-gnu/libgcc_s.so.1 /lib/x86_64-linux-gnu/libgcc_s.so.1 
#COPY --from=maker /lib/x86_64-linux-gnu/libm.so.6 /lib/x86_64-linux-gnu/libm.so.6 

ENV IS_PROD=true
ENV CONFIGURATION_DIRECTORY="/app/evaluator/configuration"
ENV CONFIGURATION_FILE="prod.yml"

COPY --from=builder /app/evaluator/target/x86_64-unknown-linux-gnu/debug/evaluator /app/evaluator/evaluator
COPY --from=maker /bin/nsjail /bin/nsjail

COPY ./evaluator/playground /app/evaluator/playground
COPY ./evaluator/resources /app/evaluator/resources

WORKDIR /app/evaluator
ENTRYPOINT ["/app/evaluator/evaluator"]


