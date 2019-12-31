# Mirage

[![Rust](https://img.shields.io/badge/rust-nightly%202019--12--15-93450a.svg)](https://www.rust-lang.org/)
[![Discord](https://img.shields.io/discord/644185512013463552?color=blue)](https://discord.gg/rJNsDfk)
[![License](https://img.shields.io/badge/license-Apache--2.0%2FMIT-blue)](./LICENSE)
[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2Fvbe0201%2FMirage.svg?type=shield)](https://app.fossa.io/projects/git%2Bgithub.com%2Fvbe0201%2FMirage?ref=badge_shield)
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

## FAQ

There are some frequently asked questions that come up every now and then.
This section is dedicated to answering those.

> Where does it differ from CFW projects, such as Atmosphère or ReiNX?

The aim of these projects is to patch and customize the behavior of Horizon OS,
to make the platform more open and allow for unsigned code execution. Ideally
speaking, these projects are similar to a Jailbreak on your iDevice.

Mirage, on the other hand, aims for a fully-featured reimplementation of the
entire Operating System, without depending on Nintendo's firmware, however
providing the same functionality.

> Why are you doing this?

Fun, research, and as a tribute to the wonderful and modern architecture of Horizon.

Some milestones for the far future:

* [ ] Very accurate reflection of the actual Horizon OS.

* [ ] Getting commercial games and homebrew applications to boot
  * [ ] Providing a Rust toolchain for application development

* [ ] Providing build options for purposefully including Horizon's flaws
  * Eases up research and exploit development, due to the open platform.

* [ ] Getting Mirage to run on other architectures, such as x86
  * Why not?

## License

Mirage is distributed under the terms of either the Apache License (Version 2.0)
or the MIT license, at the user's choice.

See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for details.

[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2Fvbe0201%2FMirage.svg?type=large)](https://app.fossa.io/projects/git%2Bgithub.com%2Fvbe0201%2FMirage?ref=badge_large)
