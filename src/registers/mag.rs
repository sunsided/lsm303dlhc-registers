//! Magnetometer specific register addresses.

/// The I2C bus address.
///
/// For magnetic sensors the default (factory) 7-bit slave address is 0011110xb.
///
/// The slave address is completed with a Read/Write bit. If the bit is `1` (read), a repeated
/// `START` (`SR`) condition must be issued after the two sub-address bytes. If the bit is `0` (write)
/// the master transmits to the slave with the direction unchanged.
pub const ADDRESS: u8 = 0b0011110;

// Magnetometer specific register addresses.
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MagnetometerRegister {
    /// See [`CraRegisterM`](super::CraRegisterM).
    CRA_REG_M = 0x00,
    /// See [`CrbRegisterM`](super::CrbRegisterM).
    CRB_REG_M = 0x01,
    /// See [`ModeRegisterM`](super::ModeRegisterM).
    MR_REG_M = 0x02,
    OUT_X_H_M = 0x03,
    OUT_X_L_M = 0x04,
    OUT_Z_H_M = 0x05,
    OUT_Z_L_M = 0x06,
    OUT_Y_H_M = 0x07,
    OUT_Y_L_M = 0x08,
    /// See [`StatusRegisterM`](super::StatusRegisterM).
    SR_REG_M = 0x09,
    /// See [`IRARegisterM`](super::IRARegisterM).
    IRA_REG_M = 0x0A,
    /// See [`IRBRegisterM`](super::IRBRegisterM).
    IRB_REG_M = 0x0B,
    /// See [`IRCRegisterM`](super::IRCRegisterM).
    IRC_REG_M = 0x0C,
    TEMP_OUT_H_M = 0x31,
    TEMP_OUT_L_M = 0x32,
}

impl MagnetometerRegister {
    /// Returns the address of a register.
    pub const fn addr(&self) -> u8 {
        *self as u8
    }
}
