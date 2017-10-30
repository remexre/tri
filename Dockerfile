FROM library/rust:latest AS build

WORKDIR /usr/src/tri
COPY . .

RUN cargo install diesel_cli
ENV DATABASE_URL=/usr/src/tri/tri.db
RUN diesel migration run

RUN cargo build --release
RUN cargo test --release

FROM debian:latest

COPY --from=build /usr/src/tri/target/release/tri /usr/local/bin/tri

VOLUME /data
WORKDIR /data
COPY --from=build /usr/src/tri/tri.db /data/tri.db

ENV DATABASE_URL=/data/tri.db
CMD ["tri"]
