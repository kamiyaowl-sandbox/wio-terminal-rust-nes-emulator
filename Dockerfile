FROM rust:1.37.0-buster

RUN apt-get update \
  && apt-get install -y git libgtk-3-dev

# for wasm
RUN apt-get install -y nodejs npm
RUN npm install -g n
RUN n 10.15.1
RUN cargo install wasm-pack

RUN mkdir /work
WORKDIR /work

CMD ["/bin/sh"]