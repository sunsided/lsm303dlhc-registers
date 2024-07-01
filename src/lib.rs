//! # STMicroelectronics LSM303DLHC E-Compass Registers
//!
//! This crate provides a typed map of the LSM303DLHC's I²C registers.

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]
// Enables the `doc_cfg` feature when the `docsrs` configuration attribute is defined.
#![cfg_attr(docsrs, feature(doc_cfg))]

mod accel;
mod mag;

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
