# Mirage Targets

This directory contains the build targets required to build Mirage for the Nintendo Switch.

## Boot order

The Tegra X1 platform executes the initial bootrom on a small ARM7TDMI, referred to as "BPMP".
The BPMP is also used to execute the first-stage bootloader ("Package1ldr").

Package1ldr then decrypts and passes execution to the actual Package1 ("PK11") that contains
the TrustZone and the second-stage bootloader ("NX-Bootloader"). These are meant to run on the
Cortex-A57.

Ideally speaking, two target files are needed to target both CPUs in the boot chain.
One for the BPMP (ARMv4t), the other one for the Cortex-A57 (AArch64).

## Targets

* [`armv4t-mirage-eabi.json`](./armv4t-mirage-eabi.json)
  * Targets the BPMP
  * Used to build first-stage bootloader

* [`aarch64-mirage-none.json`](./aarch64-mirage-none.json)
  * Targets the Cortex-A57
  * Used to build second-stage bootloader, kernel and system modules
