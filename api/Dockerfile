FROM rust:1.60.0

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080

WORKDIR /app

RUN rustup default nightly

RUN cargo install cargo-watch

CMD ["cargo", "watch", "-x", "run"]
