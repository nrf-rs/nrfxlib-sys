# nrfxlib-sys

This is a Rust wrapper for the Nordic
[nrfxlib](https://github.com/NordicPlayground/nrfxlib) set of libraries,
primarily `libmodem` and `liboberon` for the nRF91* series chips (System-in-Packages). Supported chips are the following:

* nRF9160
* nRF9151
* nRF9161

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
# A chip feature must be selected
nrfxlib-sys = { version = "3.0.2", features = ["nrf9160"] }
```

Because the modem library has its debug sections compressed and Rust's tooling doesn't have support for
that by default, this crate either strips the debug sections or decompresses them.

By default the crate uses the `llvm-tools` that can be installed using `rustup component add llvm-tools`.
In this case the debug sections get stripped.

If you'd rather have the debug sections decompressed, then disable the default features on this crate and
enable the `arm-none-eabi-objcopy` feature. This will try to use the `arm-none-eabi-objcopy` binary that you can
download from the ARM website. This one does have support for debug section compression.

This is a low level wrapper. You might prefer the blocking [higher-level wrapper](https://crates.io/crates/nrfxlib):

```toml
[dependencies]
nrfxlib = "*"
```

You might also prefer the async [higher-level wrapper](https://crates.io/crates/nrf-modem):

```toml
[dependencies]
nrf-modem = "*"
```

### Versions

The underlying Nordic
[libraries](https://github.com/nrfconnect/sdk-nrfxlib/tags) are not versioned
according to [Rust's semver](https://doc.rust-lang.org/cargo/reference/semver.html) conventions.
<!-- evidence is eg. the removal of the sa_len fields in 2.5.0 -->
Recent versions of this crate adhere to semver to the best of the maintainers' understanding,
and pack the upstream version in the build metadata (i.e., after the `+` sign).

## Licence

Any of the code outside the `./third_party` folder is under the [Blue Oak
Licence](./LICENCE.md). Any code inside the `./third_party` folder (include
the Nordic nrfxlib) has its own LICENCE file.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, shall be licensed as above,
without any additional terms or conditions.

## Changelog

### Unreleased Changes ([Source](https://github.com/nrf-rs/nrfxlib-sys/tree/develop) | [Changes](https://github.com/nrf-rs/nrfxlib-sys/compare/v3.0.3+3.0.2...develop))

### v3.0.3+3.0.2 Changes ([Source](https://github.com/nrf-rs/nrfxlib-sys/tree/v3.0.3+3.0.2) | [Changes](https://github.com/nrf-rs/nrfxlib-sys/compare/v3.0.2...v3.0.3+3.0.2))

* Changed versioning to separate own semver version from upstream library version.
* Include static files for DECT in the published crate. (fixing [#18](https://github.com/nrf-rs/nrfxlib-sys/pull/18))

### v3.0.2 ([Source](https://github.com/nrf-rs/nrfxlib-sys/tree/v3.0.2) | [Changes](https://github.com/nrf-rs/nrfxlib-sys/compare/v2.9.2...v3.0.2))

* Updated to [nrfxlib v3.0.2](https://github.com/NordicPlayground/nrfxlib/tree/v3.0.2)
* Make the DECT-2020 NR+ libmodem.a available as a feature [#16](https://github.com/nrf-rs/nrfxlib-sys/pull/16)

### v2.9.2 ([Source](https://github.com/nrf-rs/nrfxlib-sys/tree/v2.9.2) | [Changes](https://github.com/nrf-rs/nrfxlib-sys/compare/v2.9.1...v2.9.2))

* Disabled bindgen layout tests so this crate can be built for desktops
* Selected a chip feature so docs can be build again

### v2.9.1 ([Source](https://github.com/nrf-rs/nrfxlib-sys/tree/v2.9.1) | [Changes](https://github.com/nrf-rs/nrfxlib-sys/compare/v2.7.1...v2.9.1))

* Updated to [nrfxlib v2.9.1](https://github.com/NordicPlayground/nrfxlib/tree/v2.9.1)
* Added feature `log` for including the binary blobs with logging support.

### v2.7.1 ([Source](https://github.com/nrf-rs/nrfxlib-sys/tree/v2.7.1) | [Changes](https://github.com/nrf-rs/nrfxlib-sys/compare/v2.4.2...v2.7.1))

* Fixed build by including the binary blobs

### v2.7.0 ([Source](https://github.com/nrf-rs/nrfxlib-sys/tree/v2.7.0) | [Changes](https://github.com/nrf-rs/nrfxlib-sys/compare/v2.4.2...v2.7.0))

* Updated to [nrfxlib v2.7.0](https://github.com/NordicPlayground/nrfxlib/tree/v2.7.0)
* Added support for the nRF9151 and nRF9161

### v2.4.2 ([Source](https://github.com/nrf-rs/nrfxlib-sys/tree/v2.4.2) | [Changes](https://github.com/nrf-rs/nrfxlib-sys/compare/v2.1.0...v2.4.2))

* Updated to [nrfxlib v2.4.2](https://github.com/NordicPlayground/nrfxlib/tree/v2.4.2)
* Improved the way the build script searches for `llvm-objcopy`

### v2.1.0 ([Source](https://github.com/nrf-rs/nrfxlib-sys/tree/v2.1.0) | [Changes](https://github.com/nrf-rs/nrfxlib-sys/compare/v1.4.2...v2.1.0))

* Updated to [nrfxlib v2.1.0](https://github.com/NordicPlayground/nrfxlib/tree/v2.1.0)
* Added many oberon crypto functions

### v1.4.2 ([Source](https://github.com/nrf-rs/nrfxlib-sys/tree/v1.4.2) | [Changes](https://github.com/nrf-rs/nrfxlib-sys/compare/v1.2.0...v1.4.2))

* Updated to [nrfxlib v1.4.2](https://github.com/NordicPlayground/nrfxlib/tree/v1.4.2)
* Added new arguments to call to `bsd_init` function.
* Switched back to `bindgen` crate, after Cargo issue #5730 was closed.

### v1.2.0 ([Source](https://github.com/nrf-rs/nrfxlib-sys/tree/v1.2.0) | [Changes](https://github.com/nrf-rs/nrfxlib-sys/compare/v1.1.0-rc3%2Brel1...v1.2.0))

* Updated to [nrfxlib v1.2.0](https://github.com/NordicPlayground/nrfxlib/tree/v1.2.0)
* Fixed some build errors caused by usize/u32 and isize/i32 conversions.

### v1.1.0-rc3+rel1 ([Source](https://github.com/nrf-rs/nrfxlib-sys/tree/v1.1.0-rc3%2Brel1) | [Changes](https://github.com/nrf-rs/nrfxlib-sys/compare/v1.1.0-rc2%2Brel1...v1.1.0-rc3%2Brel1))

* Updated to [nrfxlib v1.1.0-rc3](https://github.com/NordicPlayground/nrfxlib/tree/v1.1.0-rc3)
* Generate headers for `libnrf_cc310` (CryptoCell 310 API)
* Generate headers for `liboberon` (Optimised software crypto implementation)

### v1.1.0-rc2+rel1 ([Source](https://github.com/nrf-rs/nrfxlib-sys/tree/v1.1.0-rc2%2Brel1) | [Changes](https://github.com/nrf-rs/nrfxlib-sys/compare/v0.2.0...v1.1.0-rc2%2Brel1))

* Updated to [nrfxlib v1.1.0-rc2](https://github.com/NordicPlayground/nrfxlib/tree/v1.1.0-rc2), including [libbsd 0.5.0](https://github.com/NordicPlayground/nrfxlib/blob/v1.1.0-rc2/bsdlib/CHANGELOG.rst)
* Changed crate version to track Nordic's nrfxlib version number.

### v0.2.0 ([Source](https://github.com/nrf-rs/nrfxlib-sys/tree/v0.2.0) | [Changes](https://github.com/nrf-rs/nrfxlib-sys/compare/v0.1.7...v0.2.0))

* Require users to install bindgen as a command-line tool.

### v0.1.7 ([Source](https://github.com/nrf-rs/nrfxlib-sys/tree/v0.1.7) | [Changes](https://github.com/nrf-rs/nrfxlib-sys/compare/v0.1.6...v0.1.7))

* Use Cargo 5730 workaround.
* Update bindgen to 0.51
* Rustfmt generated code
* Reformat using tabs

### v0.1.6 ([Source](https://github.com/nrf-rs/nrfxlib-sys/tree/v0.1.6) | [Changes](https://github.com/nrf-rs/nrfxlib-sys/compare/v0.1.5...v0.1.6))

* Bundle the C headers (fixes Travis build)

### v0.1.5 ([Source](https://github.com/nrf-rs/nrfxlib-sys/tree/v0.1.5) | [Changes](https://github.com/nrf-rs/nrfxlib-sys/compare/v0.1.4...v0.1.5))

* Sub-module in upstream nrfxlib.

### v0.1.4 ([Source](https://github.com/nrf-rs/nrfxlib-sys/tree/v0.1.4) | [Changes](https://github.com/nrf-rs/nrfxlib-sys/compare/v0.1.3...v0.1.4))

* Add the bsd_limits.h, nrf_key_mgmt.h and nrf_apn_class.h headers.

### v0.1.3 ([Source](https://github.com/nrf-rs/nrfxlib-sys/tree/v0.1.3) | [Changes](https://github.com/nrf-rs/nrfxlib-sys/compare/v0.1.2...v0.1.3))

* Updates to this README to specify checkout of tag v1.0.0.
* Added a Travis CI file.

### v0.1.2 ([Source](https://github.com/nrf-rs/nrfxlib-sys/tree/v0.1.2) | [Changes](https://github.com/nrf-rs/nrfxlib-sys/compare/v0.1.1...v0.1.2))

* Add NEWLIB_PATH so user can point to correct newlib headers.

### v0.1.1 ([Source](https://github.com/nrf-rs/nrfxlib-sys/tree/v0.1.1) | [Changes](https://github.com/nrf-rs/nrfxlib-sys/compare/v0.1.0...v0.1.1))

* Updates to the README and crate metadata

### v0.1.0 ([Source](https://github.com/nrf-rs/nrfxlib-sys/tree/v0.1.0))

* First version
