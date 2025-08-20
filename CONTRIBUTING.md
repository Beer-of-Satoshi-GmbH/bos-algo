# Contributing to bos-algo

First off, thank you for considering contributing to bos-algo! It's people like you that make bos-algo such a great tool.

## Code of Conduct

By participating in this project, you are expected to uphold our values:
- Be respectful and inclusive
- Welcome newcomers and help them get started
- Focus on constructive criticism
- Accept feedback gracefully

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues to avoid duplicates. When creating a bug report, please include:

- **Clear and descriptive title**
- **Steps to reproduce** the problem
- **Expected behavior** and what actually happened
- **Environment details** (OS, Rust version, etc.)
- **Code samples** or test cases if applicable

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, please include:

- **Clear and descriptive title**
- **Detailed description** of the proposed enhancement
- **Rationale** explaining why this would be useful
- **Examples** of how it would be used
- **Possible implementation** approach (if you have ideas)

### Pull Requests

1. **Fork the repository** and create your branch from `dev`
2. **Write clear commit messages** following conventional commits:
   - `feat:` for new features
   - `fix:` for bug fixes
   - `docs:` for documentation changes
   - `test:` for test additions/changes
   - `refactor:` for code refactoring
   - `perf:` for performance improvements
   - `chore:` for maintenance tasks

3. **Ensure tests pass:**
   ```bash
   cargo test --all-features
   cargo clippy -- -D warnings
   cargo fmt -- --check
   ```

4. **Add tests** for any new functionality

5. **Update documentation** as needed

6. **Create a Pull Request** with:
   - Clear description of changes
   - Link to related issue(s)
   - Screenshots/examples if applicable

## Development Setup

1. **Install Rust** (1.77.0 or later):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone the repository:**
   ```bash
   git clone https://github.com/Beer-of-Satoshi-GmbH/bos-algo.git
   cd bos-algo
   ```

3. **Install development tools:**
   ```bash
   cargo install cargo-audit cargo-tarpaulin cargo-fuzz
   rustup component add clippy rustfmt
   ```

4. **Run tests:**
   ```bash
   cargo test
   cargo test --doc
   cargo clippy -- -D warnings
   ```

## Testing Guidelines

- **Unit tests**: Test individual functions in isolation
- **Integration tests**: Test module interactions
- **Property tests**: Use proptest for invariant testing
- **Documentation tests**: Ensure examples in docs work
- **Fuzz tests**: Add fuzz targets for new parsing/calculation logic

Example test structure:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_specific_behavior() {
        // Arrange
        let input = /* ... */;
        
        // Act
        let result = function_under_test(input);
        
        // Assert
        assert_eq!(result, expected);
    }
}
```

## Code Style

We use `rustfmt` for formatting and `clippy` for linting:

- Run `cargo fmt` before committing
- Ensure `cargo clippy -- -D warnings` passes
- Follow Rust naming conventions
- Write descriptive variable names
- Add comments for complex logic
- Keep functions small and focused

## Documentation

- Add rustdoc comments to all public items
- Include examples in documentation
- Update README.md for user-facing changes
- Keep CHANGELOG.md up to date

## Performance Considerations

Since bos-algo handles financial calculations:

- **Always use integer arithmetic** for money values
- **Avoid allocations** in hot paths
- **Benchmark** performance-critical changes
- **Profile** before optimizing
- **Test edge cases** thoroughly

## Security

- **Never use `unsafe` code** (enforced by `#![forbid(unsafe_code)]`)
- **Validate all inputs** before processing
- **Use `Result` types** for error handling
- **Avoid panics** in library code
- **Report security issues** privately (see SECURITY.md)

## Release Process

1. Update version in `Cargo.toml`
2. Update CHANGELOG.md
3. Create a PR from `dev` to `main`
4. After merge, tag the release: `git tag v0.1.0`
5. Push tags: `git push --tags`
6. GitHub Actions will handle the rest

## Questions?

Feel free to:
- Open an issue for questions
- Reach out on X/Twitter: [@BeerOfSatoshi](https://x.com/BeerOfSatoshi)
- Email us: [dev@beersatoshi.com](mailto:dev@beersatoshi.com)

## Recognition

Contributors will be recognized in:
- The project README
- Release notes
- Our website (with permission)

Thank you for contributing! 🍺⚡