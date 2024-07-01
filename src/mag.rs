//! Magnetometer and Temperature registers.

mod types;

use crate::{Register, WritableRegister};
use bitfield_struct::bitfield;
pub use types::*;

/// The I2C bus address.
///
/// For magnetic sensors the default (factory) 7-bit slave address is 0011110xb.
///
/// The slave address is completed with a Read/Write bit. If the bit is `1` (read), a repeated
/// `START` (`SR`) condition must be issued after the two sub-address bytes. If the bit is `0` (write)
/// the master transmits to the slave with the direction unchanged.
pub const DEFAULT_DEVICE_ADDRESS: u8 = 0b0011110;

/// Register addresses specific to the magnetometer sensor.
///
/// See also [`DEFAULT_DEVICE_ADDRESS`].
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegisterAddress {
    /// See [`CraRegisterM`].
    CRA_REG_M = 0x00,
    /// See [`CrbRegisterM`].
    CRB_REG_M = 0x01,
    /// See [`ModeRegisterM`].
    MR_REG_M = 0x02,
    OUT_X_H_M = 0x03,
    OUT_X_L_M = 0x04,
    OUT_Z_H_M = 0x05,
    OUT_Z_L_M = 0x06,
    OUT_Y_H_M = 0x07,
    OUT_Y_L_M = 0x08,
    /// See [`StatusRegisterM`].
    SR_REG_M = 0x09,
    /// See [`IRARegisterM`].
    IRA_REG_M = 0x0A,
    /// See [`IRBRegisterM`].
    IRB_REG_M = 0x0B,
    /// See [`IRCRegisterM`].
    IRC_REG_M = 0x0C,
    TEMP_OUT_H_M = 0x31,
    TEMP_OUT_L_M = 0x32,
}

impl RegisterAddress {
    /// Returns the address of a register.
    pub const fn addr(&self) -> u8 {
        *self as u8
    }
}

/// [`CRA_REG_M`](RegisterAddress::CRA_REG_M) (00h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct CraRegisterM {
    /// Temperature sensor enabled.
    #[bits(1, access = RW)]
    pub temp_en: bool,

    /// Must be zero for correct operation of the device.
    #[bits(2, default = 0)]
    zeros_56: u8,

    /// Data output rate bits. These bits set the rate at which data is written to all three data
    /// output registers.
    #[bits(3, access = RW, default = MagOdr::Hz75)]
    pub data_output_rate: MagOdr,

    /// Must be zero for correct operation of the device.
    #[bits(2, default = 0)]
    zeros_01: u8,
}

impl Register for CraRegisterM {
    const DEV_ADDRESS: u8 = DEFAULT_DEVICE_ADDRESS;
    const REG_ADDRESS: u8 = RegisterAddress::CRA_REG_M.addr();

    fn from_bits(bits: u8) -> Self {
        Self::from_bits(bits)
    }

    fn to_bits(&self) -> u8 {
        self.into_bits()
    }
}

impl WritableRegister for CraRegisterM {}

/// Magnetometer gain configuration.
///
/// [`CRB_REG_M`](RegisterAddress::CRB_REG_M) (01h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct CrbRegisterM {
    /// Gain configuration.
    #[bits(3, access = RW)]
    pub gain: MagGain,

    /// Must be zero for correct operation of the device.
    #[bits(5, default = 0)]
    zeros_04: u8,
}

impl Register for CrbRegisterM {
    const DEV_ADDRESS: u8 = DEFAULT_DEVICE_ADDRESS;
    const REG_ADDRESS: u8 = RegisterAddress::CRB_REG_M.addr();

    fn from_bits(bits: u8) -> Self {
        Self::from_bits(bits)
    }

    fn to_bits(&self) -> u8 {
        self.into_bits()
    }
}

impl WritableRegister for CrbRegisterM {}

/// Magnetometer mode select.
///
/// [`MR_REG_M`](RegisterAddress::MR_REG_M) (09h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ModeRegisterM {
    /// Must be zero for correct operation of the device.
    #[bits(6, default = 0)]
    zeros_27: u8,

    /// Device is placed in sleep mode.
    #[bits(1, access = RW)]
    pub sleep_mode: bool,

    /// Enables single conversion mode.
    ///
    /// * `false` - Continuous conversion mode.
    /// * `true` - Single conversion mode.
    #[bits(1, access = RW, default = false)]
    pub single_conversion: bool,
}

impl Register for ModeRegisterM {
    const DEV_ADDRESS: u8 = DEFAULT_DEVICE_ADDRESS;
    const REG_ADDRESS: u8 = RegisterAddress::MR_REG_M.addr();

    fn from_bits(bits: u8) -> Self {
        Self::from_bits(bits)
    }

    fn to_bits(&self) -> u8 {
        self.into_bits()
    }
}

impl WritableRegister for ModeRegisterM {}

/// [`SR_REG_M`](RegisterAddress::SR_REG_M) (09h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct StatusRegisterM {
    #[bits(6)]
    __: u8,

    /// Data output register lock. Once a new set of measurements is available, this bit is
    /// set when the first magnetic file data register has been read.
    #[bits(1, access = RO)]
    pub do_lock: bool,

    /// Data-ready bit. This bit is when a new set of measurements is available.
    #[bits(1, access = RO)]
    pub data_ready: bool,
}

impl Register for StatusRegisterM {
    const DEV_ADDRESS: u8 = DEFAULT_DEVICE_ADDRESS;
    const REG_ADDRESS: u8 = RegisterAddress::SR_REG_M.addr();

    fn from_bits(bits: u8) -> Self {
        Self::from_bits(bits)
    }

    fn to_bits(&self) -> u8 {
        self.into_bits()
    }
}

/// [`IRA_REG_M`](RegisterAddress::IRA_REG_M) (0Ah)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct IRARegisterM {
    #[bits(8, access = RO)]
    pub value: u8,
}

impl Register for IRARegisterM {
    const DEV_ADDRESS: u8 = DEFAULT_DEVICE_ADDRESS;
    const REG_ADDRESS: u8 = RegisterAddress::IRA_REG_M.addr();

    fn from_bits(bits: u8) -> Self {
        Self::from_bits(bits)
    }

    fn to_bits(&self) -> u8 {
        self.into_bits()
    }
}

/// [`IRB_REG_M`](RegisterAddress::IRB_REG_M) (0Bh)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct IRBRegisterM {
    #[bits(8, access = RO)]
    pub value: u8,
}

impl Register for IRBRegisterM {
    const DEV_ADDRESS: u8 = DEFAULT_DEVICE_ADDRESS;
    const REG_ADDRESS: u8 = RegisterAddress::IRB_REG_M.addr();

    fn from_bits(bits: u8) -> Self {
        Self::from_bits(bits)
    }

    fn to_bits(&self) -> u8 {
        self.into_bits()
    }
}

/// [`IRC_REG_M`](RegisterAddress::IRC_REG_M) (0Ch)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct IRCRegisterM {
    #[bits(8, access = RO)]
    pub value: u8,
}

impl Register for IRCRegisterM {
    const DEV_ADDRESS: u8 = DEFAULT_DEVICE_ADDRESS;
    const REG_ADDRESS: u8 = RegisterAddress::IRC_REG_M.addr();

    fn from_bits(bits: u8) -> Self {
        Self::from_bits(bits)
    }

    fn to_bits(&self) -> u8 {
        self.into_bits()
    }
}
