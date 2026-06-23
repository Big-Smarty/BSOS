# BSOS

BSOS is a Rust x86_64 hobby operating system. It is a passion project heavily based on Philipp Oppermann's [Writing an OS in Rust](https://os.phil-opp.com/) series.

The repository is split into two parts:

- `kernel/`: the `no_std`, `no_main` kernel crate that targets `x86_64-unknown-none`.
- Root crate: a launcher/build wrapper that builds the kernel artifact, creates BIOS and UEFI disk images with `bootloader`, and starts `qemu-system-x86_64`.

## Features

The current kernel includes:

- VGA text output and serial output.
- GDT and IDT initialization.
- PIC setup with timer and keyboard interrupts.
- Page table setup from bootloader-provided memory information.
- Heap allocation through the kernel allocator.
- A simple async task executor.
- Keyboard input handling through an async scancode stream.

## Requirements

BSOS uses nightly Rust because it depends on unstable Cargo and compiler features, including artifact dependencies, custom test frameworks, JSON target specifications, and kernel-oriented `build-std` support. No stable Rust support is claimed.

Install the required Rust component:

```sh
rustup +nightly component add rust-src
```

You also need QEMU available on your `PATH`:

```sh
qemu-system-x86_64 --version
```

## Running

Run the UEFI image:

```sh
cargo +nightly run -- uefi
```

Run the BIOS image:

```sh
cargo +nightly run -- bios
```

Both commands build the kernel, create the requested disk image, and launch it in QEMU. Serial output is attached to the terminal.

## Tests

Kernel tests are QEMU-based and use custom test entry points rather than the standard Rust test harness. They rely on the nightly configuration in `kernel/.cargo/config.toml`, the custom target specification in `kernel/x86_64-bsos.json`, and QEMU's `isa-debug-exit` device for reporting success or failure.

Some tests are integration-style kernel binaries under `kernel/tests/`. Treat the test setup as kernel-specific infrastructure rather than a stable host test suite.

## License

This project is licensed under the MIT License. See [LICENSE.md](LICENSE.md).
