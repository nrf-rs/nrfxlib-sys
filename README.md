# nrfxlib-sys

This is a Rust wrapper for the Nordic nrfxlib driver, specifically libbsd for the nRF9160.

You will need to set the NRFXLIB_PATH environment variable to point to a
checkout of https://github.com/NordicPlayground/nrfxlib. Be sure to comply
with Nordic's licence for that repository.

Any binary which uses this crate is going to need to provide a bunch of C library functions, because Nordic's library expects them. This includes, but is not limited to:

* atoi
* snprintf
* strol

You can't just link `newlib`, because that defines `memset` which clashes with
the `compiler-builtin` definition of `memset`. Answers on a post-card please.

## Licence

Any of the code in this specific repository is under the Blue Oak Licence,
below. The Nordic components that you build this library with are under their
own licence.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, shall be licensed as above,
without any additional terms or conditions.
