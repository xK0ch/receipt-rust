FROM rust:1.78.0

ENV APP_HOME=/root/dev/receipt-rust

RUN mkdir -p $APP_HOME/src

WORKDIR $APP_HOME

COPY ./Cargo.toml ./Cargo.lock ./.env $APP_HOME
COPY ./src $APP_HOME/src
COPY ./migrations $APP_HOME/migrations

RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/receipt-rust"]