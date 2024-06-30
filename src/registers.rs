//! Contains register mappings.

use bitfield_struct::bitfield;
use Sensitivity;

/// `CTRL_REG4_A` (23h)
#[bitfield(u8, order = Msb)]
pub struct ControlRegister4A {
    #[bits(1, access = RW)]
    block_data_update: bool,

    #[bits(1, access = RW)]
    big_little_endian: bool,

    #[bits(2, access = RW)]
    full_scale: Sensitivity,

    #[bits(1, access = RW)]
    high_resolution: bool,

    #[bits(2)]
    __: u8,

    #[bits(1, access = RW)]
    spi_serial_interface_mode: bool,
}

/// `STATUS_REG_A` (27h)
#[bitfield(u8, order = Msb)]
pub struct StatusRegisterA {
    /// X-, Y-, and Z-axis data overrun.
    /// * `false` - no overrun has occurred
    /// * `true` - a new set of data has overwritten the previous data
    #[bits(1, access = RO)]
    zyx_overrun: bool,

    /// Z-axis data overrun.
    /// * `false` - no overrun has occurred
    /// * `true` - new data for the Z-axis has overwritten the previous data
    #[bits(1, access = RO)]
    z_overrun: bool,

    /// Y-axis data overrun.
    /// * `false` - no overrun has occurred
    /// * `true` - new data for the Y-axis has overwritten the previous data
    #[bits(1, access = RO)]
    y_overrun: bool,

    /// X-axis data overrun.
    /// * `false` - no overrun has occurred
    /// * `true` - new data for the X-axis has overwritten the previous data
    #[bits(1, access = RO)]
    x_overrun: bool,

    ///  X-, Y-, and Z-axis new data available.
    /// * `false` - a new set of data is not yet available
    /// * `true` - a new set of data is available
    #[bits(1, access = RO)]
    xyz_data_available: bool,

    /// Z-axis new data available.
    /// * `false` - new data for the Z-axis is not yet available
    /// * `true` - new data for the Z-axis is available
    #[bits(1, access = RO)]
    z_data_available: bool,

    /// Y-axis new data available.
    /// * `false` - new data for the Y-axis is not yet available
    /// * `true` - new data for the Y-axis is available
    #[bits(1, access = RO)]
    y_data_available: bool,

    /// X-axis new data available.
    /// * `false` - new data for the X-axis is not yet available
    /// * `true` - new data for the X-axis is available
    #[bits(1, access = RO)]
    x_data_available: bool,
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
