FROM rust:buster

RUN mkdir app
WORKDIR /app
COPY common ./common
COPY db ./db
COPY distributor ./distributor
COPY fluid ./fluid
COPY generator ./generator
COPY processor ./processor
COPY Cargo.lock ./
COPY Cargo.toml ./
COPY run.sh ./
COPY config.json ./
RUN cargo build
ENTRYPOINT ["run.sh"]