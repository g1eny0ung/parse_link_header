# parse_link_header

[![Build Status](https://travis-ci.com/g1eny0ung/parse_link_header.svg?branch=master)](https://travis-ci.com/g1eny0ung/parse_link_header)
![Crates.io](https://img.shields.io/crates/v/parse_link_header)
![Crates.io](https://img.shields.io/crates/l/parse_link_header)

A library for parse http link header.

## How to use

### Note for version 0.1.x

The version 0.1 can't correctly handle the `relative ref` which described in <https://tools.ietf.org/html/rfc3986#section-4.1>

The parsed value of version 0.1 refers to the return value of <https://github.com/thlorenz/parse-link-header>, which is a `HashMap` with the same structure.

**So if you want to parse `relative ref`, please use version `0.2`.**

**Or if you don't care about `relative ref` and wanna simple `HashMap<String, HashMap<String, String>>` result, you can use version `0.1`.**

### Version 0.2.x

In your `Cargo.toml`, add:

```toml
[dependencies]
parse_link_header = "0.2"
```

Then:

```rust
let link_header = "<https://api.github.com/repositories/41986369/contributors?page=2>; rel=\"next\", <https://api.github.com/repositories/41986369/contributors?page=14>; rel=\"last\"";

parse_link_header::parse(link_header);
```

The parsed value is a `Result<HashMap<Option<Rel>, Link>, ()>`, which `Rel` and `Link` is:

```rust
use std::collections::HashMap;

use http::Uri;

#[derive(Debug, PartialEq)]
pub struct Link {
    pub uri: Uri, // https://docs.rs/http/0.2.1/http/uri/struct.Uri.html
    pub raw_uri: String,
    pub queries: HashMap<String, String>,
    pub params: HashMap<String, String>,
}

type Rel = String;
```

You can see why the key of `HashMap` is `Option<Rel>` because if you won't provide a `rel` type, the key will be an empty string.

Refer to <https://tools.ietf.org/html/rfc8288#section-3.3>, **the rel parameter MUST be present**.

Therefore, if you find that key is `None`, please check if you provide the `rel` type.

## How to contribute

Pull a request or open an issue to describe your changes or problems.

## License

MIT @ g1eny0ung
