# Nanoid CLI

[![coverage](https://shields.io/endpoint?url=https://raw.githubusercontent.com/jlyonsmith/nanoid_cli/main/coverage.json)](https://github.com/jlyonsmith/nanoid_cli/blob/main/coverage.json)
[![Crates.io](https://img.shields.io/crates/v/nanoid_cli.svg)](https://crates.io/crates/nanoid_cli)
[![Docs.rs](https://docs.rs/nanoid_cli/badge.svg)](https://docs.rs/nanoid_cli)

This is a command line for the Rust crate [nanoid](https://crates.io/crates/nanoid), a tiny, secure, URL-friendly, unique string ID generator.  Nanoid has been ported to over 20 languages.  This project provides command line interface to the library so you can manually generate strings.  It provides the following options:

- Set the length of the ID
- Generate multiple ID's at once, one per line
- Use a variety of pre-defined alphabets:
  - **Decimal** using the digits 0 to 9
  - **Hexadecimal** using the digis 0 to 9 and A to F
  - **Alpha Numeric** using the characters 0 to 9 and A to Z
  - **Full Alpha Numeric** using the characters 0 to 9, A to Z and a to z.
  - **Secure URL** the default, URL safe alphabet

Install it with `cargo install nanoid_cli`.  Use `nanoid --help` to see the full list of options.

The tool uses the [clap](https://docs.rs/clap/latest/clap/index.html) crate for command line processing.

Pull requests welcomed for additional features, such a customizable alphabets.
