FROM scsys/rust:debian-lts as base-image

RUN apt-get update -y && apt-get upgrade -y

FROM base-image as builder

ENV COLOR=always

ADD . /workspace
WORKDIR /workspace

COPY . .
RUN cargo build --color ${COLOR} --release --verbose --workspace && \
    cargo test --all --all-features

FROM debian:latest as runner-base

ENV RUST_LOG = "info" \
    SERVER_PORT=8080

RUN apt-get update -y && apt-get upgrade -y

RUN mkdir config
VOLUME [ "/config" ]

FROM runner-base as runner 

RUN apt-get install -y \
    openssl

COPY --chown=55 conduit/Conduit.toml /config/Conduit.toml
COPY --from=builder /workspace/target/release/conduit /bin/conduit

FROM runner

EXPOSE 80
EXPOSE ${SERVER_PORT}

ENTRYPOINT [ "conduit" ]

CMD [""]