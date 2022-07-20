FROM rust:1.62-slim

ENV LANG C.UTF-8
ENV APP_ROOT /var/app

RUN apt-get update -y && \
  apt-get install -y curl libssl-dev pkg-config

ENV DOCKERIZE_VERSION v0.6.1
RUN curl -LO https://github.com/jwilder/dockerize/releases/download/$DOCKERIZE_VERSION/dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz \
  && tar -C /usr/local/bin -xzvf dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz \
  && rm dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz

WORKDIR ${APP_ROOT}

RUN cargo install cargo-watch sea-orm-cli

CMD ["cargo", "watch", "-x", "'run --bin rust_api'"]
