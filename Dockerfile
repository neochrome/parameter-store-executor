FROM crystallang/crystal:1.2.2-alpine-build

WORKDIR /app

COPY ./ ./

RUN shards install

RUN crystal spec

RUN shards build
