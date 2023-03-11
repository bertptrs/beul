# Beul

Beul is a minimalistic futures executor. No dependencies, no unsafe rust. It simply executes
futures.

## Usage

Simply call `execute` with your future:

```rust
beul::execute(async {});
```

### Backwards compatibility

This crate requires at least Rust 1.68, due to its reliance on [std::pin::pin!]. Increases in this
version will be considered breaking changes. This crate follows semantic versioning.

### Limitations

Beul is a single-threaded executor and will not provide anything but execution. Futures that depend
on runtime features, as present for example in [Tokio], will not work.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

[Tokio]: https://tokio.rs/
[std::pin::pin!]: https://doc.rust-lang.org/std/pin/macro.pin.html
