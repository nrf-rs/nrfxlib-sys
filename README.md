# nrfxlib-sys

This is a Rust wrapper for the Nordic nrfxlib driver, specifically libbsd for
the nRF9160.

Any binary which uses this crate is going to need to provide a bunch of C
library functions, because Nordic's library expects them. This includes, but
is not limited to:

* atoi
* snprintf
* strol

You can't just link `newlib`, because that defines `memset` which clashes with
the `compiler-builtin` crate's definition of `memset`. Answers on a post-card
please - for now I'm using
[tinyrlibc](https://github.com/thejpster/tinyrlibc).

## Using

Because this crate uses `cargo-5730` to avoid putting `bindgen` in your build-deps (which then causes conflicts with other dependencies you might have, thanks to Cargo issue #5730), I can't currently publish it to crates.io. You must therefore either pull it in to your project using a git dependency:

```toml
[dependencies]
nrfxlib-sys = { git = "https://github.com/thejpster/nrfxlib-sys", tag="v0.1.7" }
```

Or, you can add it as a git-submodule:

```console
~/my_project$ git submodule add https://github.com/thejpster/nrfxlib-sys                                                                           Cloning into '~/my_project/nrfxlib-sys'...
remote: Enumerating objects: 150, done.                                                                                                remote: Counting objects: 100% (150/150), done.                                                                                        remote: Compressing objects: 100% (105/105), done.                                                                                     remote: Total 150 (delta 76), reused 104 (delta 35), pack-reused 0                                                                     Receiving objects: 100% (150/150), 21.32 KiB | 559.00 KiB/s, done.
Resolving deltas: 100% (76/76), done.

~/my_project$ cat Cargo.toml | grep nrfxlib-sys
nrfxlib-sys = { path = "./nrfxlib-sys" }
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

### Unreleased Changes ([Source](https://github.com/thejpster/nrfxlib-sys/tree/master) | [Changes](https://github.com/thejpster/nrfxlib-sys/compare/v0.1.7...master))

### v0.1.7 ([Source](https://github.com/thejpster/nrfxlib-sys/tree/v0.1.7) | [Changes](https://github.com/thejpster/nrfxlib-sys/compare/v0.1.6...v0.1.7))

* Use Cargo 5730 workaround.
* Update bindgen to 0.51
* Rustfmt generated code
* Reformat using tabs

### v0.1.6 ([Source](https://github.com/thejpster/nrfxlib-sys/tree/v0.1.6) | [Changes](https://github.com/thejpster/nrfxlib-sys/compare/v0.1.5...v0.1.6))

* Bundle the C headers (fixes Travis build)

### v0.1.5 ([Source](https://github.com/thejpster/nrfxlib-sys/tree/v0.1.5) | [Changes](https://github.com/thejpster/nrfxlib-sys/compare/v0.1.4...v0.1.5))

* Sub-module in upstream nrfxlib.

### v0.1.4 ([Source](https://github.com/thejpster/nrfxlib-sys/tree/v0.1.4) | [Changes](https://github.com/thejpster/nrfxlib-sys/compare/v0.1.3...v0.1.4))

* Add the bsd_limits.h, nrf_key_mgmt.h and nrf_apn_class.h headers.

### v0.1.3 ([Source](https://github.com/thejpster/nrfxlib-sys/tree/v0.1.3) | [Changes](https://github.com/thejpster/nrfxlib-sys/compare/v0.1.2...v0.1.3))

* Updates to this README to specify checkout of tag v1.0.0.
* Added a Travis CI file.

### v0.1.2 ([Source](https://github.com/thejpster/nrfxlib-sys/tree/v0.1.2) | [Changes](https://github.com/thejpster/nrfxlib-sys/compare/v0.1.1...v0.1.2))

* Add NEWLIB_PATH so user can point to correct newlib headers.

### v0.1.1 ([Source](https://github.com/thejpster/nrfxlib-sys/tree/v0.1.1) | [Changes](https://github.com/thejpster/nrfxlib-sys/compare/v0.1.0...v0.1.1))

* Updates to the README and crate metadata

### v0.1.0 ([Source](https://github.com/thejpster/nrfxlib-sys/tree/v0.1.0))

* First version
