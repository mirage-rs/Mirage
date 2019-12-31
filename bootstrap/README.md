# Mirage Bootstrap

`mirage-bootstrap` is Mirage's first-stage bootloader that runs under the
Boot and Power Management Processor- Lite. It is similar to the design of
[Package1ldr](https://switchbrew.org/wiki/Package1#Package1ldr) and is
responsible for initializing the hardware and loading the second-stage
bootloader, which runs under the CCPLEX (Arm Cortex-A57).

It is laid out to be injected through
[CVE-2018-6242](https://nvd.nist.gov/vuln/detail/CVE-2018-6242), which
allows for a full takeover of the BPMP, making it possible to run
arbitrary code.

It also contains an additional debug interface that can be used to poke
the Tegra ReCovery Mode (RCM).

## Features

* [x] RCM payload relocation

* [x] Hardware initialization

* [ ] Detailed panic handling

* [ ] Debugging functionality (USB, UART, display, ...)

* [ ] Booting CCPLEX and chainloading other payloads

## Compiling

As for all other components, [`cargo-make`](https://github.com/sagiegurari/cargo-make) is
required to build the bootstrap. It takes care of all the build dependencies.

Install it with:

```shell script
cargo install --force cargo-make
```

Then you can build the bootstrap:

```shell script
# Debug build
cargo make bootstrap

# Release build
cargo make bootstrap --profile production
```

Debug builds are useful if you need additional symbols and buildinfo, for example
if you want to load the payload into IDA.

## Credits

* [roblabla](https://github.com/roblabla), [Thog](https://github.com/Thog), and
[leo60228](https://github.com/leo60228) for lots of advice, troubleshooting assistance
and Rust support

* [Thog](https://github.com/Thog) for the `rboot` project where the `tegra210` module was
a great inspirational source to start off with

* [SwitchBrew](https://switchbrew.org/wiki/Main_Page) for their extensive research
and documentation pertaining to the Nintendo Switch

* [CTCaer](https://github.com/CTCaer) for the `hekate` project and the late-night hours
of bugfixing and consultance pertaining to the Switch hardware

* [rust-embedded](https://github.com/rust-embedded) and [rust-osdev](https://github.com/rust-osdev)
for some great documentation and tools pertaining to Rust on the bare metal

* Nvidia for their TRM, which contains tons of useless stuff, but also some helpful documentation

* Again Nvidia for their SoC design that gave me a headache almost every day
