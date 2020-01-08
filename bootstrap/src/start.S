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
    beq start

    // Copy a relocator to the upper IRAM.
    adr r2, relocator
    ldr r3, =__relocator_address__
    adr r4, relocator_end
    sub r4, r4, r2

    copy_relocator_loop:
    ldmia r2!, {r5}
    stmia r3!, {r5}
    subs r4, #4
    bne copy_relocator_loop

    // Use the relocator to copy the payload to the right place.
    ldr r2, =__payload_end__
    sub r2, r2, r0
    ldr r3, =start

    ldr r4, =__relocator_address__
    bx r4

//
// Used to copy the RCM payload data to the
// start of the execution stack and jump to
// the actual application entrypoint.
//
// Since the payload data is aligned to 16-byte
// boundary, this is safe to use.
//
// The `relocator_end` symbol is used to
// determine relocator length for copying
// it to a safe location in IRAM.
//
//   r0 - Destination address.
//   r1 - Source address.
//   r2 - Length in bytes.
//   r3 - Application entrypoint.
//
// Destroys r0-r7.
//
.align 4
relocator:
    // Relocate the payload.
    ldmia r1!, {r4-r7}
    stmia r0!, {r4-r7}
    subs r2, #0x10
    bne relocator

    // Jump to the relocated entry.
    bx r3
relocator_end:

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
    // Set the stack pointer and frame pointer.
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
