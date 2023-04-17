# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] - 2023-04-17

### Changed

- Use dynamic dispatch internally to save on code gen. External API unchanged.
- No longer heap-allocate futures.

### Breaking

- Minimum supported Rust version bumped to 1.68.

## [0.1.1] - 2022-09-05

### Fixed

- Fixed potential hanging when the waker is called while polling.

## [0.1.0] - 2022-09-01

- Initial release

[Unreleased]: https://github.com/bertptrs/beul/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/bertptrs/beul/compare/v0.1.1...v1.0.0
[0.1.1]: https://github.com/bertptrs/beul/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/bertptrs/beul/releases/tag/v0.1.0
