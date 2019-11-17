//! Tegra210 Security Engine driver.

use core::convert::{TryFrom, TryInto};

use register::mmio::ReadWrite;

/// Base address for SE registers.
const SE_BASE: u32 = 0x7001_2000;

const KEYSLOT_AES_MAX: usize = 0x10;
const KEYSLOT_RSA_MAX: usize = 0x2;

const KEYSIZE_AES_MAX: usize = 0x20;
const KEYSIZE_RSA_MAX: usize = 0x100;

const OP_ABORT: u32 = 0;
const OP_START: u32 = 1;
const OP_RESTART: u32 = 2;
const OP_CTX_SAVE: u32 = 3;
const OP_RESTART_IN: u32 = 4;

/// Representation of the SE registers.
#[allow(non_snake_case)]
#[repr(C)]
struct Registers {
    _0x0: ReadWrite<u32>,
    _0x4: ReadWrite<u32>,
    pub OPERATION_REG: ReadWrite<u32>,
    pub INT_ENABLE_REG: ReadWrite<u32>,
    pub INT_STATUS_REG: ReadWrite<u32>,
    pub CONFIG_REG: ReadWrite<u32>,
    pub IN_LL_ADDR_REG: ReadWrite<u32>,
    _0x1C: ReadWrite<u32>,
    _0x20: ReadWrite<u32>,
    pub OUT_LL_ADDR_REG: ReadWrite<u32>,
    _0x28: ReadWrite<u32>,
    _0x2C: ReadWrite<u32>,
    pub HASH_RESULT_REG: [ReadWrite<u8>; 0x20],
    _0x50: [ReadWrite<u8>; 0x20],
    pub CONTEXT_SAVE_CONFIG_REG: ReadWrite<u32>,
    _0x74: [ReadWrite<u8>; 0x18C],
    pub SHA_CONFIG_REG: ReadWrite<u32>,
    pub SHA_MSG_LENGTH_REG: ReadWrite<u32>,
    _0x208: ReadWrite<u32>,
    _0x20C: ReadWrite<u32>,
    _0x210: ReadWrite<u32>,
    pub SHA_MSG_LEFT_REG: ReadWrite<u32>,
    _0x218: ReadWrite<u32>,
    _0x21C: ReadWrite<u32>,
    _0x220: ReadWrite<u32>,
    _0x224: ReadWrite<u32>,
    _0x228: [ReadWrite<u8>; 0x58],
    pub AES_KEY_READ_DISABLE_REG: ReadWrite<u32>,
    pub AES_KEYSLOT_FLAGS: [ReadWrite<u32>; 0x10],
    _0x2C4: [ReadWrite<u8>; 0x3C],
    _0x300: ReadWrite<u32>,
    pub CRYPTO_REG: ReadWrite<u32>,
    pub CRYPTO_CTR_REG: [ReadWrite<u32>; 4],
    pub BLOCK_COUNT_REG: ReadWrite<u32>,
    pub AES_KEYTABLE_ADDR: ReadWrite<u32>,
    pub AES_KEYTABLE_DATA: ReadWrite<u32>,
    _0x324: ReadWrite<u32>,
    _0x328: ReadWrite<u32>,
    _0x32C: ReadWrite<u32>,
    pub CRYPTO_KEYTABLE_DST_REG: ReadWrite<u32>,
    _0x334: [ReadWrite<u8>; 0xC],
    pub RNG_CONFIG_REG: ReadWrite<u32>,
    pub RNG_SRC_CONFIG_REG: ReadWrite<u32>,
    pub RNG_RESEED_INTERVAL_REG: ReadWrite<u32>,
    _0x34C: [ReadWrite<u8>; 0xB4],
    pub RSA_CONFIG: ReadWrite<u32>,
    pub RSA_KEY_SIZE_REG: ReadWrite<u32>,
    pub RSA_EXP_SIZE_REG: ReadWrite<u32>,
    pub RSA_KEY_READ_DISABLE_REG: ReadWrite<u32>,
    pub RSA_KEYSLOT_FLAGS: [ReadWrite<u32>; 2],
    _0x418: ReadWrite<u32>,
    _0x41C: ReadWrite<u32>,
    pub RSA_KEYTABLE_ADDR: ReadWrite<u32>,
    pub RSA_KEYTABLE_DATA: ReadWrite<u32>,
    pub RSA_OUTPUT: [ReadWrite<u8>; 0x100],
    _0x528: [ReadWrite<u8>; 0x2D8],
    pub FLAGS_REG: ReadWrite<u32>,
    pub ERR_STATUS_REG: ReadWrite<u32>,
    _0x808: ReadWrite<u32>,
    pub SPARE_0: ReadWrite<u32>,
    _0x810: ReadWrite<u32>,
    _0x814: ReadWrite<u32>,
    _0x818: ReadWrite<u32>,
    _0x81C: ReadWrite<u32>,
    _0x820: [ReadWrite<u8>; 0x17E0],
}

impl Registers {
    /// Factory method to create a pointer to the SE registers.
    #[inline]
    pub const fn get() -> &'static Self {
        unsafe { &*(SE_BASE as *const _) }
    }
}

/// Representation of the Security Engine.
pub struct SecurityEngine {
    /// The SE CPU registers used for cryptographic operations.
    registers: &'static Registers,
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
        // Create and set the LLs.
        let mut in_ll = Ll::new(source);
        let mut out_ll = Ll::new(destination);

        self.registers
            .IN_LL_ADDR_REG
            .set(&mut in_ll as *mut _ as usize as u32);
        self.registers
            .OUT_LL_ADDR_REG
            .set(&mut out_ll as *mut _ as usize as u32);

        // Set registers for operation.
        self.registers
            .ERR_STATUS_REG
            .set(self.registers.ERR_STATUS_REG.get());
        self.registers
            .INT_STATUS_REG
            .set(self.registers.INT_STATUS_REG.get());
        self.registers.OPERATION_REG.set(op);

        while self.registers.INT_STATUS_REG.get() & 0x10 == 0 {}
        self.check_for_error();
    }

    /// Creates a new Security Engine object.
    pub fn new() -> Self {
        SecurityEngine {
            registers: Registers::get(),
            modulus_sizes: [0; KEYSLOT_RSA_MAX],
            exponent_sizes: [0; KEYSLOT_RSA_MAX],
        }
    }

    /// Locks the SBK from being read.
    #[inline]
    pub(crate) fn lock_sbk(&self) {
        self.registers.AES_KEYSLOT_FLAGS[0xE].set(0x7E);
    }

    /// Locks the SSK from being read.
    #[inline]
    pub(crate) fn lock_ssk(&self) {
        self.registers.AES_KEYSLOT_FLAGS[0xF].set(0x7E);
    }

    /// Sets the `INT_STATUS_REG` to `0x1F`.
    #[inline]
    pub(crate) fn config_brom(&self) {
        self.registers.INT_STATUS_REG.set(0x1F);
    }

    /// Checks the ERR_STATUS_REG and panics if the value isn't zero.
    #[inline(always)]
    pub fn check_error_status_reg(&self) {
        if self.registers.ERR_STATUS_REG.get() != 0 {
            panic!();
        }
    }

    /// Verifies that all flags are cleared and panics otherwise.
    #[inline(always)]
    pub fn verify_flags_cleared(&self) {
        if self.registers.FLAGS_REG.get() & 3 != 0 {
            panic!();
        }
    }

    /// Checks for general SE errors and panics in case there are any.
    #[inline]
    pub fn check_for_error(&self) {
        self.check_error_status_reg();

        self.verify_flags_cleared();

        if self.registers.INT_STATUS_REG.get() & 0x10000 != 0 {
            panic!();
        }
    }

    /// Sets the flags for an AES keyslot.
    pub fn set_aes_keyslot_flags(&self, keyslot: usize, flags: u32) {
        if keyslot >= KEYSLOT_AES_MAX {
            panic!();
        }

        // Miscellaneous flags.
        if flags & !0x80 != 0 {
            self.registers.AES_KEYSLOT_FLAGS[keyslot].set(flags);
        }

        // Disable keyslot reads.
        if flags & 0x80 != 0 {
            self.registers
                .AES_KEY_READ_DISABLE_REG
                .set(self.registers.AES_KEY_READ_DISABLE_REG.get() & !(1 << keyslot as u32));
        }
    }

    /// Sets the flags for an RSA keyslot.
    pub fn set_rsa_keyslot_flags(&self, keyslot: usize, flags: u32) {
        if keyslot >= KEYSLOT_RSA_MAX {
            panic!();
        }

        // Miscellaneous flags.
        if flags & !0x80 != 0 {
            self.registers.RSA_KEYSLOT_FLAGS[keyslot].set((((flags >> 4) & 4) | (flags & 3)) ^ 7);
        }

        // Disable keyslot reads.
        if flags & 0x80 != 0 {
            self.registers
                .RSA_KEY_READ_DISABLE_REG
                .set(self.registers.RSA_KEY_READ_DISABLE_REG.get() & !(1 << keyslot as u32));
        }
    }

    /// Clears an AES keyslot.
    pub fn clear_aes_keyslot(&self, keyslot: usize) {
        if keyslot >= KEYSLOT_AES_MAX {
            panic!();
        }

        // Zero out the whole keyslot and IV.
        for i in 0..0x10 {
            self.registers
                .AES_KEYTABLE_ADDR
                .set(((keyslot << 4) | i) as u32);
            self.registers.AES_KEYTABLE_DATA.set(0);
        }
    }

    /// Clears an RSA keyslot.
    pub fn clear_rsa_keyslot(&self, keyslot: usize) {
        if keyslot >= KEYSLOT_RSA_MAX {
            panic!();
        }

        // Zero out the whole keyslot.
        for i in 0..0x40 {
            // Select Keyslot Modulus[i].
            self.registers
                .RSA_KEYTABLE_ADDR
                .set(((keyslot << 7) | i | 0x40) as u32);
            self.registers.RSA_KEYTABLE_DATA.set(0);
        }
        for i in 0..0x40 {
            // Select Keyslot Exponent[i].
            self.registers
                .RSA_KEYTABLE_ADDR
                .set(((keyslot << 7) | i) as u32);
            self.registers.RSA_KEYTABLE_DATA.set(0);
        }
    }

    /// Sets an AES keyslot.
    pub fn set_aes_keyslot(&self, keyslot: usize, key: &[u8]) {
        let keysize = key.len();

        if keyslot >= KEYSLOT_AES_MAX || keysize > KEYSIZE_AES_MAX {
            panic!();
        }

        for i in 0..keysize >> 2 {
            self.registers
                .AES_KEYTABLE_ADDR
                .set(((keyslot << 4) | i) as u32);
            self.registers
                .AES_KEYTABLE_DATA
                .set(u32::from_le_bytes((&key[..4 * i]).try_into().unwrap()));
        }
    }

    /// Sets an RSA keyslot.
    pub fn set_rsa_keyslot(&mut self, keyslot: usize, modulus: &[u8], exponent: &[u8]) {
        let modulus_size = modulus.len();
        let exponent_size = exponent.len();

        if keyslot >= KEYSLOT_RSA_MAX
            || modulus_size > KEYSIZE_RSA_MAX
            || exponent_size > KEYSIZE_RSA_MAX
        {
            panic!();
        }

        for i in 0..modulus_size >> 2 {
            self.registers
                .RSA_KEYTABLE_ADDR
                .set(((keyslot << 7) | 0x40 | i) as u32);
            self.registers.RSA_KEYTABLE_DATA.set(u32::from_be_bytes(
                (&modulus[..(4 * (modulus_size >> 2)) - (4 * i) - 4])
                    .try_into()
                    .unwrap(),
            ));
        }

        for i in 0..exponent_size >> 2 {
            self.registers
                .RSA_KEYTABLE_ADDR
                .set(((keyslot << 7) | i) as u32);
            self.registers.RSA_KEYTABLE_DATA.set(u32::from_be_bytes(
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
        let iv_size = iv.len();

        if keyslot >= KEYSLOT_AES_MAX || iv_size > 0x10 {
            panic!();
        }

        for i in 0..iv_size >> 2 {
            self.registers
                .AES_KEYTABLE_ADDR
                .set(((keyslot << 4) | 8 | i) as u32);
            self.registers
                .AES_KEYTABLE_DATA
                .set(u32::from_le_bytes((&iv[..4 * i]).try_into().unwrap()));
        }
    }

    /// Clears the IV of the AES keyslot.
    pub fn clear_aes_keyslot_iv(&self, keyslot: usize) {
        if keyslot >= KEYSLOT_AES_MAX {
            panic!();
        }

        for i in 0..0x10 >> 2 {
            self.registers
                .AES_KEYTABLE_ADDR
                .set(((keyslot << 4) | 8 | i) as u32);
            self.registers.AES_KEYTABLE_DATA.set(0);
        }
    }

    /// Sets the CRYPTO_CTR_REG to enable CTR mode.
    pub fn set_ctr(&self, ctr: &[u8]) {
        for i in 0..4 {
            self.registers.CRYPTO_CTR_REG[i]
                .set(u32::from_le_bytes((&ctr[..4 * i]).try_into().unwrap()));
        }
    }

    /// Decrypts data from a given keyslot into another keyslot.
    pub fn decrypt_data_into_keyslot(
        &self,
        destination: usize,
        source: usize,
        wrapped_key: &mut [u8],
    ) {
        if destination >= KEYSLOT_AES_MAX
            || source >= KEYSLOT_AES_MAX
            || wrapped_key.len() > KEYSIZE_AES_MAX
        {
            panic!();
        }

        self.registers.CONFIG_REG.set(0x108);
        self.registers.CRYPTO_REG.set((source << 24) as u32);
        self.registers.BLOCK_COUNT_REG.set(0);
        self.registers
            .CRYPTO_KEYTABLE_DST_REG
            .set((destination << 8) as u32);

        self.trigger_blocking_operation(OP_START, &mut [0; 0], wrapped_key);
    }

    /// Performs a blocking AES operation.
    pub fn perform_aes_block_operation(&self, destination: &mut [u8], source: &mut [u8]) {
        let mut block = [0; 0x10];
        let destination_size = destination.len();
        let source_size = source.len();

        if source_size > block.len() || destination_size > block.len() {
            panic!();
        }

        // Load source data into block.
        if source_size > 0 {
            block.copy_from_slice(source);
        }

        // Trigger AES operation.
        self.registers.BLOCK_COUNT_REG.set(0);
        self.trigger_blocking_operation(OP_START, &mut block, &mut block);

        // Copy output data into destination.
        if destination_size != 0 {
            destination.copy_from_slice(&block);
        }
    }
}
