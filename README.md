# Mirage

[![Built with cargo-make](https://sagiegurari.github.io/cargo-make/assets/badges/cargo-make.svg)](https://sagiegurari.github.io/cargo-make)

Mirage is a reimplementation of the Nintendo Switch firmware, based on
reverse-engineering results. It (obviously) targets the Switch itself.

## Components

* [`bootstrap`](./bootstrap): The initial first-stage bootloader

* [`libtegra`](./libtegra): Low-level hardware access library for the Switch

* [`linker-scripts`](./linker-scripts): Various linker scripts used for the build

* [`mmio`](./mmio): Memory-Mapped I/O abstractions for Rust

* [`targets`](./targets): Rust target specifications for the Switch

## Support

If something isn't working as expected or you have questions, feel free to open
an [issue](https://github.com/vbe0201/Mirage/issues).

If you're looking for a more direct way to contact the developers, we have a
Discord server [right here](https://discord.gg/rJNsDfk).

## Compiling

If you want to compile individual components of Mirage, please take a look at
the README files in the respective directories for more specific instructions.

For a full build of Mirage, [`cargo-make`](https://github.com/sagiegurari/cargo-make)
is required.  It takes care of all the build dependencies.

Install it with:

```shell script
cargo install --force cargo-make
```

Then you can build Mirage:

```shell script
cargo make --profile production
```

Debug builds (default profile) aren't recommended here, please consider
building the individual components respectively.

## Roadmap

Mirage is very young and under heavy development. You can view development
items and their progress state in the
[Mirage project boards](https://github.com/vbe0201/Mirage/projects).

## Contributing

*Coming soon.*

## License

Mirage is distributed under the terms of either the Apache License (Version 2.0)
or the MIT license, at the user's choice.

See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for details.
