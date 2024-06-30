/// A sensor register.
pub trait Register {
    /// The slave device address.
    const DEV_ADDRESS: u8;

    /// Gets the address of the register.
    const REG_ADDRESS: u8;

    /// Creates a new register instance from bit values.
    fn from_bits(bits: u8) -> Self;

    /// Converts the register instance into bit values.
    fn to_bits(&self) -> u8;
}

/// A register that can be written to.
pub trait WritableRegister: Register {}
