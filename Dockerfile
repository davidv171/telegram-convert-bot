FROM rust:latest as build

WORKDIR /app

COPY . .

RUN cargo build --release

CMD ["./target/release/simple-unit-converter"]
