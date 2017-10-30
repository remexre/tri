FROM library/rust:latest AS build

WORKDIR /usr/src/tri
COPY . .
RUN cargo build --release
RUN cargo test --release

FROM debian:latest;

COPY --from=build /usr/src/tri/target/release/tri /usr/local/bin/tri

VOLUME /data
WORKDIR /data
COPY .env /data/.env

CMD ["tri"]
