//! A library for parse http link header.
//!
//! # How to use

//! In your `Cargo.toml`, add:

//! ```toml
//! [dependencies]
//! parse_link_header = "0.1"
//! ```

//! Then:

//! ```rust
//! let link_header = "<https://api.github.com/repositories/41986369/contributors?page=2>; rel=\"next\", <https://api.github.com/repositories/41986369/contributors?page=14>; rel=\"last\"";

//! parse_link_header::parse(link_header);
//! ```

use std::collections::HashMap;

use http::Uri;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Link {
    pub uri: Uri,
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
}
