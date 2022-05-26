FROM rust:latest

# Install nodejs
RUN curl -sL https://deb.nodesource.com/setup_12.x | bash -
RUN apt-get update


RUN rustup install stable
RUN rustup default stable
RUN rustup target add wasm32-unknown-unknown

RUN cargo install trunk

# Install wasm-pack
# RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

WORKDIR /app

COPY . .

# COPY .env.example .env

EXPOSE 8080

CMD [ "trunk", "serve", "--watch", ".", "--address", "0.0.0.0", "--port", "8080" ]
