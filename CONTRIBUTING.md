# Contributing to TrustVoting

Thank you for helping build voting infrastructure the public can actually verify. Contributions of code, review, documentation, and adversarial security analysis are all welcome.

## Licensing of contributions

This repository is **dual-licensed by directory**. By contributing, you agree that your contribution is licensed under the license of the directory you are changing:

- `crates/device/**` and `crates/county/**` → **AGPL-3.0-only**
- `crates/public/**` and `crates/shared/**` → **Apache-2.0**

Keep the `SPDX-License-Identifier` header at the top of every source file, and preserve copyright notices.

> **Open governance question (not yet decided): DCO vs CLA.**
> A lightweight **Developer Certificate of Origin** (sign-off with `git commit -s`) keeps things simple but locks the project into its current licenses. A **Contributor License Agreement** would let Scale Campaign LLC offer a commercial license later, at the cost of contributor friction. Until this is decided, please **sign off your commits** (`git commit -s`) so provenance is clear either way.

## Development

```bash
cargo build            # whole workspace
cargo test             # all tests
cargo fmt --all        # format (rustfmt)
cargo clippy --all-targets --all-features -- -D warnings   # lint
```

Please run `cargo fmt` and `cargo clippy` before opening a pull request, and add tests for new behavior.

## Ground rules

- **Verifiability first.** Prefer designs whose correctness a non-expert can check over designs that ask the reader to trust the implementation.
- **No secrets in the repo — ever.** Keys, certificates, `.env` files, and signing material never belong here (they are excluded by design and by `.gitignore`).
- **Non-partisan.** This is not a tool for any party or government. The villain is opacity and unverifiability, never a candidate.
- Security-sensitive reports go through [SECURITY.md](SECURITY.md), not public issues.

## Pull requests

Small, focused PRs with a clear description are easiest to review. Reference the module (crate) you're touching and explain what a reader should be able to verify about your change.
