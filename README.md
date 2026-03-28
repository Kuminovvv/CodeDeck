# CodeDeck

[![CI](https://github.com/zed-industries/zed/actions/workflows/run_tests.yml/badge.svg)](https://github.com/zed-industries/zed/actions/workflows/run_tests.yml)

CodeDeck is a JetBrains-style IDE fork of [Zed](https://github.com/zed-industries/zed).

The goal of this fork is straightforward: keep Zed's performance and architecture, while evolving the shell and workflows toward a WebStorm / JetBrains-like experience.

Current direction:

- JetBrains-style window chrome and toolbar layout
- Denser project tree and panel styling
- Rounded, separated work areas and dock surfaces
- Tab and shell behavior tuned toward IDE-style workflows

CodeDeck is built on top of the upstream Zed codebase and should be treated as a fork, not an independent reimplementation.

---

### Status

CodeDeck is currently developed from source. Prebuilt installers and independent release channels are not set up yet.

### Building CodeDeck

- [Building on macOS](./docs/src/development/macos.md)
- [Building on Linux](./docs/src/development/linux.md)
- [Building on Windows](./docs/src/development/windows.md)

### Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for contribution guidelines.

When contributing, prefer changes that move the product toward clearer IDE-style workflows without fighting upstream architecture unnecessarily.

### Upstream

The upstream project for this fork is [zed-industries/zed](https://github.com/zed-industries/zed).

If you need stock Zed documentation, release information, or official downloads, use the upstream project and website:

- [Zed repository](https://github.com/zed-industries/zed)
- [Zed website](https://zed.dev)

### Licensing

License information for third party dependencies must be correctly provided for CI to pass.

We use [`cargo-about`](https://github.com/EmbarkStudios/cargo-about) to automatically comply with open source licenses. If CI is failing, check the following:

- Is it showing a `no license specified` error for a crate you've created? If so, add `publish = false` under `[package]` in your crate's Cargo.toml.
- Is the error `failed to satisfy license requirements` for a dependency? If so, first determine what license the project has and whether this system is sufficient to comply with this license's requirements. If you're unsure, ask a lawyer. Once you've verified that this system is acceptable add the license's SPDX identifier to the `accepted` array in `script/licenses/zed-licenses.toml`.
- Is `cargo-about` unable to find the license for a dependency? If so, add a clarification field at the end of `script/licenses/zed-licenses.toml`, as specified in the [cargo-about book](https://embarkstudios.github.io/cargo-about/cli/generate/config.html#crate-configuration).
