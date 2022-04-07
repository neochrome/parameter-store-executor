FROM crystallang/crystal:1.4.0-alpine-build

WORKDIR /app

COPY ./ ./

RUN shards install

RUN crystal spec

RUN shards build
