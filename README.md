# contained

[![crates.io](https://img.shields.io/crates/v/contained?style=for-the-badge&logo=rust)](https://crates.io/crates/contained)
[![docs.rs](https://img.shields.io/docsrs/contained?style=for-the-badge&logo=docs.rs)](https://docs.rs/contained)
[![GitHub License](https://img.shields.io/github/license/fl03/contained?style=for-the-badge&logo=github)](./LICENSE)

***

Welcome to `contained`, a collection of macros and other utilities designed to facilitate the creation and manipulation of so-called wrapper types in Rust. Here, a wrapper type is essentially any object capable of implementing the `#[repr(transparent)]` attribute, such as newtypes, tuple structs, and single-field enums.

## Usage

Before you start using `contained`, make sure to add it as a dependency in your `Cargo.toml` file. You can do this by adding the following lines:

```toml
[dependencies.contained]
features = [
    "derive",
]
version = "0.2.x"
```

### Examples

For more detailed examples, please visit the [examples](https://github.com/FL03/contained/tree/main/contained/examples) directory in the repository. Below are some brief examples highlighting certain features of the library.

## Getting Started

To get started with `contained`, you can check out the [QUICKSTART.md](QUICKSTART.md) file, which provides a step-by-step guide on how to set up your development environment and start using the library.

## License

Licensed under the Apache License, Version 2.0, ([LICENSE-APACHE](http://www.apache.org/licenses/LICENSE-2.0))

## Contribution

Contributions are welcome, however, ensure that you have read the [CONTRIBUTING.md](CONTRIBUTING.md) file before submitting a pull request.
