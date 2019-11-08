//! Internal utilities and helpers for libswitch.

/// Helper macro to create ready-to-use-ish global references to MMIO registers.
#[macro_export]
macro_rules! register {
    ($name:ident, $addr:expr) => {
        ::paste::item! {
            lazy_static! {
                pub(crate) static ref $name: &'static ::register::mmio::ReadWrite<u32> =
                    unsafe { &*($addr as *const ::register::mmio::ReadWrite<u32>) };
            }
        }
    };
}
