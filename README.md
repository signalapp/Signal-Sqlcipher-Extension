# Overview

Signal-Sqlcipher-Extension bundles:

- [Signal-FTS5-Extension](https://github.com/signalapp/Signal-FTS5-Extension)
- Rust-based cryptography provider

into a single .a (.lib on Windows) file that could be linked into relevant
sqlcipher builds.

# Build Instructions

For x86_64:
```sh
cargo build --release
```

For arm64
```sh
RUSTFLAGS="--cfg aes_armv8" cargo build --release
```

# Usage

Cryptography provider needs to be registered right before opening the database:
```c
int status = signal_register_crypto_provider();
assert(status == SQLITE_OK);

status = sqlite3_open_v2("path", &db, ...);
assert(status == SQLITE_OK);
```

# Legal things

## License

Copyright 2024 Signal Messenger, LLC.

Licensed under the AGPLv3: http://www.gnu.org/licenses/agpl-3.0.html

