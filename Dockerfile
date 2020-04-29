#FROM rust:1.43
FROM rustlang/rust:nightly-slim

RUN apt update \
    && apt install -y --no-install-recommends libpq5 libpq-dev \
    &&  rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

RUN cargo install --path .

CMD ["rocket-api"]
