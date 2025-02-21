FROM rust:1.80.1

RUN cargo install sqlx-cli

WORKDIR /usr/src/currency-api

COPY Cargo.toml Cargo.lock /usr/src/currency-api/

CMD [ "cargo", "run", "--package", "currency_api" ]