//! Interface for Memory-Mapped I/O registers.
//!
//! Defines memory regions through pointers which are
//! acessed by [volatile] read/write operations.
//!
//! # Design
//!
//! The [`Mmio`] struct internally wraps around an unsafe raw pointer to
//! an instance of [`RegisterCell`] which provides the underlying methods
//! for accessing hardware registers.
//!
//! [`RegisterCell`] then stores an instance of [`UnsafeCell`] which is
//! supposed to manage the actually supplied pointer to a certain
//! memory region and provides [`RegisterCell::get`] and
//! [`RegisterCell::set`] which perform [volatile] reads/writes. Ideally
//! speaking, [`RegisterCell`] is designed similar to [`Cell`] from `std`.
//!
//! This makes it possible to mutate [`RegisterCell`] in immutable structs.
//! And since it is the underlying construction for the [`Mmio`] struct,
//! this applies to it as well. In other words, it enables "interior
//! mutability" for [`Mmio`]s.
//!
//! Compared to other MMIO implementations, the end user is often required
//! to dereference raw pointers, which is considered unsafe, in order to
//! operate on the registers. This implementation does the dereferencing
//! of the raw pointers internally, however explicitly marks the exposed
//! methods which create instances of the [`Mmio`] structure as unsafe
//! because falsy pointers will trigger Undefined Behavior, which should
//! be avoided.
//!
//! In practice, registers are often mapped as blocks of registers which
//! belong together. These registers can be represented nicely through
//! structures, which should implement [`VolatileStorage`]. It requires
//! you to implement [`VolatileStorage::make_ptr`] which is supposed to
//! return a pointer to the memory address where the register block starts.
//! The unsafe [`VolatileStorage::get`] method can then be used to create
//! a reference of the respectively aligned instance of the structure that
//! represents the register block.
//!
//! [`Mmio::new`] and [`VolatileStorage::get`] are considered unsafe for
//! reasons elaborated below.
//!
//! If you only want to represent certain registers at certain addresses,
//! you can do that through [`Mmio::new`], taking a pointer to the memory
//! address of the register.
//!
//! # Safety
//!
//! The dereferencing of raw pointers and operations on the memory regions
//! is actually done by the unexposed API of [`RegisterCell`]. It takes
//! `*const Self` as its `self` type and dereferences itself (`*self`) at
//! every operation on the underlying memory region. This act may result
//! in dangling pointers, triggering Undefined Behavior, which is to be
//! avoided. And that's why it is intrinsic to pass correct pointers to
//! [`Mmio`]s which uses them to initialize [`RegisterCell`]s.
//!
//! Though end users shouldn't dereference raw pointers in their own code
//! by design, the supplied pointers nonetheless need to be correct and
//! that's why entry points to the creation of instances of the [`Mmio`]
//! structure are marked as unsafe - the general risk to trigger UB is
//! always present.
//!
//! # Usage
//!
//! ``` no_run
//! use mirage_mmio::{Mmio, VolatileStorage};
//!
//! // A struct that represents a memory-mapped register block.
//! // Needs to be made repr(C) so the fields get aligned correctly.
//! #[repr(C)]
//! pub struct RegisterBlock {
//!     pub some_reg: Mmio<u32>,
//!     _unknown: [Mmio<u32>; 4],
//!     pub another_reg: Mmio<u32>,
//! }
//!
//! impl VolatileStorage for RegisterBlock {
//!     fn make_ptr() -> *const Self {
//!         // Return a raw pointer to the memory region
//!         // where this register block is mapped.
//!         0xE000_000 as *const _
//!     }
//! }
//!
//! // A globally defined hardware register which can be accessed directly.
//! const REGISTER: Mmio<u32> = unsafe { Mmio::new(0xF000_0000 as *const _) };
//!
//! fn do_xy() {
//!     // The previously implemented make_ptr is needed
//!     // to create a reference to the memory-mapped
//!     // registers, which can now be accessed.
//!     let registers = unsafe { RegisterBlock::get() };
//!
//!     // Read from a register.
//!     let some_value = registers.some_reg.read();
//!     // Modify the value of another one.
//!     register.another_reg.write(some_value << REGISTER.read());
//! }
//! ```
//!
//! [volatile]: https://doc.rust-lang.org/core/ptr/fn.read_volatile.html
//! [`Mmio`]: struct.Mmio.html
//! [`RegisterCell`]: struct.RegisterCell.html
//! [`UnsafeCell`]: https://doc.rust-lang.org/core/cell/struct.UnsafeCell.html
//! [`RegisterCell::get`]: struct.RegisterCell.html#method.get
//! [`RegisterCell::set`]: struct.RegisterCell.html#method.set
//! [`Cell`]: https://doc.rust-lang.org/std/cell/struct.Cell.html
//! [`VolatileStorage`]: trait.VolatileStorage.html
//! [`VolatileStorage::make_ptr`]: trait.VolatileStorage.html#method.make_ptr
//! [`VolatileStorage::get`]: trait.VolatileStorage.html#method.get

#![no_std]
#![deny(missing_docs)]
#![feature(arbitrary_self_types)]
#![feature(const_fn)]

extern crate num_traits;

use core::{
    cell::UnsafeCell,
    fmt,
    ptr::{read_volatile, write_volatile},
};

use num_traits::PrimInt;

/// A mutable hardware register location in memory.
struct RegisterCell<T: PrimInt> {
    register: UnsafeCell<T>,
}

impl<T: PrimInt> RegisterCell<T> {
    /// Creates a new instance of [`RegisterCell`]
    /// containing the given value.
    ///
    /// NOTE: unsafe because it may trigger
    /// Undefined Behavior for falsy pointers.
    ///
    /// [`RegisterCell`]: struct.RegisterCell.html
    #[allow(dead_code)]
    pub const unsafe fn new(value: T) -> Self {
        RegisterCell {
            register: UnsafeCell::new(value),
        }
    }

    /// Performs a [volatile read] of the underlying
    /// memory region and returns the resulting value.
    ///
    /// NOTE: unsafe because of volatile memory access.
    ///
    /// [volatile read]: https://doc.rust-lang.org/core/ptr/fn.read_volatile.html
    #[inline(always)]
    pub unsafe fn get(self: *const Self) -> T {
        read_volatile((*self).register.get())
    }

    /// Performs a [volatile write] to the underlying
    /// memory region and writes the given value to it.
    ///
    /// NOTE: unsafe because of volatile memory access.
    ///
    /// [volatile write]: https://doc.rust-lang.org/core/ptr/fn.write_volatile.html
    #[inline(always)]
    pub unsafe fn set(self: *const Self, value: T) {
        write_volatile((*self).register.get(), value)
    }
}

/// A trait providing methods for the creation of instances of
/// structures which represent register blocks.
///
/// While [`VolatileStorage::make_ptr`] needs to be implemented,
/// [`VolatileStorage::get`] is always the preferred way for
/// creating references to instances of the register block struct.
///
/// Structures that implement this trait should always be made
/// `repr(C)` so that the pointers to the memory region get aligned
/// correctly.
///
/// NOTE: unsafe methods because `make_ptr` pointing to an invalid
/// memory region may trigger Undefined Behavior.
///
/// [`VolatileStorage::make_ptr`]: trait.VolatileStorage.html#method.make_ptr
/// [`VolatileStorage::get`]: trait.VolatileStorage.html#method.get
pub trait VolatileStorage {
    /// Creates an instance of the underlying structure and returns
    /// a reference to it.
    ///
    /// NOTE: unsafe because it dereferenced the raw pointer created
    /// in [`VolatileStorage::make_ptr`].
    ///
    /// [`VolatileStorage::make_ptr`]: trait.VolatileStorage.html#method.make_ptr
    unsafe fn get<'a>() -> &'a Self {
        &(*Self::make_ptr())
    }

    /// Creates a pointer to the memory region where the register block
    /// is mapped.
    ///
    /// NOTE: unsafe because this pointer will be dereferenced
    /// in [VolatileStorage::get], causing a risk to trigger
    /// Undefined Behavior.
    ///
    /// [`VolatileStorage::get`]: trait.VolatileStorage.html#method.get
    unsafe fn make_ptr() -> *const Self;
}

/// Abstraction of a memory-mapped hardware register.
///
/// Generally used behind a pointer, providing volatile
/// read and write access to the managed memory region.
pub struct Mmio<T: PrimInt> {
    /// A pointer to the underlying [`RegisterCell`],
    /// managing the memory region.
    ///
    /// [`RegisterCell`]: struct.RegisterCell.html
    value: *const RegisterCell<T>,
}

impl<T: PrimInt> Mmio<T> {
    /// Creates a new instance of [`Mmio`]
    /// wrapping the given value.
    ///
    /// NOTE: unsafe because it may trigger
    /// Undefined Behavior.
    ///
    /// [`Mmio`]: struct.Mmio.html
    pub const unsafe fn new(value: *const T) -> Self {
        Mmio {
            value: value as *const _,
        }
    }

    /// Reads the underlying hardware register
    /// and returns the resulting value.
    #[inline(always)]
    pub fn read(&self) -> T {
        unsafe { self.value.get() }
    }

    /// Writes the given value to the
    /// underlying hardware register.
    #[inline(always)]
    pub fn write(&self, value: T) {
        unsafe { self.value.set(value) }
    }
}

impl<T> fmt::Debug for Mmio<T>
where
    T: fmt::Debug + PrimInt,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("Mmio").field("value", &self.read()).finish()
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use alloc::format;

    use crate::Mmio;

    /// Tests volatile reads from registers for correctness.
    #[test]
    fn read_register() {
        let x: i32 = 50;
        let register = unsafe { Mmio::new(&x as *const i32) };

        assert_eq!(50, x);
        assert_eq!(50, register.read());
        assert_eq!(x, register.read());
    }

    /// Tests volatile writes to registers for correctness.
    #[test]
    fn write_to_register() {
        let x: i32 = 50;
        let register = unsafe { Mmio::new(&x as *const i32) };

        assert_eq!(x, register.read());

        register.write(500);

        assert_ne!(50, x);
        assert_eq!(500, x);
        assert_eq!(500, register.read());
    }

    /// Verifies the correctness of debug output.
    #[test]
    fn debug_register() {
        let x: i32 = 50;
        let register = unsafe { Mmio::new(&x as *const i32) };

        // Since the Debug trait obtains the representation of
        // the value field by calling `register.read()`, this
        // also asserts that the values of `x` and
        // `register.read()` are equal.
        assert_eq!(
            format!("Mmio {{ value: {} }}", x),
            format!("{:?}", register)
        );
    }
}
