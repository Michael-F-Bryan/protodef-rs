# ProtoDef Rust Code Generator

[![Continuous integration](https://github.com/Michael-F-Bryan/protodef-rs/workflows/Continuous%20integration/badge.svg?branch=master)](https://github.com/Michael-F-Bryan/protodef-rs/actions)

([API Docs])

A Rust code generator for the [ProtoDef][proto] protocol specification format.

This project is split into three pieces:

- `protodef-core` - core abstractions and types used by ProtoDef-generated code
- `protodef-codegen` - a tool for converting a `protocol.json` into Rust types
- `protodef-cli` - a helper for invoking `protodef-codegen` on a single
  `protocol.json` file

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE.md) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT.md) or
   http://opensource.org/licenses/MIT)

at your option.

It is recommended to always use [cargo-crev][crev] to verify the
trustworthiness of each of your dependencies, including this one.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

The intent of this crate is to be free of soundness bugs. The developers will
do their best to avoid them, and welcome help in analysing and fixing them.

[API Docs]: https://michael-f-bryan.github.io/protodef-rs
[crev]: https://github.com/crev-dev/cargo-crev
[proto]: https://github.com/ProtoDef-io/ProtoDef
