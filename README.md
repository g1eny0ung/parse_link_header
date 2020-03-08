# parse_link_header

[![Build Status](https://travis-ci.com/g1eny0ung/parse_link_header.svg?branch=master)](https://travis-ci.com/g1eny0ung/parse_link_header)
![Crates.io](https://img.shields.io/crates/v/parse_link_header)

A library for parse http link header.

## How to use

In your `Cargo.toml`, add:

```toml
[dependencies]
parse_link_header = "0.1"
```

Then:

```rust
let link_header = "<https://api.github.com/repositories/41986369/contributors?page=2>; rel=\"next\", <https://api.github.com/repositories/41986369/contributors?page=14>; rel=\"last\"";

parse_link_header::parse(link_header);
```

## How to contribute

Pull a request or open an issue to describe your changes or problems.

## License

MIT @ g1eny0ung
