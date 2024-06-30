//! Contains register mappings.

use super::{AccelOdr, Sensitivity};
use bitfield_struct::bitfield;

pub mod accel;
pub mod mag;

/// `CTRL_REG1_A` (20h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ControlRegister1A {
    #[bits(4, access = RW)]
    pub output_data_rate: AccelOdr,

    #[bits(1, access = RW)]
    pub low_power_enable: bool,

    #[bits(1, access = RW)]
    pub z_enable: bool,

    #[bits(1, access = RW)]
    pub y_enable: bool,

    #[bits(1, access = RW)]
    pub x_enable: bool,
}

/// `CTRL_REG4_A` (23h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ControlRegister4A {
    #[bits(1, access = RW)]
    pub block_data_update: bool,

    #[bits(1, access = RW)]
    pub big_little_endian: bool,

    #[bits(2, access = RW)]
    pub full_scale: Sensitivity,

    #[bits(1, access = RW)]
    pub high_resolution: bool,

    #[bits(2)]
    __: u8,

    #[bits(1, access = RW)]
    pub spi_serial_interface_mode: bool,
}

/// `STATUS_REG_A` (27h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct StatusRegisterA {
    /// X-, Y-, and Z-axis data overrun.
    /// * `false` - no overrun has occurred
    /// * `true` - a new set of data has overwritten the previous data
    #[bits(1, access = RO)]
    pub zyx_overrun: bool,

    /// Z-axis data overrun.
    /// * `false` - no overrun has occurred
    /// * `true` - new data for the Z-axis has overwritten the previous data
    #[bits(1, access = RO)]
    pub z_overrun: bool,

    /// Y-axis data overrun.
    /// * `false` - no overrun has occurred
    /// * `true` - new data for the Y-axis has overwritten the previous data
    #[bits(1, access = RO)]
    pub y_overrun: bool,

    /// X-axis data overrun.
    /// * `false` - no overrun has occurred
    /// * `true` - new data for the X-axis has overwritten the previous data
    #[bits(1, access = RO)]
    pub x_overrun: bool,

    ///  X-, Y-, and Z-axis new data available.
    /// * `false` - a new set of data is not yet available
    /// * `true` - a new set of data is available
    #[bits(1, access = RO)]
    pub xyz_data_available: bool,

    /// Z-axis new data available.
    /// * `false` - new data for the Z-axis is not yet available
    /// * `true` - new data for the Z-axis is available
    #[bits(1, access = RO)]
    pub z_data_available: bool,

    /// Y-axis new data available.
    /// * `false` - new data for the Y-axis is not yet available
    /// * `true` - new data for the Y-axis is available
    #[bits(1, access = RO)]
    pub y_data_available: bool,

    /// X-axis new data available.
    /// * `false` - new data for the X-axis is not yet available
    /// * `true` - new data for the X-axis is available
    #[bits(1, access = RO)]
    pub x_data_available: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_register_a() {
        let reg = StatusRegisterA::from(0b1001_0010);
        assert!(reg.zyx_overrun());
        assert!(!reg.z_overrun());
        assert!(!reg.y_overrun());
        assert!(reg.x_overrun());
        assert!(!reg.xyz_data_available());
        assert!(!reg.z_data_available());
        assert!(reg.y_data_available());
        assert!(!reg.x_data_available());
    }
}
