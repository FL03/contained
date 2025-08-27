# Quickstart Guide `contained`

Welcome to the quickstart guide for the `contained` library! This guide will help you get started with the library and provide you with the necessary steps to set up your development environment.

## Getting Started

### Prerequisites

Before you can start using the `contained` library, you need to have the following tools installed on your machine:

- [Git](https://git-scm.com/downloads) - for version control and cloning the repository.
- [Rust](https://www.rust-lang.org/tools/install) - the programming language used to develop the `contained` library.
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) - the Rust package manager and build system, which is included with Rust.

Optionally, you may also want to install the following tools for development:

- [Visual Studio Code](https://code.visualstudio.com/) - a popular code editor with Rust support.
- [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) - a language server for Rust that provides features like code completion, go to definition, and more.
- [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) - a tool to streamline the installation of Rust binaries.

#### Rust

If you don't have rustup installed, you can install it by following the instructions on the [official website](https://www.rust-lang.org/tools/install).

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

##### _rustup_

Once installed, you can use rustup to manage your Rust toolchain. This includes installing the latest stable version of Rust, as well as any other versions you may need for your projects. For now, we simply recommend using the latest stable version of Rust and making sure that any other toolchains you may have installed are up to date.

```bash
rustup update
```

### Building from the source

Start by cloning the repository locally to your machine:

```bash
git clone https://github.com/FL03/contained.git --branch main
```

Then, navigate to the cloned directory:

```bash
cd contained
```

To build the project, use `cargo build` command:

```bash
cargo build --all-features --locked --release --workspace
```

Test the project using the `cargo test` command:

```bash
cargo test --all-features --locked --release --workspace
```

Or, benchmark the project using the `cargo bench` command:

```bash
cargo bench --all-features --verbose --workspace
```
