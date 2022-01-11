# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.2] - 2022-01-11

### Changed

- Refactor `parse()`, `parse_with_rel()` for speed [bb9918c](https://github.com/g1eny0ung/parse_link_header/commit/bb9918c80a8e28a8bc62a34e911e161153df94e2)

## [0.3.0] - 2022-01-08

### Added

- Implement feature to use `url::Url` rather than `http::Uri` by @mdharm in <https://github.com/g1eny0ung/parse_link_header/pull/10>
- Add `parse_with_rel()` for when the `rel` parameter MUST be present by @mdharm in <https://github.com/g1eny0ung/parse_link_header/pull/11>

### Changed

- Add proper type for Result<T, E> by @mdharm in <https://github.com/g1eny0ung/parse_link_header/pull/5>
- Move regular expression compilation to `lazy_static!` by @mdharm in <https://github.com/g1eny0ung/parse_link_header/pull/9>
