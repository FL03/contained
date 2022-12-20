FROM scratch as workspace

RUN mkdir workspace
VOLUME [ "/workspace" ]

FROM rust:latest as base

RUN apt-get update -y && apt-get upgrade -y

FROM base as builder-base

FROM builder-base as builder

ENV CARGO_TERM_COLOR=always

ADD . /workspace
WORKDIR /workspace

COPY . .
RUN cargo build --release -v --workspace

FROM workspace

COPY --chown=55 Conduit.toml /workspace/config/Conduit.toml
COPY --from=builder /workspace/target/release/conduit /workspace/conduit

CMD [ "ls" ]