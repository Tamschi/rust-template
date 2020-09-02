# fn-formats

[![Lib.rs](https://img.shields.io/badge/Lib.rs-*-84f)](https://lib.rs/crates/fn-formats)
[![Crates.io](https://img.shields.io/crates/v/fn-formats)](https://crates.io/crates/fn-formats)
[![Docs.rs](https://img.shields.io/badge/Docs.rs-*-black)](https://docs.rs/crates/fn-formats)

![Rust 1.46.0](https://img.shields.io/static/v1?logo=Rust&label=&message=1.46.0&color=grey)
[![Build Status](https://travis-ci.com/Tamschi/fn-formats.svg?branch=develop)](https://travis-ci.com/Tamschi/fn-formats/branches)
![Crates.io - License](https://img.shields.io/crates/l/fn-formats/0.0.4)

[![GitHub](https://img.shields.io/static/v1?logo=GitHub&label=&message=%20&color=grey)](https://github.com/Tamschi/fn-formats)
[![open issues](https://img.shields.io/github/issues-raw/Tamschi/fn-formats)](https://github.com/Tamschi/fn-formats/issues)
[![open pull requests](https://img.shields.io/github/issues-pr-raw/Tamschi/fn-formats)](https://github.com/Tamschi/fn-formats/pulls)
[![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/fn-formats.svg)](https://web.crev.dev/rust-reviews/crate/fn-formats/)

This is a small shim library for passing closures where you need one of the format traits (including [`Binary`] etc.).

[`Binary`]: https://doc.rust-lang.org/stable/core/fmt/trait.Binary.html

## Installation

Please use [cargo-edit](https://crates.io/crates/cargo-edit) to always add the latest version of this library:

```cmd
cargo add fn-formats
```

## Example

```rust
use fn_formats::DebugFmt;

let debug = DebugFmt(|f| {
    f.debug_struct("StructName")
        .field("list", &DebugFmt(|f| f.debug_list().entries(&[1, 2, 3]).finish()))
        .field("set", &DebugFmt(|f| f.debug_set().entries(&[4, 5, 6]).finish()))
        .finish()
});

assert_eq!(format!("{:?}", debug), "StructName { list: [1, 2, 3], set: {4, 5, 6} }");
```

## License

Licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## [Changelog](CHANGELOG.md)

## Versioning

`fn-formats` strictly follows [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html) with the following exceptions:

* The minor version will not reset to 0 on major version changes (except for v1).  
Consider it the global feature level.
* The patch version will not reset to 0 on major or minor version changes (except for v0.1 and v1).  
Consider it the global patch level.

This includes the Rust version requirement specified above.  
Earlier versions may be compatible, but this can change with minor or patch releases.

Which versions are affected by features and patches can be determined from the respective headings in [CHANGELOG.md](CHANGELOG.md).
