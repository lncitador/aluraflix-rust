FROM rust:1-slim-buster AS BUILD

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:buster-slim

COPY --from=BUILD /app/target/release/aluraflix_rust /usr/local/bin/aluraflix_rust

CMD ["aluraflix_rust"]