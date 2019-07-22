FROM rust:1.36-stretch

RUN apt-get -y -q update \
  && apt-get install -y -q \
     curl \
     gnupg \
     apt-transport-https\
  && curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add - \
  && echo "deb https://dl.yarnpkg.com/debian/ stable main" | tee /etc/apt/sources.list.d/yarn.list \
  && curl -sL https://deb.nodesource.com/setup_10.x | bash - \
  && apt-get install -y -q \
     nodejs \
     yarn

ENV CARGO_BUILD_TARGET_DIR=/tmp/target

RUN USER=root cargo new --bin app
WORKDIR /app
COPY ./Cargo.toml Cargo.toml
COPY ./Cargo.lock Cargo.lock

RUN cargo build --release --color never && \
    rm src/*.rs
