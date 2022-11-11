FROM scsys/rust:debian-lts as base-image

RUN apt-get update -y && apt-get upgrade -y

FROM base-image as builder

ENV COLOR=always

ADD . /workspace
WORKDIR /workspace

COPY . .
RUN cargo build --color ${COLOR} --release --verbose --workspace && \
    cargo test --all --all-features

FROM photon:latest as runner

RUN yum update -y && yum upgrade -y

RUN yum install -y \
    build-essential \
    libcrypto \
    libssl \
    libtls

COPY --from=builder /workspace/target/release/conduit /bin/conduit

CMD ["conduit"]