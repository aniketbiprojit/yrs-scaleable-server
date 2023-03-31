FROM public.ecr.aws/docker/library/rust:1.68.0-alpine3.17

WORKDIR /app

COPY . /app

EXPOSE 2000

ENV USE_VAULT_SECRETS=true
ENV NO_COLOR=true

RUN apk add --no-cache musl-dev

RUN cargo build --release --bin core-server

CMD ["cargo","run","--release","--bin","core-server"]