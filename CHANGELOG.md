# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- MIT License file
- Comprehensive CI/CD pipeline with GitHub Actions
- Security audit configuration with cargo-audit
- SECURITY.md for vulnerability reporting process
- Code coverage configuration (codecov.yml, tarpaulin.toml)
- CONTRIBUTING.md with contribution guidelines
- Dependabot configuration for automated dependency updates
- rustfmt.toml for consistent code formatting
- clippy.toml for enhanced linting rules
- CHANGELOG.md for tracking changes
- .editorconfig for consistent coding styles across editors
- Fuzz testing setup
- Benchmarking configuration

### Changed
- Updated .gitignore to clarify Cargo.lock tracking for binary crates
- Enhanced documentation with security and contribution information

### Fixed
- Resolved inconsistency between .gitignore and tracked Cargo.lock file

## [0.1.0] - 2024-12-XX

### Added
- Initial release of bos-algo
- Core distribution algorithm for 31,500 bottles
- Six-tier reward system (A-F)
- Cryptographically secure random number generation
- Integer-only arithmetic for financial calculations
- EUR cap support for Tier F bottles
- CLI simulation tool (sim)
- Comprehensive test suite with property-based testing
- Multi-language algorithm implementations in artifacts/
- Detailed algorithm documentation in ALGO.md
- Support for Rust 2024 edition with MSRV 1.77

### Security
- Enforced `#![forbid(unsafe_code)]` for memory safety
- Input validation for all parameters
- Secure random number generation using rand::thread_rng() (CSPRNG)

[Unreleased]: https://github.com/Beer-of-Satoshi-GmbH/bos-algo/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Beer-of-Satoshi-GmbH/bos-algo/releases/tag/v0.1.0