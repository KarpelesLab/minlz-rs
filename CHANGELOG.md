# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.2.3](https://github.com/KarpelesLab/minlz-rs/compare/v1.2.2...v1.2.3) - 2026-06-24

### Other

- MinLZ performance summary + target-cpu=native tip
- Faster MinLZ encode: AVX2/BMI2 multiversioning of the matchers
- Faster MinLZ decode: AVX2 multiversioning + 32-byte wildcopy
- Faster MinLZ encode: drop bounds checks in matcher hot loops
- Add zero-alloc compress_into; share body builder
- MinLZ perf: recycle encoder tables + SSSE3 small-offset decode

## [1.2.2](https://github.com/KarpelesLab/minlz-rs/compare/v1.2.1...v1.2.2) - 2026-06-24

### Other

- Faster MinLZ decode: pointer-based wildcopy fast zone
- Faster MinLZ Fastest encode: 3-hash matcher + copy chaining
- Faster MinLZ encode: two-table Balanced matcher + fused literals

## [1.2.1](https://github.com/KarpelesLab/minlz-rs/compare/v1.2.0...v1.2.1) - 2026-06-24

### Other

- steer new code to s2/minlz modules; document cli feature
- Optimize MinLZ encode and decode hot paths
- scope minimal-versions to library features
- release v1.2.0 ([#1](https://github.com/KarpelesLab/minlz-rs/pull/1))

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
