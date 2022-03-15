FROM rust:slim as builder

WORKDIR /usr/src/shakespeareswords
COPY . .

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev

RUN cargo install --path .


FROM debian:buster-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev

COPY --from=builder /usr/local/cargo/bin/shakespeareswords /
COPY --from=builder /usr/src/shakespeareswords/templates /templates

WORKDIR /

CMD ["/shakespeareswords"]
