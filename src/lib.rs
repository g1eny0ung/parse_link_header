//! A library for parse http link header.
//!
//! # How to use
//!
//! ### Note for version 0.1.x
//!
//! The version 0.1 can't correctly handle the `relative ref` which described in <https://tools.ietf.org/html/rfc3986#section-4.1>
//!
//! The parsed value of version 0.1 refers to the return value of <https://github.com/thlorenz/parse-link-header>, which is a `HashMap` with the same structure.
//!
//! **So if you want to parse `relative ref`, please use version `0.2`.**
//!
//! **Or if you don't care about `relative ref` and wanna simple `HashMap<String, HashMap<String, String>>` result, you can use version `0.1`.**
//!
//! ### Version 0.2.x
//!
//! In your `Cargo.toml`, add:
//!
//! ```toml
//! [dependencies]
//! parse_link_header = "0.2"
//! ```
//!
//! Then:
//!
//! ```rust
//! let link_header = "<https://api.github.com/repositories/41986369/contributors?page=2>; rel=\"next\", <https://api.github.com/repositories/41986369/contributors?page=14>; rel=\"last\"";
//!
//! parse_link_header::parse(link_header);
//! ```
//!
//! The parsed value is a `Result<HashMap<Option<Rel>, Link>, ()>`, which `Rel` and `Link` is:
//!
//! ```rust
//! use std::collections::HashMap;
//!
//! use http::Uri;
//!
//! #[derive(Debug, PartialEq)]
//! pub struct Link {
//!     pub uri: Uri, // https://docs.rs/http/0.2.1/http/uri/struct.Uri.html
//!     pub raw_uri: String,
//!     pub queries: HashMap<String, String>,
//!     pub params: HashMap<String, String>,
//! }
//!
//! type Rel = String;
//! ```
//!
//! You can see why the key of `HashMap` is `Option<Rel>` because if you won't provide a `rel` type, the key will be an empty string.
//!
//! Refer to <https://tools.ietf.org/html/rfc8288#section-3.3>, **The rel parameter MUST be present**.
//!
//! Therefore, if you find that key is `None`, please check if you provide the `rel` type.

use std::collections::HashMap;

use http::Uri;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Link {
    pub uri: Uri, // https://docs.rs/http/0.2.1/http/uri/struct.Uri.html
    pub raw_uri: String,
    pub queries: HashMap<String, String>,
    pub params: HashMap<String, String>,
}

type Rel = String;

/// Parse link header.
pub fn parse(link_header: &str) -> Result<HashMap<Option<Rel>, Link>, ()> {
    let mut result = HashMap::new();

    let re = Regex::new(r#"[<>"\s]"#).unwrap();
    let preprocessed = re.replace_all(link_header, "");
    let splited = preprocessed.split(',');

    for s in splited {
        let mut link_vec: Vec<&str> = s.split(";").collect();
        link_vec.reverse();

        let raw_uri = link_vec.pop().unwrap().to_string();
        let uri = match raw_uri.parse::<Uri>() {
            Ok(uri) => uri,
            Err(error) => panic!("Fail to parse uri: {}", error),
        };

        let mut queries = HashMap::new();
        if let Some(query) = uri.query() {
            let mut query = query.to_string();

            if query.starts_with('&') {
                query = query.chars().skip(1).collect();
            }

            for q in query.split('&') {
                let query_kv: Vec<&str> = q.split('=').collect();

                queries.insert(query_kv[0].to_string(), query_kv[1].to_string());
            }
        }

        let mut params = HashMap::new();
        let mut rel = None;

        for param in link_vec {
            let param_kv: Vec<&str> = param.split('=').collect();
            let key = param_kv[0];
            let val = param_kv[1];

            if key == "rel" {
                rel = Some(val.to_string());
            }

            params.insert(key.to_string(), val.to_string());
        }

        result.insert(
            rel,
            Link {
                uri,
                raw_uri,
                queries,
                params,
            },
        );
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{parse, Link, Uri};

    #[test]
    fn parse_link_header_works() {
        let link_header = "<https://api.github.com/repositories/41986369/contributors?page=2>; rel=\"next\", <https://api.github.com/repositories/41986369/contributors?page=14>; rel=\"last\"";
        let mut expected = HashMap::new();

        expected.insert(
            Some("next".to_string()),
            Link {
                uri: "https://api.github.com/repositories/41986369/contributors?page=2"
                    .parse::<Uri>()
                    .unwrap(),
                raw_uri: "https://api.github.com/repositories/41986369/contributors?page=2"
                    .to_string(),
                queries: [("page".to_string(), "2".to_string())]
                    .iter()
                    .cloned()
                    .collect(),
                params: [("rel".to_string(), "next".to_string())]
                    .iter()
                    .cloned()
                    .collect(),
            },
        );
        expected.insert(
            Some("last".to_string()),
            Link {
                uri: "https://api.github.com/repositories/41986369/contributors?page=14"
                    .parse::<Uri>()
                    .unwrap(),
                raw_uri: "https://api.github.com/repositories/41986369/contributors?page=14"
                    .to_string(),
                queries: [("page".to_string(), "14".to_string())]
                    .iter()
                    .cloned()
                    .collect(),
                params: [("rel".to_string(), "last".to_string())]
                    .iter()
                    .cloned()
                    .collect(),
            },
        );

        let parsed = parse(link_header).unwrap();

        assert_eq!(expected, parsed);

        let mut rel_link_expected = HashMap::new();

        rel_link_expected.insert(
            Some("foo/bar".to_string()),
            Link {
                uri: "/foo/bar".parse::<Uri>().unwrap(),
                raw_uri: "/foo/bar".to_string(),
                queries: HashMap::new(),
                params: [("rel".to_string(), "foo/bar".to_string())]
                    .iter()
                    .cloned()
                    .collect(),
            },
        );

        let rel_link_parsed = parse("</foo/bar>; rel=\"foo/bar\"").unwrap();

        assert_eq!(rel_link_expected, rel_link_parsed);
    }

    #[test]
    #[should_panic]
    fn parse_link_header_should_panic() {
        let _ = parse("<>");
    }

    #[test]
    fn sentry_paginating_results() {
        let link_header = "<https://sentry.io/api/0/projects/1/groups/?&cursor=1420837590:0:1>; rel=\"previous\"; results=\"false\", <https://sentry.io/api/0/projects/1/groups/?&cursor=1420837533:0:0>; rel=\"next\"; results=\"true\"";
        let mut expected = HashMap::new();

        expected.insert(
            Some("previous".to_string()),
            Link {
                uri: "https://sentry.io/api/0/projects/1/groups/?&cursor=1420837590:0:1"
                    .parse::<Uri>()
                    .unwrap(),
                raw_uri: "https://sentry.io/api/0/projects/1/groups/?&cursor=1420837590:0:1"
                    .to_string(),
                queries: [("cursor".to_string(), "1420837590:0:1".to_string())]
                    .iter()
                    .cloned()
                    .collect(),
                params: [
                    ("rel".to_string(), "previous".to_string()),
                    ("results".to_string(), "false".to_string()),
                ]
                .iter()
                .cloned()
                .collect(),
            },
        );

        expected.insert(
            Some("next".to_string()),
            Link {
                uri: "https://sentry.io/api/0/projects/1/groups/?&cursor=1420837533:0:0"
                    .parse::<Uri>()
                    .unwrap(),
                raw_uri: "https://sentry.io/api/0/projects/1/groups/?&cursor=1420837533:0:0"
                    .to_string(),
                queries: [("cursor".to_string(), "1420837533:0:0".to_string())]
                    .iter()
                    .cloned()
                    .collect(),
                params: [
                    ("rel".to_string(), "next".to_string()),
                    ("results".to_string(), "true".to_string()),
                ]
                .iter()
                .cloned()
                .collect(),
            },
        );

        let parsed = parse(link_header).unwrap();

        assert_eq!(expected, parsed);
    }
}
