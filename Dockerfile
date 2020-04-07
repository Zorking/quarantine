FROM rustlang/rust:nightly

RUN cargo install diesel_cli --no-default-features --features sqlite

RUN apt install sqlite3

COPY ./ /usr/src/app

WORKDIR /usr/src/app

RUN rustup default nightly

RUN diesel setup

RUN diesel migration run

RUN cargo build
