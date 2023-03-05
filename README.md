# Contained

[![crates.io](https://img.shields.io/crates/v/contained.svg)](https://crates.io/crates/contained)
[![docs.rs](https://docs.rs/contained/badge.svg)](https://docs.rs/contained)
[![Clippy](https://github.com/FL03/contained/actions/workflows/clippy.yml/badge.svg)](https://github.com/FL03/contained/actions/workflows/clippy.yml)
[![Docker](https://github.com/FL03/contained/actions/workflows/docker.yml/badge.svg)](https://github.com/FL03/contained/actions/workflows/docker.yml)
[![Rust](https://github.com/FL03/contained/actions/workflows/rust.yml/badge.svg)](https://github.com/FL03/contained/actions/workflows/rust.yml)

***

Contained is a research oriented project focusing on implementing the proposed harmonic computational framework. Contained considers a harmonic framework capable of efficiently orchestrating clusters of devices following a set of instructions broadcast from valid actors preserving only the I/O and required metadata. The metadata stored typically pertains to the temporality or ordering of events which lead to the completion of any particular task and generally distributed as a part of a unique proof. In order to do so, we consider the neo-Riemannian theory and its potential implications for computational systems. Consequentially, several more traditional notions of harmonic analysis are additionally introduced to complete the theorem and solidify the framework. The resulting compositional procedures suggest novel means of generating ephemeral computational spaces capable of supporting interactions across each of the four permutations facilitated by digital systems. Typically these spaces are leveraged in conjunction with one another to elegantly orchestrate complex workloads.

## Getting Started

### Building from the source

#### _Clone the repository_

```bash
git clone https://github.com/FL03/contained
```

### Docker

#### _Build the image locally_

```bash
docker buildx build --tag contained:alpha .
```

#### _Pull the pre-built image_

```bash
docker pull jo3mccain/contained:latest
```

#### _Run the image_

```bash
docker run -p 8080:8080 jo3mccain/contained:latest system --up
```

### Usage

```rust

```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

- [Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)
- [MIT](https://choosealicense.com/licenses/mit/)
