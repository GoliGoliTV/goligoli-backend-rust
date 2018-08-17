# goligoli-backend-rust
**all
Goligoli backend implementation in rust

## Environments

- rust >= 1.26 (nightly channel)
- PostgreSQL

## Development

### Rustup

This service is built upon actix-web, and you will need Rust to get started with it.
If you don’t have it yet we recommend you use [rustup](https://rustup.rs/) to manage your Rust installation.
The [official rust guide](https://doc.rust-lang.org/book/second-edition/ch01-01-installation.html) has a wonderful section on getting started.

### Rust 2018 Edition (Temporarily on nightly channel)
**This server use rust 2018 edition.** 

Install nightly toolchain and set rustup to use nightly channel by default:
```bash
$ rustup install nightly
$ rustup default nightly
```

By now you have to install nightly channel to enable rust 2018 edition support. When Rust 2018 Edition is shipped in stable channel, we can delete the following lines in Cargo.toml:

    cargo-features = ["edition"]

### Start the server
```bash
$ cd goligoli-backend-rust
$ cargo run
```

## Docker
TODO

## Wiki
TODO

## License

This project is licensed under either of

- GNU General Public License v3.0, (LICENSE)
