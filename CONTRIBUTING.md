# Contribute

- [Getting Started](#getting-started)
  - [Requirements](#requirements)
  - [First setup](#first-setup)
  - [Optional requirements](#optional-requirements)
- [Local Development](#local-development)
  - [Test](#test)
- [Documentation](#documentation)
  - [Readme](#readme)
- [Code of Conduct](#code-of-conduct)
- [Commit Message Format](#commit-message-format)

---

## Getting Started

### Requirements

- [Rust](https://www.rust-lang.org/tools/install): v1

### First setup

> [!IMPORTANT]  
> If you're under **Windows**, you nust run all CLI commands under a Linux shell-like terminal (i.e.: WSL or Git Bash).

Then run:

```sh
git clone https://github.com/ivangabriele/mistralai-client-rs.git # or your fork
cd ./mistralai-client-rs
cargo build
cp .env.example .env
```

Then edit the `.env` file to set your `MISTRAL_API_KEY`.

> [!NOTE]
> All tests use either the `open-mistral-7b` or `mistral-embed` models and only consume a few dozen tokens.
> So you would have to run them thousands of times to even reach a single dollar of usage.

### Optional requirements

- [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov?tab=readme-ov-file#installation) for `make test-cover`
- [cargo-watch](https://github.com/watchexec/cargo-watch#install) for `make test-watch`.

## Local Development

### Test

```sh
make test
```

or

```sh
make test-watch
```

## Documentation

### Readme

> [!IMPORTANT]
> Do not edit the `README.md` file directly. It is generated from the `README.template.md` file.

1. Edit the `README.template.md` file.
2. Run `make readme` to generate/update the `README.md` file.

## Code of Conduct

Help us keep this project open and inclusive. Please read and follow our [Code of Conduct](./CODE_OF_CONDUCT.md).

## Commit Message Format

This repository follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification.
