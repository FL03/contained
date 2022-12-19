FROM nixos/nix:latest as builder-base

ADD . /workspace
WORKDIR /workspace

COPY . .

RUN nix flake update

RUN nix shell

FROM builder-base as builder

ENTRYPOINT [ "nix-shell" ]
