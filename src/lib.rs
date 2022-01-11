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
//! ### Example
//!
//! In your `Cargo.toml`, add:
//!
//! ```toml
//! [dependencies]
//! parse_link_header = "0.3"
//! ```
//!
//! Then:
//!
//! ```rust
//! let link_header = r#"<https://api.github.com/repositories/41986369/contributors?page=2>; rel="next", <https://api.github.com/repositories/41986369/contributors?page=14>; rel="last""#;
//!
//! let res = parse_link_header::parse(link_header);
//! assert!(res.is_ok());
//!
//! let val = res.unwrap();
//! assert_eq!(val.len(), 2);
//! assert_eq!(val.get(&Some("next".to_string())).unwrap().raw_uri, "https://api.github.com/repositories/41986369/contributors?page=2");
//! assert_eq!(val.get(&Some("last".to_string())).unwrap().raw_uri, "https://api.github.com/repositories/41986369/contributors?page=14");
//! ```
//!
//! The parsed value is a `Result<HashMap<Option<Rel>, Link>, Error>` (aka a
//! [`LinkMap`](type.LinkMap.html)), which `Rel` and `Link` is:
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
//! Refer to <https://tools.ietf.org/html/rfc8288#section-3.3> (October 2017), **the rel parameter MUST be present**.
//!
//! Therefore, if you find that key is `None`, please check if you provide the `rel` type.
//!
//! ## parse_with_rel
//!
//! > Version >= 0.3.0
//!
//! Alternatively, use the `parse_with_rel()` function to get a `HashMap<String, Link>` (aka a
//! [`RelLinkMap`](type.RelLinkMap.html)), as in:
//!
//! ```rust
//! let link_header = r#"<https://api.github.com/repositories/41986369/contributors?page=2>; rel="next", <https://api.github.com/repositories/41986369/contributors?page=14>; rel="last""#;
//!
//! let res = parse_link_header::parse_with_rel(link_header);
//! assert!(res.is_ok());
//!
//! let val = res.unwrap();
//! assert_eq!(val.len(), 2);
//! assert_eq!(val.get("next").unwrap().raw_uri, "https://api.github.com/repositories/41986369/contributors?page=2");
//! assert_eq!(val.get("last").unwrap().raw_uri, "https://api.github.com/repositories/41986369/contributors?page=14");
//! ```
//!
//! ## Feature: `url`
//!
//! > Version >= 0.3.0
//!
//! If you enable the `url` feature, the `uri` field of struct [`Link`](struct.Link.html) will be
//! of type url::Url from the [url crate](https://crates.io/crates/url), rather than the
//! `http::Uri` it normally is.  This allows direct use of the `uri` field with other popular
//! crates that use `url`, such as [`reqwest`](https://crates.io/crates/reqwest).
//!
//! **NOTE:** This implictly disabled support for relative refs, as URLs do not support relative
//! refs (whereas URIs do).

use std::collections::HashMap;
use std::fmt;

#[cfg(not(feature = "url"))]
use http::Uri;

#[cfg(feature = "url")]
use url::Url as Uri;

/// A `Result` alias where the `Err` case is [`parse_link_header::Error`].
///
/// [`parse_link_header::Error`]: struct.Error.html
pub type Result<T> = std::result::Result<T, Error>;

/// An error encountered when attempting to parse a `Link:` HTTP header
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Error(ErrorKind);

/// Enum to indicate the type of error encountered
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ErrorKind {
    /// Internal error of the type that should never happen
    InternalError,

    /// Failure to parse link value into URI
    InvalidURI,

    /// Malformed parameters
    MalformedParam,

    /// Malformed URI query
    MalformedQuery,

    /// Missing `rel` parameter when required
    MissingRel,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            ErrorKind::InternalError => write!(f, "internal parser error"),
            ErrorKind::InvalidURI => write!(f, "unable to parse URI component"),
            ErrorKind::MalformedParam => write!(f, "malformed parameter list"),
            ErrorKind::MalformedQuery => write!(f, "malformed URI query"),
            ErrorKind::MissingRel => write!(f, "missing 'rel' parameter"),
        }
    }
}

impl std::error::Error for Error {}

impl From<&Error> for Error {
    /// Create a new Error object from a borrowed immutable reference.  This is required as part of
    /// using `lazy_static!`, as that deals in references.
    fn from(x: &Error) -> Self {
        Error(x.0)
    }
}

/// Struct to describe a single `Link:` header entry
///
/// This stores the raw URI found in the header, as well as parsed forms of that URI (including the
/// queries) and any parameters associated with this URI.
#[derive(Debug, PartialEq)]
pub struct Link {
    /// A parsed form of the URI
    pub uri: Uri, // https://docs.rs/http/0.2.1/http/uri/struct.Uri.html

    /// The raw text string of the URI
    pub raw_uri: String,

    /// A `HashMap` of the query part of the URI (in the form of key=value)
    pub queries: HashMap<String, String>,

    /// A `HashMap` of the parameters associated with this URI.  The most common is `rel`,
    /// indicating the relationship between the current HTTP data being fetched and the URI in this
    /// `Link:` header.
    pub params: HashMap<String, String>,
}

type Rel = String;

/// Type alias for the parsed data returned as a `HashMap` with a key of `Option<Rel>`.
///
/// This is different from [`RelLinkMap`](type.RelLinkMap.html) which has a key of `Rel`.
pub type LinkMap = HashMap<Option<Rel>, Link>;

/// Type alias for the parsed data returned as a `HashMap` where the `rel` parameter is required to
/// be present.
///
/// This is different from the [`LinkMap`](type.LinkMap.html) which has a key of `Option<Rel>`
pub type RelLinkMap = HashMap<Rel, Link>;

/// Parse link header into a [`RelLinkMap`](type.RelLinkMap.html).
///
/// Takes a `&str` which is the value of the HTTP `Link:` header, attempts to parse it, and returns
/// a `Result<RelLinkMap>` which represents the mapping between the relationship and the link entry.
pub fn parse_with_rel(link_header: &str) -> Result<RelLinkMap> {
    parse_with(link_header, |x| x.ok_or(Error(ErrorKind::MissingRel)))
}

/// Parse link header into a [`LinkMap`](type.LinkMap.html).
///
/// Takes a `&str` which is the value of the HTTP `Link:` header, attempts to parse it, and returns
/// a `Result<LinkMap>` which represents the mapping between the relationship and the link entry.
pub fn parse(link_header: &str) -> Result<LinkMap> {
    parse_with(link_header, Ok)
}

/// Generic parser function
///
/// Does the actual parsing work, and then uses make_key() to proceses the HashMap key into the
/// desired type.
fn parse_with<K, F>(link_header: &str, make_key: F) -> Result<HashMap<K, Link>>
where
    K: Eq + std::hash::Hash,
    F: Fn(Option<String>) -> Result<K>,
{
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        static ref RE: Result<Regex> =
            Regex::new(r#"[<>"\s]"#).or(Err(Error(ErrorKind::InternalError)));
    }
    let mut result = HashMap::new();

    // remove all quotes, angle brackets, and whitespace
    let preprocessed = RE.as_ref()?.replace_all(link_header, "");

    // split along comma into different entries
    let splited = preprocessed.split(',');

    for s in splited {
        // split each entry into parts
        let mut link_vec: Vec<_> = s.split(';').collect();
        link_vec.reverse();

        // pop off the link value; the split() guarantees at least one entry to pop()
        let raw_uri = link_vec
            .pop()
            .ok_or(Error(ErrorKind::InternalError))?
            .to_string();
        let uri: Uri = raw_uri.parse().or(Err(Error(ErrorKind::InvalidURI)))?;

        let mut queries = HashMap::new();
        if let Some(query) = uri.query() {
            let mut query = query.to_string();

            // skip leading ampersand
            if query.starts_with('&') {
                query = query.chars().skip(1).collect();
            }

            // split each query and extract as (key, value) pairs
            for q in query.split('&') {
                let (key, val) = q.split_once('=').ok_or(Error(ErrorKind::MalformedQuery))?;

                queries.insert(key.to_string(), val.to_string());
            }
        }

        let mut params = HashMap::new();

        // extract the parameter list as (key, value) pairs
        for param in link_vec {
            let (key, val) = param
                .split_once('=')
                .ok_or(Error(ErrorKind::MalformedParam))?;

            params.insert(key.to_string(), val.to_string());
        }

        result.insert(
            make_key(params.get("rel").cloned())?,
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

    use super::*;

    #[test]
    fn parse_link_header_works() {
        let link_header = r#"<https://api.github.com/repositories/41986369/contributors?page=2>; rel="next", <https://api.github.com/repositories/41986369/contributors?page=14>; rel="last""#;
        let mut expected = HashMap::new();

        expected.insert(
            Some("next".to_string()),
            Link {
                uri: "https://api.github.com/repositories/41986369/contributors?page=2"
                    .parse()
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
                    .parse()
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

        #[cfg(not(feature = "url"))]
        {
            let mut rel_link_expected = HashMap::new();

            rel_link_expected.insert(
                Some("foo/bar".to_string()),
                Link {
                    uri: "/foo/bar".parse().unwrap(),
                    raw_uri: "/foo/bar".to_string(),
                    queries: HashMap::new(),
                    params: [("rel".to_string(), "foo/bar".to_string())]
                        .iter()
                        .cloned()
                        .collect(),
                },
            );

            let rel_link_parsed = parse(r#"</foo/bar>; rel="foo/bar""#).unwrap();

            assert_eq!(rel_link_expected, rel_link_parsed);
        }
    }

    #[test]
    fn parse_with_rel_works() {
        let link_header = r#"<https://api.github.com/repositories/41986369/contributors?page=2>; rel="next", <https://api.github.com/repositories/41986369/contributors?page=14>; rel="last""#;
        let mut expected = HashMap::new();

        expected.insert(
            "next".to_string(),
            Link {
                uri: "https://api.github.com/repositories/41986369/contributors?page=2"
                    .parse()
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
            "last".to_string(),
            Link {
                uri: "https://api.github.com/repositories/41986369/contributors?page=14"
                    .parse()
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

        let parsed = parse_with_rel(link_header).unwrap();

        assert_eq!(expected, parsed);

        #[cfg(not(feature = "url"))]
        {
            let mut rel_link_expected = HashMap::new();

            rel_link_expected.insert(
                "foo/bar".to_string(),
                Link {
                    uri: "/foo/bar".parse().unwrap(),
                    raw_uri: "/foo/bar".to_string(),
                    queries: HashMap::new(),
                    params: [("rel".to_string(), "foo/bar".to_string())]
                        .iter()
                        .cloned()
                        .collect(),
                },
            );

            let rel_link_parsed = parse_with_rel(r#"</foo/bar>; rel="foo/bar""#).unwrap();

            assert_eq!(rel_link_expected, rel_link_parsed);
        }
    }

    #[test]
    fn parse_link_header_should_err() {
        assert_eq!(parse("<>"), Err(Error(ErrorKind::InvalidURI)));
    }

    #[test]
    fn parse_with_rel_should_err() {
        assert_eq!(
            parse_with_rel(r#"<http://local.host/foo/bar>; type="foo/bar""#),
            Err(Error(ErrorKind::MissingRel))
        );
    }

    #[test]
    fn sentry_paginating_results() {
        let link_header = r#"<https://sentry.io/api/0/projects/1/groups/?&cursor=1420837590:0:1>; rel="previous"; results="false", <https://sentry.io/api/0/projects/1/groups/?&cursor=1420837533:0:0>; rel="next"; results="true""#;
        let mut expected = HashMap::new();

        expected.insert(
            Some("previous".to_string()),
            Link {
                uri: "https://sentry.io/api/0/projects/1/groups/?&cursor=1420837590:0:1"
                    .parse()
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
                    .parse()
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

    #[test]
    fn test_error_display() {
        assert_eq!(
            format!("{}", Error(ErrorKind::InternalError)),
            "internal parser error"
        );

        assert_eq!(
            format!("{}", Error(ErrorKind::InvalidURI)),
            "unable to parse URI component"
        );

        assert_eq!(
            format!("{}", Error(ErrorKind::MalformedParam)),
            "malformed parameter list"
        );

        assert_eq!(
            format!("{}", Error(ErrorKind::MalformedQuery)),
            "malformed URI query"
        );

        assert_eq!(
            format!("{}", Error(ErrorKind::MissingRel)),
            "missing 'rel' parameter"
        );
    }

    #[test]
    fn test_error_from() {
        let e1 = Error(ErrorKind::InternalError);
        let e2 = Error::from(&e1);

        assert_eq!(e1, e2);
    }
}
