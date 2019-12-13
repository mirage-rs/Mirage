//
// mirage-bootstrap
// ----------------
//
// `mirage-bootstrap` is Mirage's initial first-stage
// bootloader, responsible for initializing hardware
// and chainloading the second-stage bootloader.
//
// As this compiles down to a baremetal ARM payload,
// the very first stage of the bootstrap is implemented
// in Assembly for convenience.
//
// After injecting the payload through the CVE-2018-6242
// ("Fusée Gelée") vulnerability, it will relocate to
// the stack start, if necessary, and jump to the second
// bootstrap stage which is implemented in Rust.
//

.extern memset
.type memset, %function

.extern main
.type main, %function

.section .text.crt0, "ax", %progbits
.arm
.global _start
.type _start, %function

_start:

    // Check if it is necessary to relocate.
    ldr r0, =__payload_start__
    adr r1, _start
    cmp r0, r1
    bne start

    // Copy data to the start of SDRAM.
    // Alignment is declared to 16-byte bounadry, so this is safe.
    ldr r2, =__payload_end__
    sub r2, r2, r0

    relocation_loop:
    ldmia r1!, {r4-r7}
    stmia r0!, {r4-r7}
    subs r2, #0x10
    bne relocation_loop

    // Jump to the application start.
    ldr r3, =start
    bx r3

//
// The actual entrypoint to the bootstrap.
//
// The implementation is derived from Package1ldr's
// initialization routine, except with a few additions.
//
// See: https://switchbrew.org/wiki/Package1#Initialization
//
// Sets sp and fp.
//
start:

    // Set the stack and frame pointer, respectively.
    ldr sp, =__stack_top__
    mov fp, #0

    // Clear .bss section.
    ldr r0, =__bss_start__
    mov r1, #0
    ldr r2, =__bss_end__
    sub r2, r2, r0
    bl memset

    // Jump to main.
    bl main

    // Infinite loop.
    b .
