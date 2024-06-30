pub use bitfield_struct::bitfield;
use Sensitivity;

pub const ADDRESS: u8 = 0b0011001;

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum Register {
    CTRL_REG1_A = 0x20,
    CTRL_REG2_A = 0x21,
    CTRL_REG3_A = 0x22,
    CTRL_REG4_A = 0x23,
    CTRL_REG5_A = 0x24,
    CTRL_REG6_A = 0x25,
    REFERENCE_A = 0x26,
    STATUS_REG_A = 0x27,
    OUT_X_L_A = 0x28,
    OUT_X_H_A = 0x29,
    OUT_Y_L_A = 0x2A,
    OUT_Y_H_A = 0x2B,
    OUT_Z_L_A = 0x2C,
    OUT_Z_H_A = 0x2D,
    FIFO_CTRL_REG_A = 0x2E,
    FIFO_SRC_REG_A = 0x2F,
    INT1_CFG_A = 0x30,
    INT1_SRC_A = 0x31,
    INT1_THS_A = 0x32,
    INT1_DURATION_A = 0x33,
    INT2_CFG_A = 0x34,
    INT2_SRC_A = 0x35,
    INT2_THS_A = 0x36,
    INT2_DURATION_A = 0x37,
    CLICK_CFG_A = 0x38,
    CLICK_SRC_A = 0x39,
    CLICK_THS_A = 0x3A,
    TIME_LIMIT_A = 0x3B,
    TIME_LATENCY_A = 0x3C,
    TIME_WINDOW_A = 0x3D,
}

impl Register {
    pub fn addr(&self) -> u8 {
        *self as u8
    }
}

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
