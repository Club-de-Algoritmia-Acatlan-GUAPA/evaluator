FROM amd64/alpine:3.18 as base
#FROM ubuntu:18.04 AS base

#RUN apt-get -y update && apt-get install -y \
RUN apk add autoconf
RUN apk add bison 
RUN apk add flex 
RUN apk add g++ 
RUN apk add gcc 
RUN apk add git 
RUN apk add protobuf-dev 
RUN apk add libnl3-dev 
RUN apk add libtool 
RUN apk add make 
RUN apk add vim 
RUN apk add pkgconfig 
RUN apk add protoc
RUN apk add python3 
RUN apk add linux-headers
RUN apk add libevhtp-dev
RUN apk add bsd-compat-headers
#&& rm -rf /var/lib/apt/lists/*
RUN apk add --no-cache ca-certificates
COPY ./evaluator/nsjail /nsjail



FROM messense/rust-musl-cross:x86_64-musl as rust-chef

RUN cargo install cargo-chef
RUN rustup target add x86_64-unknown-linux-musl

RUN apt-get update -y \
    && apt-get install -y openssl ca-certificates \
    && apt-get install -y lld clang pkg-config -y\
    && apt-get install -y ca-certificates libssl-dev musl-dev musl-tools
WORKDIR /app

FROM rust-chef as rust-planner
#COPY ./evaluator .
COPY ./evaluator /app/evaluator
COPY ./primitypes /app/primitypes
WORKDIR /app/evaluator
RUN cargo chef prepare --recipe-path recipe.json

FROM rust-chef AS rust-builder
COPY --from=rust-planner /app/evaluator/recipe.json /app/evaluator/recipe.json
COPY ./evaluator /app/evaluator
COPY ./primitypes /app/primitypes
WORKDIR /app/evaluator
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY ./evaluator /app/evaluator
COPY ./primitypes /app/primitypes
WORKDIR /app/evaluator
RUN cargo build --release --target x86_64-unknown-linux-musl --bin evaluator

FROM base AS maker

RUN cd /nsjail && make && mv /nsjail/nsjail /bin && rm -rf -- /nsjail




FROM amd64/alpine:3.18 as fin
#FROM scratch as fin
#COPY /usr/bin/python3 /python3
RUN apk add --no-cache python3 g++
COPY --from=maker /bin/nsjail /bin/nsjail
COPY --from=maker /lib/ld-musl-x86_64.so.1     /lib/ld-musl-x86_644.so.1 
COPY --from=maker /usr/lib/libprotobuf.so.32    /usr/lib/libprotobuf.so.32 
COPY --from=maker /usr/lib/libnl-route-3.so.200 /usr/lib/libnl-route-3.so.200 
COPY --from=maker /usr/lib/libnl-3.so.200       /usr/lib/libnl-3.so.200 
COPY --from=maker /usr/lib/libstdc++.so.6       /usr/lib/libstdc++.so.6 
COPY --from=maker /lib/libz.so.1                /lib/libz.so.1 
COPY --from=maker /usr/lib/libgcc_s.so.1        /usr/lib/libgcc_s.so.1 
COPY --from=rust-builder /app/evaluator/target/x86_64-unknown-linux-musl/release/evaluator /evaluator
COPY ./evaluator /eval
WORKDIR /eval/

ENV IS_PROD true
ENV CONFIGURATION_DIRECTORY configuration
ENV CONFIGURATION_FILE prod.yml

#ENTRYPOINT ["/evaluator"]
EXPOSE 5672
EXPOSE 6379

FROM amd64/alpine:3.18 as test_dev
RUN apk add --no-cache python3 g++ rust
COPY --from=maker /bin/nsjail /bin/nsjail
COPY --from=maker /lib/ld-musl-x86_64.so.1     /lib/ld-musl-x86_644.so.1 
COPY --from=maker /usr/lib/libprotobuf.so.32    /usr/lib/libprotobuf.so.32 
COPY --from=maker /usr/lib/libnl-route-3.so.200 /usr/lib/libnl-route-3.so.200 
COPY --from=maker /usr/lib/libnl-3.so.200       /usr/lib/libnl-3.so.200 
COPY --from=maker /usr/lib/libstdc++.so.6       /usr/lib/libstdc++.so.6 
COPY --from=maker /lib/libz.so.1                /lib/libz.so.1 
COPY --from=maker /usr/lib/libgcc_s.so.1        /usr/lib/libgcc_s.so.1 
COPY ./evaluator /app/evaluator
COPY ./primitypes /app/primitypes
COPY  ./evaluator/pypy/bin /bin
COPY  ./evaluator/pypy/lib /lib
WORKDIR /app



#COPY ./script.py script.py
#COPY ./run.sh run.sh
#ENTRYPOINT ["/bin/nsjail", "-Mo","--user", "2000", "--group","99999", "-R", "/code/", "-R","/lib/ld-musl-aarch64.so.1", "-R","/usr/bin/", "-R","/usr/lib/libpython3.9.so.1.0", "-R","/lib/ld-musl-aarch64.so.1", "-R","/usr/lib/python3.9", "--time_limit","1", "--disable_proc", "--keep_caps", "--", "/usr/bin/python3", "/code/script.py","<","/code/b.in"]
#COPY --from=maker /usr/lib/python3.6 /usr/lib/python3.6
#COPY --from=maker /lib/aarch64-linux-gnu/ /lib/aarch64-linux-gnu/
#CMD ["python3", "script.py", "
