FROM rust:latest

ENV USER=aias
RUN apt-get -y update && apt-get install -y openssl
ADD . /app
WORKDIR /app
RUN cargo build