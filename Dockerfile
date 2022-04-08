FROM rust:1.60-alpine

ENV LANG C.UTF-8
ENV APP_ROOT /var/app

RUN apk add openssl gcc g++

ENV DOCKERIZE_VERSION v0.6.1
RUN wget https://github.com/jwilder/dockerize/releases/download/$DOCKERIZE_VERSION/dockerize-alpine-linux-amd64-$DOCKERIZE_VERSION.tar.gz \
  && tar -C /usr/local/bin -xzvf dockerize-alpine-linux-amd64-$DOCKERIZE_VERSION.tar.gz \
  && rm dockerize-alpine-linux-amd64-$DOCKERIZE_VERSION.tar.gz

WORKDIR ${APP_ROOT}

RUN cargo install cargo-watch

CMD ["cargo", "watch", "-x", "'run --bin rust_api'"]
