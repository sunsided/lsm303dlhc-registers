//! Accelerometer specific register addresses.

/// The I2C bus address.
pub const ADDRESS: u8 = 0b0011001;

/// Accelerometer specific register addresses.
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Register {
    /// See [`ControlRegister1A`].
    CTRL_REG1_A = 0x20,
    /// See [`ControlRegister2A`].
    CTRL_REG2_A = 0x21,
    /// See [`ControlRegister3A`].
    CTRL_REG3_A = 0x22,
    /// See [`ControlRegister4A`].
    CTRL_REG4_A = 0x23,
    /// See [`ControlRegister5A`].
    CTRL_REG5_A = 0x24,
    /// See [`ControlRegister6A`].
    CTRL_REG6_A = 0x25,
    REFERENCE_A = 0x26,
    /// See [`StatusRegisterA`].
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
    /// Returns the address of a register.
    pub fn addr(&self) -> u8 {
        *self as u8
    }
}
