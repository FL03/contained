# Contained

[![Clippy](https://github.com/FL03/contained/actions/workflows/clippy.yml/badge.svg)](https://github.com/FL03/contained/actions/workflows/clippy.yml)
[![Docker](https://github.com/FL03/contained/actions/workflows/docker.yml/badge.svg)](https://github.com/FL03/contained/actions/workflows/docker.yml)
[![Nightly](https://github.com/FL03/contained/actions/workflows/nightly.yml/badge.svg)](https://github.com/FL03/contained/actions/workflows/nightly.yml)
[![Rust](https://github.com/FL03/contained/actions/workflows/rust.yml/badge.svg)](https://github.com/FL03/contained/actions/workflows/rust.yml)

***

Contained is a simple command line interface designed for testing the proposed harmonic runtime which leverages the Neo-Riemannian Theory as a means of preserving the I/O from remote surfaces transformed according to some set of LPR transformations. This theory directly ties into to our desire to implement a dynamic encryption protocol combining fully-homomorphic encryption mechanisms with elliptic-curve cryptography, something more formally discussed under the guise of [aqueduct](https://github.com/scattered-systems/aqueduct).

## Concepts

### The Neo-Riemannian Theory

The Neo-Riemannian Theory is a set of theoretical approaches to the study of harmony and tonal organization in Western music. It was developed in the early 2000s by a group of scholars and theorists, and is based on nineteenth-century German music theorist Hugo Riemann’s theories of tonal space. Neo-Riemannian theorists have developed a number of analytical tools to study and describe the relationships between chords and keys in music. These tools include the concept of “Voice-Leading Spaces”, which is a visual representation of all the possible transitions between chords. Neo-Riemannian Theory is a powerful tool for understanding the harmonic structure of music, and has been used to explain a wide variety of musical styles, from classical to jazz and popular music.

#### _Tonnetz_

#### _Triads_

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

* [Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)
* [MIT](https://choosealicense.com/licenses/mit/)
