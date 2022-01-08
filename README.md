# parse_link_header

[![Rust](https://github.com/g1eny0ung/parse_link_header/actions/workflows/rust.yml/badge.svg)](https://github.com/g1eny0ung/parse_link_header/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/g1eny0ung/parse_link_header/branch/master/graph/badge.svg?token=ZEDQWONIIZ)](https://codecov.io/gh/g1eny0ung/parse_link_header)
![Crates.io](https://img.shields.io/crates/v/parse_link_header)
![Crates.io](https://img.shields.io/crates/l/parse_link_header)

A library for parse http link header.

<!-- toc -->

- [How to use](#how-to-use)
  - [Note for version 0.1.x](#note-for-version-01x)
  - [Version 0.2.x](#version-02x)
- [Feature: `url`](#feature-url)
- [How to contribute](#how-to-contribute)
- [License](#license)

<!-- tocstop -->

## How to use

### Note for version 0.1.x

The version 0.1 can't correctly handle the `relative ref` which described in
<https://tools.ietf.org/html/rfc3986#section-4.1>

The parsed value of version 0.1 refers to the return value of
<https://github.com/thlorenz/parse-link-header>, which is a `HashMap` with the
same structure.

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
let link_header = r#"<https://api.github.com/repositories/41986369/contributors?page=2>; rel="next", <https://api.github.com/repositories/41986369/contributors?page=14>; rel="last""#;

let res = parse_link_header::parse(link_header);
assert!(res.is_ok());

let val = res.unwrap();
assert_eq!(val.len(), 2);
assert_eq!(val.get(&Some("next".to_string())).unwrap().raw_uri, "https://api.github.com/repositories/41986369/contributors?page=2");
assert_eq!(val.get(&Some("last".to_string())).unwrap().raw_uri, "https://api.github.com/repositories/41986369/contributors?page=14");
```

The parsed value is a `Result<HashMap<Option<Rel>, Link>, Error>` (aka a
`LinkMap`, which `Rel` and `Link` is:

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

You can see why the key of `HashMap` is `Option<Rel>` because if you won't
provide a `rel` type, the key will be an empty string.

Refer to <https://tools.ietf.org/html/rfc8288#section-3.3> (October 2017),
**the rel parameter MUST be present**.

Therefore, if you find that key is `None`, please check if you provide the
`rel` type.

Alternatively, use the `parse_with_rel()` function to get a `HashMap<String, Link>` (aka a [`RelLinkMap`](type.RelLinkMap.html)), as in:

```rust
let link_header = r#"<https://api.github.com/repositories/41986369/contributors?page=2>; rel="next", <https://api.github.com/repositories/41986369/contributors?page=14>; rel="last""#;

let res = parse_link_header::parse_with_rel(link_header);
assert!(res.is_ok());

let val = res.unwrap();
assert_eq!(val.len(), 2);
assert_eq!(val.get("next").unwrap().raw_uri, "https://api.github.com/repositories/41986369/contributors?page=2");
assert_eq!(val.get("last").unwrap().raw_uri, "https://api.github.com/repositories/41986369/contributors?page=14");
```

## Feature: `url`

If you enable the `url` feature, the `uri` field of struct [`Link`](struct.Link.html) will be
of type url::Url from the [url crate](https://crates.io/crates/url), rather than the
`http::Uri` it normally is. This allows direct use of the `uri` field with other popular
crates that use `url`, such as [`reqwest`](https://crates.io/crates/reqwest).

**NOTE:** This implictly disabled support for relative refs, as URLs do not support relative
refs (whereas URIs do).

## How to contribute

Pull a request or open an issue to describe your changes or problems.

## License

MIT @ g1eny0ung
