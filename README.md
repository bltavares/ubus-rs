# ubus-rs

(prototype)

Expose [ubus](https://openwrt.org/docs/techref/ubus) library to build OpenWRT Rust apps.

## Compile the project

Requires [cross](https://github.com/rust-embedded/cross) installed.

:warning: Has git submodules. :warning:

Tested on:

- Windows WSL (requires Docker)
- Windows (does not work/requires Docker)

```sh
cross build --target mips-unknown-linux-musl -p ubus-cli
```

## Using this as a library to build other projects

Add this as a dependency:

```toml
[dependencies]
ubus-sys = { git = "https://github.com/bltavares/ubus-rs" }
```

Copy the custom `Cargo.toml` file from this project to yours, in order to use an image that works with OpenWRT for cross-compilation.

Then it should be possible to run:

```sh
cross build --target mips-unknown-linux-musl
```

No promises there...
