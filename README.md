# nrfxlib-sys

This is a Rust wrapper for the Nordic nrfxlib driver, specifically libbsd for
the nRF9160.

You will need to set the NRFXLIB_PATH environment variable to point to a
checkout of https://github.com/NordicPlayground/nrfxlib. Be sure to comply
with Nordic's licence for that repository.

You will also need to set the NEWLIB_PATH environment variable to point
to a locally installed copy of the newlib C library headers. This is
because Nordic's nrfxlib headers include `<sys/types.h>`, amongst
other things. On Ubuntu, you might use `/usr/include/newlib`. If you
have installed a GCC release from Arm, look in `/opt/gcc*` somewhere.

```console
user@machine:~ $ git clone https://github.com/NordicPlayground/nrfxlib
user@machine:~ $ cd ~/nrfxlib
user@machine:~/nrfxlib $ git checkout tags/v1.0.0
user@machine:~/nrfxlib $ cd ..
user@machine:~ $ export NRFXLIB_PATH=~/nrfxlib
user@machine:~ $ export NEWLIB_PATH=/usr/include/newlib
user@machine:~ $ cd some_project
user@machine:~/some_project $ cargo build
```

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

## Licence

Any of the code in this specific repository is under the [Blue Oak
Licence](./LICENCE.md). The Nordic components that you build this library with
are under their own licence.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, shall be licensed as above,
without any additional terms or conditions.

## Changelog

### Unreleased Changes ([Source](https://github.com/thejpster/nrfxlib-sys/tree/master) | [Changes](https://github.com/thejpster/nrfxlib-sys/compare/v0.1.3...master))

* None

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
