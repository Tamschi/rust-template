# fn-formats

This is a small shim library for passing closures where you need one of the format traits (including [`Binary`] etc.).

[`Binary`]: https://doc.rust-lang.org/stable/core/fmt/trait.Binary.html

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

Which versions are affected by features and patches can be determined from the respective headings in [CHANGELOG.md](CHANGELOG.md).
