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

RUN apt-get update -y && apt-get upgrade -y

FROM runner-base as runner 

RUN apt-get install -y \
    openssl

COPY --from=builder /workspace/target/release/conduit /bin/conduit

FROM runner

CMD ["conduit"]