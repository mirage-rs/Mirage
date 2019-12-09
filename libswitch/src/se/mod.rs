//! Tegra210 Security Engine driver.

use core::convert::{TryFrom, TryInto};

use mirage_mmio::{Mmio, VolatileStorage};

/// Base address for SE registers.
pub(crate) const SE_BASE: u32 = 0x7001_2000;

const KEYSLOT_AES_MAX: usize = 0x10;
const KEYSLOT_RSA_MAX: usize = 0x2;

const KEYSIZE_AES_MAX: usize = 0x20;
const KEYSIZE_RSA_MAX: usize = 0x100;

pub const OP_ABORT: u32 = 0;
pub const OP_START: u32 = 1;
pub const OP_RESTART: u32 = 2;
pub const OP_CTX_SAVE: u32 = 3;
pub const OP_RESTART_IN: u32 = 4;

/// Representation of the SE registers.
#[allow(non_snake_case)]
#[repr(C)]
struct Registers {
    _0x0: Mmio<u32>,
    _0x4: Mmio<u32>,
    pub OPERATION_REG: Mmio<u32>,
    pub INT_ENABLE_REG: Mmio<u32>,
    pub INT_STATUS_REG: Mmio<u32>,
    pub CONFIG_REG: Mmio<u32>,
    pub IN_LL_ADDR_REG: Mmio<u32>,
    _0x1C: Mmio<u32>,
    _0x20: Mmio<u32>,
    pub OUT_LL_ADDR_REG: Mmio<u32>,
    _0x28: Mmio<u32>,
    _0x2C: Mmio<u32>,
    pub HASH_RESULT_REG: [Mmio<u8>; 0x20],
    _0x50: [Mmio<u8>; 0x20],
    pub CONTEXT_SAVE_CONFIG_REG: Mmio<u32>,
    _0x74: [Mmio<u8>; 0x18C],
    pub SHA_CONFIG_REG: Mmio<u32>,
    pub SHA_MSG_LENGTH_REG: Mmio<u32>,
    _0x208: Mmio<u32>,
    _0x20C: Mmio<u32>,
    _0x210: Mmio<u32>,
    pub SHA_MSG_LEFT_REG: Mmio<u32>,
    _0x218: Mmio<u32>,
    _0x21C: Mmio<u32>,
    _0x220: Mmio<u32>,
    _0x224: Mmio<u32>,
    _0x228: [Mmio<u8>; 0x58],
    pub AES_KEY_READ_DISABLE_REG: Mmio<u32>,
    pub AES_KEYSLOT_FLAGS: [Mmio<u32>; 0x10],
    _0x2C4: [Mmio<u8>; 0x3C],
    _0x300: Mmio<u32>,
    pub CRYPTO_REG: Mmio<u32>,
    pub CRYPTO_CTR_REG: [Mmio<u32>; 4],
    pub BLOCK_COUNT_REG: Mmio<u32>,
    pub AES_KEYTABLE_ADDR: Mmio<u32>,
    pub AES_KEYTABLE_DATA: Mmio<u32>,
    _0x324: Mmio<u32>,
    _0x328: Mmio<u32>,
    _0x32C: Mmio<u32>,
    pub CRYPTO_KEYTABLE_DST_REG: Mmio<u32>,
    _0x334: [Mmio<u8>; 0xC],
    pub RNG_CONFIG_REG: Mmio<u32>,
    pub RNG_SRC_CONFIG_REG: Mmio<u32>,
    pub RNG_RESEED_INTERVAL_REG: Mmio<u32>,
    _0x34C: [Mmio<u8>; 0xB4],
    pub RSA_CONFIG: Mmio<u32>,
    pub RSA_KEY_SIZE_REG: Mmio<u32>,
    pub RSA_EXP_SIZE_REG: Mmio<u32>,
    pub RSA_KEY_READ_DISABLE_REG: Mmio<u32>,
    pub RSA_KEYSLOT_FLAGS: [Mmio<u32>; 2],
    _0x418: Mmio<u32>,
    _0x41C: Mmio<u32>,
    pub RSA_KEYTABLE_ADDR: Mmio<u32>,
    pub RSA_KEYTABLE_DATA: Mmio<u32>,
    pub RSA_OUTPUT: [Mmio<u8>; 0x100],
    _0x528: [Mmio<u8>; 0x2D8],
    pub FLAGS_REG: Mmio<u32>,
    pub ERR_STATUS_REG: Mmio<u32>,
    _0x808: Mmio<u32>,
    pub SPARE_0: Mmio<u32>,
    _0x810: Mmio<u32>,
    _0x814: Mmio<u32>,
    _0x818: Mmio<u32>,
    _0x81C: Mmio<u32>,
    _0x820: [Mmio<u8>; 0x17E0],
}

impl VolatileStorage for Registers {
    unsafe fn make_ptr() -> *const Self {
        SE_BASE as *const _
    }
}

/// Representation of the Security Engine.
pub struct SecurityEngine {
    /// A buffer to keep track of the modulus sizes for RSA keyslots.
    modulus_sizes: [usize; KEYSLOT_RSA_MAX],
    /// A buffer to keep track of the exponent sizes for RSA keyslots.
    exponent_sizes: [usize; KEYSLOT_RSA_MAX],
}

/// Simplified version of a SE LL.
#[repr(C)]
struct Ll {
    /// The number of entries. Should always be set to 0, i.e. 1 entry.
    pub entries: u32,
    /// The address of the buffer to be used.
    pub address: u32,
    /// The size of the buffer.
    pub size: u32,
}

impl Ll {
    /// Creates a new LL object.
    pub fn new(buffer: &mut [u8]) -> Self {
        Ll {
            entries: 0,
            address: u32::try_from(buffer.as_mut_ptr() as usize).expect("Value must fit an u32."),
            size: buffer.len() as u32,
        }
    }
}

// TODO(Vale): How to design the panic handler in favor of thrown panics?

impl SecurityEngine {
    fn trigger_blocking_operation(&self, op: u32, destination: &mut [u8], source: &mut [u8]) {
        let register_base = unsafe { Registers::get() };

        // Create and set the LLs.
        let mut in_ll = Ll::new(source);
        let mut out_ll = Ll::new(destination);

        register_base
            .IN_LL_ADDR_REG
            .write(&mut in_ll as *mut _ as usize as u32);
        register_base
            .OUT_LL_ADDR_REG
            .write(&mut out_ll as *mut _ as usize as u32);

        // Set registers for operation.
        register_base.ERR_STATUS_REG.write(register_base.ERR_STATUS_REG.read());
        register_base.INT_STATUS_REG.write(register_base.INT_STATUS_REG.read());
        register_base.OPERATION_REG.write(op);

        while register_base.INT_STATUS_REG.read() & 0x10 == 0 {
            // Wait.
        }

        self.check_for_error();
    }

    /// Creates a new Security Engine object.
    pub const fn new() -> Self {
        SecurityEngine {
            modulus_sizes: [0; KEYSLOT_RSA_MAX],
            exponent_sizes: [0; KEYSLOT_RSA_MAX],
        }
    }

    /// Locks the SBK from being read.
    #[inline]
    pub(crate) fn lock_sbk(&self) {
        let register_base = unsafe { Registers::get() };

        register_base.AES_KEYSLOT_FLAGS[0xE].write(0x7E);
    }

    /// Locks the SSK from being read.
    #[inline]
    pub(crate) fn lock_ssk(&self) {
        let register_base = unsafe { Registers::get() };

        register_base.AES_KEYSLOT_FLAGS[0xF].write(0x7E);
    }

    /// Sets the `INT_STATUS_REG` to `0x1F`.
    #[inline]
    pub(crate) fn config_brom(&self) {
        let register_base = unsafe { Registers::get() };

        register_base.INT_STATUS_REG.write(0x1F);
    }

    /// Checks the ERR_STATUS_REG and panics if the value isn't zero.
    #[inline(always)]
    pub fn check_error_status_reg(&self) {
        let register_base = unsafe { Registers::get() };

        if register_base.ERR_STATUS_REG.read() != 0 {
            panic!();
        }
    }

    /// Verifies that all flags are cleared and panics otherwise.
    #[inline(always)]
    pub fn verify_flags_cleared(&self) {
        let register_base = unsafe { Registers::get() };

        if register_base.FLAGS_REG.read() & 3 != 0 {
            panic!();
        }
    }

    /// Checks for general SE errors and panics in case there are any.
    #[inline]
    pub fn check_for_error(&self) {
        let register_base = unsafe { Registers::get() };

        self.check_error_status_reg();

        self.verify_flags_cleared();

        if register_base.INT_STATUS_REG.read() & 0x10000 != 0 {
            panic!();
        }
    }

    /// Sets the flags for an AES keyslot.
    pub fn set_aes_keyslot_flags(&self, keyslot: usize, flags: u32) {
        let register_base = unsafe { Registers::get() };

        if keyslot >= KEYSLOT_AES_MAX {
            panic!();
        }

        // Miscellaneous flags.
        if flags & !0x80 != 0 {
            register_base.AES_KEYSLOT_FLAGS[keyslot].write(flags);
        }

        // Disable keyslot reads.
        if flags & 0x80 != 0 {
            let value = register_base.AES_KEY_READ_DISABLE_REG.read();
            register_base.AES_KEY_READ_DISABLE_REG.write(value & !(1 << keyslot as u32));
        }
    }

    /// Sets the flags for an RSA keyslot.
    pub fn set_rsa_keyslot_flags(&self, keyslot: usize, flags: u32) {
        let register_base = unsafe { Registers::get() };

        if keyslot >= KEYSLOT_RSA_MAX {
            panic!();
        }

        // Miscellaneous flags.
        if flags & !0x80 != 0 {
            register_base.RSA_KEYSLOT_FLAGS[keyslot].write((((flags >> 4) & 4) | (flags & 3)) ^ 7);
        }

        // Disable keyslot reads.
        if flags & 0x80 != 0 {
            let value = register_base.RSA_KEY_READ_DISABLE_REG.read();
            register_base.RSA_KEY_READ_DISABLE_REG.write(value & !(1 << keyslot as u32));
        }
    }

    /// Clears an AES keyslot.
    pub fn clear_aes_keyslot(&self, keyslot: usize) {
        let register_base = unsafe { Registers::get() };

        if keyslot >= KEYSLOT_AES_MAX {
            panic!();
        }

        // Zero out the whole keyslot and IV.
        for i in 0..0x10 {
            register_base.AES_KEYTABLE_ADDR.write(((keyslot << 4) | i) as u32);
            register_base.AES_KEYTABLE_DATA.write(0);
        }
    }

    /// Clears an RSA keyslot.
    pub fn clear_rsa_keyslot(&self, keyslot: usize) {
        let register_base = unsafe { Registers::get() };

        if keyslot >= KEYSLOT_RSA_MAX {
            panic!();
        }

        // Zero out the whole keyslot.
        for i in 0..0x40 {
            // Select Keyslot Modulus[i].
            register_base.RSA_KEYTABLE_ADDR.write(((keyslot << 7) | i | 0x40) as u32);
            register_base.RSA_KEYTABLE_DATA.write(0);
        }
        for i in 0..0x40 {
            // Select Keyslot Exponent[i].
            register_base.RSA_KEYTABLE_ADDR.write(((keyslot << 7) | i) as u32);
            register_base.RSA_KEYTABLE_DATA.write(0);
        }
    }

    /// Sets an AES keyslot.
    pub fn set_aes_keyslot(&self, keyslot: usize, key: &[u8]) {
        let register_base = unsafe { Registers::get() };

        let keysize = key.len();

        if keyslot >= KEYSLOT_AES_MAX || keysize > KEYSIZE_AES_MAX {
            panic!();
        }

        for i in 0..keysize >> 2 {
            register_base.AES_KEYTABLE_ADDR.write(((keyslot << 4) | i) as u32);
            register_base.AES_KEYTABLE_DATA
                .write(u32::from_le_bytes((&key[..4 * i]).try_into().unwrap()));
        }
    }

    /// Sets an RSA keyslot.
    pub fn set_rsa_keyslot(&mut self, keyslot: usize, modulus: &[u8], exponent: &[u8]) {
        let register_base = unsafe { Registers::get() };

        let modulus_size = modulus.len();
        let exponent_size = exponent.len();

        if keyslot >= KEYSLOT_RSA_MAX
            || modulus_size > KEYSIZE_RSA_MAX
            || exponent_size > KEYSIZE_RSA_MAX
        {
            panic!();
        }

        for i in 0..modulus_size >> 2 {
            register_base.RSA_KEYTABLE_ADDR.write(((keyslot << 7) | 0x40 | i) as u32);
            register_base.RSA_KEYTABLE_DATA.write(u32::from_be_bytes(
                (&modulus[..(4 * (modulus_size >> 2)) - (4 * i) - 4])
                    .try_into()
                    .unwrap(),
            ));
        }

        for i in 0..exponent_size >> 2 {
            register_base.RSA_KEYTABLE_ADDR.write(((keyslot << 7) | i) as u32);
            register_base.RSA_KEYTABLE_DATA.write(u32::from_be_bytes(
                (&exponent[..(4 * (exponent_size >> 2)) - (4 * i) - 4])
                    .try_into()
                    .unwrap(),
            ));
        }

        self.modulus_sizes[keyslot] = modulus_size;
        self.exponent_sizes[keyslot] = exponent_size;
    }

    /// Sets the IV of the AES keyslot.
    pub fn set_aes_keyslot_iv(&self, keyslot: usize, iv: &[u8]) {
        let register_base = unsafe { Registers::get() };

        let iv_size = iv.len();

        if keyslot >= KEYSLOT_AES_MAX || iv_size > 0x10 {
            panic!();
        }

        for i in 0..iv_size >> 2 {
            register_base.AES_KEYTABLE_ADDR.write(((keyslot << 4) | 8 | i) as u32);
            register_base.AES_KEYTABLE_DATA
                .write(u32::from_le_bytes((&iv[..4 * i]).try_into().unwrap()));
        }
    }

    /// Clears the IV of the AES keyslot.
    pub fn clear_aes_keyslot_iv(&self, keyslot: usize) {
        let register_base = unsafe { Registers::get() };

        if keyslot >= KEYSLOT_AES_MAX {
            panic!();
        }

        for i in 0..0x10 >> 2 {
            register_base.AES_KEYTABLE_ADDR.write(((keyslot << 4) | 8 | i) as u32);
            register_base.AES_KEYTABLE_DATA.write(0);
        }
    }

    /// Sets the CRYPTO_CTR_REG to enable CTR mode.
    pub fn set_ctr(&self, ctr: &[u8]) {
        let register_base = unsafe { Registers::get() };

        for i in 0..4 {
            register_base.CRYPTO_CTR_REG[i]
                .write(u32::from_le_bytes((&ctr[..4 * i]).try_into().unwrap()));
        }
    }

    /// Decrypts data from a given keyslot into another keyslot.
    pub fn decrypt_data_into_keyslot(
        &self,
        destination: usize,
        source: usize,
        wrapped_key: &mut [u8],
    ) {
        let register_base = unsafe { Registers::get() };

        if destination >= KEYSLOT_AES_MAX
            || source >= KEYSLOT_AES_MAX
            || wrapped_key.len() > KEYSIZE_AES_MAX
        {
            panic!();
        }

        register_base.CONFIG_REG.write(0x108);
        register_base.CRYPTO_REG.write((source << 24) as u32);
        register_base.BLOCK_COUNT_REG.write(0);
        register_base.CRYPTO_KEYTABLE_DST_REG.write((destination << 8) as u32);

        self.trigger_blocking_operation(OP_START, &mut [0; 0], wrapped_key);
    }

    /// Performs a blocking AES operation.
    pub fn perform_aes_block_operation(&self, destination: &mut [u8], source: &mut [u8]) {
        let register_base = unsafe { Registers::get() };

        if source.len() > 0x10 || destination.len() > 0x10 {
            panic!();
        }

        // Trigger AES operation.
        register_base.BLOCK_COUNT_REG.write(0);
        self.trigger_blocking_operation(OP_START, destination, source);
    }
}
