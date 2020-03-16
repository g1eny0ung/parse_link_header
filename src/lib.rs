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

use regex::Regex;
use url::Url;

/// Parse link header.
pub fn parse(link_header: &str) -> HashMap<String, HashMap<String, String>> {
    let mut result: HashMap<String, HashMap<String, String>> = HashMap::new();
    let re = Regex::new(r#"[<>"\s]"#).unwrap();
    let preprocessed = re.replace_all(link_header, "");
    let splited = preprocessed.split(',');

    for s in splited {
        let mut link_vec: Vec<&str> = s.split(";").collect();
        link_vec.reverse();

        let link_val = link_vec.pop().unwrap();
        let url_parsed = Url::parse(link_val).unwrap();
        let query_pairs = url_parsed.query_pairs();

        let mut rel_val = String::from("");
        let mut map: HashMap<String, String> = HashMap::new();

        for param in link_vec {
            let param_kv: Vec<&str> = param.split("=").collect();
            let key = param_kv[0];
            let val = param_kv[1];

            if key == "rel" {
                rel_val = val.to_string();
            }

            map.insert(key.to_string(), val.to_string());
        }

        for pair in query_pairs {
            map.insert(pair.0.to_string(), pair.1.to_string());
        }

        map.insert("link".to_string(), link_val.to_string());

        result.insert(rel_val, map);
    }

    result
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn parse_link_header_works() {
        let link_header = "<https://api.github.com/repositories/41986369/contributors?page=2>; rel=\"next\", <https://api.github.com/repositories/41986369/contributors?page=14>; rel=\"last\"";
        let mut expected: HashMap<String, HashMap<String, String>> = HashMap::new();
        let mut next_map: HashMap<String, String> = HashMap::new();
        let mut last_map: HashMap<String, String> = HashMap::new();

        next_map.insert("rel".to_string(), "next".to_string());
        next_map.insert(
            "link".to_string(),
            "https://api.github.com/repositories/41986369/contributors?page=2".to_string(),
        );
        next_map.insert("page".to_string(), "2".to_string());

        last_map.insert("rel".to_string(), "last".to_string());
        last_map.insert(
            "link".to_string(),
            "https://api.github.com/repositories/41986369/contributors?page=14".to_string(),
        );
        last_map.insert("page".to_string(), "14".to_string());

        expected.insert("next".to_string(), next_map);
        expected.insert("last".to_string(), last_map);

        let parsed = super::parse(link_header);

        assert_eq!(expected, parsed);
    }
}
