#FROM debian:bookworm-slim as base
FROM rust:slim-bookworm as base
FROM rust:slim-bookworm as base
#FROM ubuntu:18.04 AS base

#RUN apt-get -y update && apt-get install -y \


RUN apt-get -y update && apt-get install -y \
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
    python3
#&& rm -rf /var/lib/apt/lists/*
RUN  apt-get install -y vim
#&& rm -rf /var/lib/apt/lists/*
RUN apt-get install -y ca-certificates
COPY ./evaluator/nsjail /nsjail

FROM base AS maker

RUN cd /nsjail && make && mv /nsjail/nsjail /bin && rm -rf -- /nsjail

FROM base as end
COPY --from=maker /lib/aarch64-linux-gnu/libprotobuf.so.32 /lib/aarch64-linux-gnu/libprotobuf.so.32 
COPY --from=maker /lib/aarch64-linux-gnu/libnl-route-3.so.200 /lib/aarch64-linux-gnu/libnl-route-3.so.200
COPY --from=maker /lib/aarch64-linux-gnu/libnl-3.so.200  /lib/aarch64-linux-gnu/libnl-3.so.200  
COPY --from=maker /lib/aarch64-linux-gnu/libstdc++.so.6 /lib/aarch64-linux-gnu/libstdc++.so.6 
COPY --from=maker /lib/aarch64-linux-gnu/libc.so.6  /lib/aarch64-linux-gnu/libc.so.6  
COPY --from=maker /lib/aarch64-linux-gnu/libz.so.1 /lib/aarch64-linux-gnu/libz.so.1
COPY --from=maker /lib/ld-linux-aarch64.so.1 /lib/ld-linux-aarch64.so.1 
COPY --from=maker /lib/aarch64-linux-gnu/libgcc_s.so.1 /lib/aarch64-linux-gnu/libgcc_s.so.1 
COPY --from=maker /lib/aarch64-linux-gnu/libm.so.6 /lib/aarch64-linux-gnu/libm.so.6 
COPY ./evaluator /app/evaluator
COPY ./primitypes /app/primitypes
COPY --from=maker /bin/nsjail /bin/nsjail
#
#RUN cargo install cargo-chef
#RUN rustup target add aarch64-unknown-linux-gnu	
#
#RUN apt-get update -y \
#    && apt-get install -y openssl ca-certificates \
#    && apt-get install -y lld clang pkg-config -y\
#    && apt-get install -y ca-certificates libssl-dev musl-dev musl-tools
#WORKDIR /app
#
#FROM rust-chef as rust-planner
##COPY ./evaluator .
#COPY ./evaluator /app/evaluator
#COPY ./primitypes /app/primitypes
#WORKDIR /app/evaluator
#RUN cargo chef prepare --recipe-path recipe.json
#
#FROM rust-chef AS rust-builder
#COPY --from=rust-planner /app/evaluator/recipe.json /app/evaluator/recipe.json
#COPY ./evaluator /app/evaluator
#COPY ./primitypes /app/primitypes
#WORKDIR /app/evaluator
#RUN cargo chef cook --release --target aarch64-unknown-linux-gnu --recipe-path recipe.json
#COPY ./evaluator /app/evaluator
#COPY ./primitypes /app/primitypes
#WORKDIR /app/evaluator
#RUN cargo build --release --target aarch64-unknown-linux-gnu --bin evaluator
#
#FROM base AS maker
#
#RUN cd /nsjail && make && mv /nsjail/nsjail /bin && rm -rf -- /nsjail
#
#
#FROM amd64/alpine:3.18 as fin
##FROM scratch as fin
##COPY /usr/bin/python3 /python3
#RUN apk add python3 g++
#COPY --from=maker /bin/nsjail /bin/nsjail
#COPY --from=maker /lib/ld-musl-x86_64.so.1     /lib/ld-musl-x86_644.so.1 
#COPY --from=maker /usr/lib/libprotobuf.so.32    /usr/lib/libprotobuf.so.32 
#COPY --from=maker /usr/lib/libnl-route-3.so.200 /usr/lib/libnl-route-3.so.200 
#COPY --from=maker /usr/lib/libnl-3.so.200       /usr/lib/libnl-3.so.200 
#COPY --from=maker /usr/lib/libstdc++.so.6       /usr/lib/libstdc++.so.6 
#COPY --from=maker /lib/libz.so.1                /lib/libz.so.1 
#COPY --from=maker /usr/lib/libgcc_s.so.1        /usr/lib/libgcc_s.so.1 
#COPY --from=rust-builder /app/evaluator/target/x86_64-unknown-linux-musl/release/evaluator /evaluator
#COPY ./evaluator /eval
#WORKDIR /eval/
#
#ENV IS_PROD true
#ENV CONFIGURATION_DIRECTORY configuration
#ENV CONFIGURATION_FILE prod.yml
#
##ENTRYPOINT ["/evaluator"]
#EXPOSE 5672
#EXPOSE 6379
#
#FROM arm64v8/alpine:3.18 as test_dev
#RUN apk add python3 g++ 
#COPY --from=maker /bin/nsjail /bin/nsjail
#COPY --from=maker /lib/ld-musl-aarch64.so.1     /lib/ld-musl-aarch64.so.1 
#COPY --from=maker /usr/lib/libprotobuf.so.32    /usr/lib/libprotobuf.so.32 
#COPY --from=maker /usr/lib/libnl-route-3.so.200 /usr/lib/libnl-route-3.so.200 
#COPY --from=maker /usr/lib/libnl-3.so.200       /usr/lib/libnl-3.so.200 
#COPY --from=maker /usr/lib/libstdc++.so.6       /usr/lib/libstdc++.so.6 
#COPY --from=maker /lib/libz.so.1                /lib/libz.so.1 
#COPY --from=maker /usr/lib/libgcc_s.so.1        /usr/lib/libgcc_s.so.1 
#COPY ./evaluator /app/evaluator
#COPY ./primitypes /app/primitypes
#
#RUN apk add \
#        ca-certificates \
#        gcc \
#        perl \
#        make
#
#
#
#ENV RUSTUP_HOME=/usr/local/rustup \
#    CARGO_HOME=/usr/local/cargo \
#    PATH=/usr/local/cargo/bin:$PATH \
#    RUST_VERSION=1.72.0
#
#ENV IS_PROD true
#RUN set -eux; \
#    apkArch="$(apk --print-arch)"; \
#    case "$apkArch" in \
#        x86_64) rustArch='x86_64-unknown-linux-musl'; rustupSha256='7aa9e2a380a9958fc1fc426a3323209b2c86181c6816640979580f62ff7d48d4' ;; \
#        aarch64) rustArch='aarch64-unknown-linux-musl'; rustupSha256='b1962dfc18e1fd47d01341e6897cace67cddfabf547ef394e8883939bd6e002e' ;; \
#        *) echo >&2 "unsupported architecture: $apkArch"; exit 1 ;; \
#    esac; \
#    url="https://static.rust-lang.org/rustup/archive/1.26.0/${rustArch}/rustup-init"; \
#    wget "$url"; \
#    echo "${rustupSha256} *rustup-init" | sha256sum -c -; \
#    chmod +x rustup-init; \
#    ./rustup-init -y --no-modify-path --profile minimal --default-toolchain $RUST_VERSION --default-host ${rustArch}; \
#    rm rustup-init; \
#    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
#    rustup --version; \
#    cargo --version; \
#    rustc --version;
#
#
#
##COPY ./script.py script.py
##COPY ./run.sh run.sh
##ENTRYPOINT ["/bin/nsjail", "-Mo","--user", "2000", "--group","99999", "-R", "/code/", "-R","/lib/ld-musl-aarch64.so.1", "-R","/usr/bin/", "-R","/usr/lib/libpython3.9.so.1.0", "-R","/lib/ld-musl-aarch64.so.1", "-R","/usr/lib/python3.9", "--time_limit","1", "--disable_proc", "--keep_caps", "--", "/usr/bin/python3", "/code/script.py","<","/code/b.in"]
##COPY --from=maker /usr/lib/python3.6 /usr/lib/python3.6
##COPY --from=maker /lib/aarch64-linux-gnu/ /lib/aarch64-linux-gnu/
##CMD ["python3", "script.py", "
