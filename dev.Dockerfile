#FROM debian:bookworm-slim as base
FROM ubuntu:22.04 as base
#FROM ubuntu:18.04 AS base

#RUN apt-get -y update && apt-get install -y \

RUN  \
    --mount=type=cache,target=/var/cache/apt \
    apt-get -y update && apt-get install -y \
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
#RUN apt-get install -y \
#	make \
#	git \
#	pkg-config \
#	flex \
#	bison \
#    	protobuf-compiler  

RUN cd /nsjail && make && mv /nsjail/nsjail /bin && rm -rf -- /nsjail
FROM base as rust_cmp
COPY ./evaluator /app/evaluator
COPY ./primitypes /app/primitypes

FROM base as end
COPY --from=maker /lib/aarch64-linux-gnu/libprotobuf.so.23 /lib/aarch64-linux-gnu/libprotobuf.so.23 
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
ENV IS_PROD=true
ENV CONFIGURATION_DIRECTORY="configuration"
ENV CONFIGURATION_FILE="prod.yml"
COPY --from=maker /bin/nsjail /bin/nsjail
#ENTRYPOINT ["/app/evaluator/evaluator"]
