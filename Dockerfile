FROM rust:1.58.1

WORKDIR /app

ENV OPENSSL_VERSION=1.1.1m
ENV OPENSSL_LIB_DIR=/tmp/openssl-${OPENSSL_VERSION}
ENV OPENSSL_INCLUDE_DIR=/tmp/openssl-${OPENSSL_VERSION}/include

ENV MACHINE=armv7
ENV ARCH=arm
ENV CC=arm-linux-gnueabihf-gcc

RUN \
  apt-get update && \
  apt-get install -y gcc-arm-linux-gnueabihf wget

RUN \
  cd /tmp && \
  wget https://www.openssl.org/source/openssl-${OPENSSL_VERSION}.tar.gz && \
  tar xzf openssl-${OPENSSL_VERSION}.tar.gz && cd openssl-${OPENSSL_VERSION} && \
  ./config shared && \
  make -j 16

RUN \
  rustup target add armv7-unknown-linux-gnueabihf

CMD ["cargo", "build", "--target", "armv7-unknown-linux-gnueabihf", "--release"]