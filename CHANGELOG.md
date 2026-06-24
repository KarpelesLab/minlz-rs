# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.2.0](https://github.com/KarpelesLab/minlz-rs/compare/v1.1.0...v1.2.0) - 2026-06-24

### Other

- Add release-plz + release-binaries workflows
- Fold CLI tools into the minlz crate behind a `cli` feature
- build each codec on its own (incl. bare-metal no_std)
- Document complete MinLZ feature set (stream/index/dict/levels/CLI)
- Add mzc/mzd MinLZ CLI tools
- Add MinLZ dictionary support (crate-local)
- Add MinLZ stream index + seeking (chunk 0x40)
- Add MinLZ stream format (Reader/Writer + CRC32C)
- Snappy/S2 block fallback + encoder levels
- run the MinLZ reference-vector test
- Document the S2 + MinLZ dual-codec layout
- Add MinLZ block encoder (greedy matcher)
- Add MinLZ codec scaffolding + verified block decoder
- Add no_std support (block API on core + alloc)
