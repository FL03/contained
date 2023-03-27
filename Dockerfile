FROM rust:latest as base

RUN apt-get update -y && apt-get upgrade -y

FROM base as builder-base

RUN apt-get install -y \
    protobuf-compiler

RUN rustup update && \
    rustup default nightly && \
    rustup target add wasm32-unknown-unknown wasm32-wasi --toolchain nightly

FROM builder-base as builder

ENV CARGO_TERM_COLOR=always

ADD . /workspace
WORKDIR /workspace

COPY . .
RUN cargo build --release -v --workspace

FROM debian:buster-slim as runner-base

ENV RUST_LOG="info" \
    SERVER_PORT=8080

RUN apt-get update -y && apt-get upgrade -y

RUN apt-get install -y \
    protobuf-compiler

RUN mkdir data
VOLUME [ "/data" ]

COPY --chown=55 .config /config
VOLUME [ "/config" ]

COPY --chown=55 --from=builder /workspace/target/release/contained /bin/contained

FROM runner-base as runner

EXPOSE 80
EXPOSE ${SERVER_PORT}

ENTRYPOINT [ "contained" ]
CMD [ "-h" ]