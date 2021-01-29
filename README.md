# nrfxlib-sys

This is a Rust wrapper for the Nordic
[nrfxlib](https://github.com/NordicPlayground/nrfxlib) set of libraries,
primarily `libbsd` and `liboberon` for the nRF9160.

Any binary which uses this crate is going to need to provide a bunch of C
library functions, because Nordic's library expects them. This includes, but
is not limited to:

* atoi
* snprintf
* strol
* strchr

You can't just link `newlib`, because that defines `memset` which clashes with
the `compiler-builtin` crate's definition of `memset`. Answers on a post-card
please - for now I'm using
[tinyrlibc](https://github.com/thejpster/tinyrlibc).

## Using

In your own program or library, you can depend on this crate in the usual fashion:

```toml
[dependencies]
nrfxlib-sys = "1.4"
```

Or you might prefer the [higher-level wrapper by 42 Technology](https://crates.io/crates/nrfxlib):

```toml
[dependencies]
nrfxlib = "*"
```

Note that you may need to use Nightly Rust and add `-Zfeatures=host_dep` to enable the Cargo #5730 fix.

```console
$ cargo +nightly build -Zfeatures=host_dep
```

## Licence

Any of the code outside the `./third_party` folder is under the [Blue Oak
Licence](./LICENCE.md). Any code inside the `./third_party` folder (include
the Nordic nrfxlib) has its own LICENCE file.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, shall be licensed as above,
without any additional terms or conditions.

## Changelog

### Unreleased Changes ([Source](https://github.com/42-technology-ltd/nrfxlib-sys/tree/master) | [Changes](https://github.com/42-technology-ltd/nrfxlib-sys/compare/v1.2.0...master))

* Updated to [nrfxlib v1.4.2](https://github.com/NordicPlayground/nrfxlib/tree/v1.4.2)
* Added new arguments to call to `bsd_init` function.
* Switched back to `bindgen` crate, after Cargo issue #5730 was closed.

### v1.2.0 ([Source](https://github.com/42-technology-ltd/nrfxlib-sys/tree/v1.2.0) | [Changes](https://github.com/42-technology-ltd/nrfxlib-sys/compare/v1.1.0-rc3%2Brel1...v1.2.0))

* Updated to [nrfxlib v1.2.0](https://github.com/NordicPlayground/nrfxlib/tree/v1.2.0)
* Fixed some build errors caused by usize/u32 and isize/i32 conversions.

### v1.1.0-rc3+rel1 ([Source](https://github.com/42-technology-ltd/nrfxlib-sys/tree/v1.1.0-rc3%2Brel1) | [Changes](https://github.com/42-technology-ltd/nrfxlib-sys/compare/v1.1.0-rc2%2Brel1...v1.1.0-rc3%2Brel1))

* Updated to [nrfxlib v1.1.0-rc3](https://github.com/NordicPlayground/nrfxlib/tree/v1.1.0-rc3)
* Generate headers for `libnrf_cc310` (CryptoCell 310 API)
* Generate headers for `liboberon` (Optimised software crypto implementation)

### v1.1.0-rc2+rel1 ([Source](https://github.com/42-technology-ltd/nrfxlib-sys/tree/v1.1.0-rc2%2Brel1) | [Changes](https://github.com/42-technology-ltd/nrfxlib-sys/compare/v0.2.0...v1.1.0-rc2%2Brel1))

* Updated to [nrfxlib v1.1.0-rc2](https://github.com/NordicPlayground/nrfxlib/tree/v1.1.0-rc2), including [libbsd 0.5.0](https://github.com/NordicPlayground/nrfxlib/blob/v1.1.0-rc2/bsdlib/CHANGELOG.rst)
* Changed crate version to track Nordic's nrfxlib version number.

### v0.2.0 ([Source](https://github.com/42-technology-ltd/nrfxlib-sys/tree/v0.2.0) | [Changes](https://github.com/42-technology-ltd/nrfxlib-sys/compare/v0.1.7...v0.2.0))

* Require users to install bindgen as a command-line tool.

### v0.1.7 ([Source](https://github.com/42-technology-ltd/nrfxlib-sys/tree/v0.1.7) | [Changes](https://github.com/42-technology-ltd/nrfxlib-sys/compare/v0.1.6...v0.1.7))

* Use Cargo 5730 workaround.
* Update bindgen to 0.51
* Rustfmt generated code
* Reformat using tabs

### v0.1.6 ([Source](https://github.com/42-technology-ltd/nrfxlib-sys/tree/v0.1.6) | [Changes](https://github.com/42-technology-ltd/nrfxlib-sys/compare/v0.1.5...v0.1.6))

* Bundle the C headers (fixes Travis build)

### v0.1.5 ([Source](https://github.com/42-technology-ltd/nrfxlib-sys/tree/v0.1.5) | [Changes](https://github.com/42-technology-ltd/nrfxlib-sys/compare/v0.1.4...v0.1.5))

* Sub-module in upstream nrfxlib.

### v0.1.4 ([Source](https://github.com/42-technology-ltd/nrfxlib-sys/tree/v0.1.4) | [Changes](https://github.com/42-technology-ltd/nrfxlib-sys/compare/v0.1.3...v0.1.4))

* Add the bsd_limits.h, nrf_key_mgmt.h and nrf_apn_class.h headers.

### v0.1.3 ([Source](https://github.com/42-technology-ltd/nrfxlib-sys/tree/v0.1.3) | [Changes](https://github.com/42-technology-ltd/nrfxlib-sys/compare/v0.1.2...v0.1.3))

* Updates to this README to specify checkout of tag v1.0.0.
* Added a Travis CI file.

### v0.1.2 ([Source](https://github.com/42-technology-ltd/nrfxlib-sys/tree/v0.1.2) | [Changes](https://github.com/42-technology-ltd/nrfxlib-sys/compare/v0.1.1...v0.1.2))

* Add NEWLIB_PATH so user can point to correct newlib headers.

### v0.1.1 ([Source](https://github.com/42-technology-ltd/nrfxlib-sys/tree/v0.1.1) | [Changes](https://github.com/42-technology-ltd/nrfxlib-sys/compare/v0.1.0...v0.1.1))

* Updates to the README and crate metadata

### v0.1.0 ([Source](https://github.com/42-technology-ltd/nrfxlib-sys/tree/v0.1.0))

* First version
