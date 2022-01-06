FROM crystallang/crystal:1.3.0-alpine-build

WORKDIR /app

COPY ./ ./

RUN shards install

RUN crystal spec

RUN shards build
