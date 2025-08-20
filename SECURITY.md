# Security Policy

## Supported Versions

We release patches for security vulnerabilities. Currently supported versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

We take the security of `bos-algo` seriously. If you believe you have found a security vulnerability, please report it to us as described below.

### Reporting Process

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them via email to:
- **Email**: dev@beersatoshi.com
- **Subject Line**: [SECURITY] bos-algo vulnerability

Please include the following information:
- Type of issue (e.g., buffer overflow, SQL injection, cross-site scripting, etc.)
- Full paths of source file(s) related to the manifestation of the issue
- The location of the affected source code (tag/branch/commit or direct URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit it

### Response Timeline

You should receive a response within **48 hours**. If for some reason you do not, please follow up via X/Twitter: [@BeerOfSatoshi](https://x.com/BeerOfSatoshi).

We will:
1. Confirm receipt of your vulnerability report
2. Investigate and validate the issue
3. Develop and test a fix
4. Prepare a security advisory
5. Release the fix and publish the advisory

### Disclosure Policy

- We request that you give us reasonable time to address the issue before public disclosure
- We will credit reporters who follow responsible disclosure (unless you prefer to remain anonymous)
- We aim to resolve critical issues within 7 days of validation

## Security Best Practices for Users

When using `bos-algo`:

1. **Keep dependencies updated**: Regularly run `cargo update` and `cargo audit`
2. **Use official releases**: Download from crates.io or official GitHub releases
3. **Verify checksums**: When downloading binaries, verify SHA256 checksums
4. **Review configuration**: Ensure EUR caps and BTC prices are validated from trusted sources
5. **Monitor logs**: Keep an eye on any unusual distribution patterns

## Security Features

`bos-algo` implements several security measures:

- ✅ **No unsafe code**: `#![forbid(unsafe_code)]`
- ✅ **Cryptographically secure RNG**: Uses `rand::thread_rng()` (CSPRNG)
- ✅ **Integer-only arithmetic**: Prevents floating-point vulnerabilities
- ✅ **Input validation**: All inputs are validated before processing
- ✅ **Memory safety**: Guaranteed by Rust's ownership system
- ✅ **No external network calls**: Operates entirely offline
- ✅ **Deterministic testing**: Property-based tests ensure correctness

## Past Security Advisories

No security advisories have been issued for this project yet.

## Contact

- **Website**: [beerofsatoshi.com](https://beerofsatoshi.com/)
- **Email**: [dev@beersatoshi.com](mailto:dev@beersatoshi.com)
- **X/Twitter**: [@BeerOfSatoshi](https://x.com/BeerOfSatoshi)