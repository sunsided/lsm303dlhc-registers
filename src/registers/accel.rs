//! Accelerometer specific register addresses.

/// The I2C bus address.
///
/// For linear acceleration the default (factory) 7-bit slave address is `0011001b`.
///
/// The slave address is completed with a Read/Write bit. If the bit is `1` (read), a repeated
/// `START` (`SR`) condition must be issued after the two sub-address bytes; if the bit is `0` (write)
/// the master transmits to the slave with the direction unchanged.
///
/// When the MSB is set to `1`, multiple bytes can be read.
pub const ADDRESS: u8 = 0b0011001;

/// Accelerometer specific register addresses.
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AccelerometerRegister {
    /// See [`ControlRegister1A`](super::ControlRegister1A).
    CTRL_REG1_A = 0x20,
    /// See [`ControlRegister2A`](super::ControlRegister2A).
    CTRL_REG2_A = 0x21,
    /// See [`ControlRegister3A`](super::ControlRegister3A).
    CTRL_REG3_A = 0x22,
    /// See [`ControlRegister4A`](super::ControlRegister4A).
    CTRL_REG4_A = 0x23,
    /// See [`ControlRegister5A`](super::ControlRegister5A).
    CTRL_REG5_A = 0x24,
    /// See [`ControlRegister6A`](super::ControlRegister6A).
    CTRL_REG6_A = 0x25,
    /// See [`ReferenceRegisterA`](super::ReferenceRegisterA).
    REFERENCE_A = 0x26,
    /// See [`StatusRegisterA`](super::StatusRegisterA).
    STATUS_REG_A = 0x27,
    OUT_X_L_A = 0x28,
    OUT_X_H_A = 0x29,
    OUT_Y_L_A = 0x2A,
    OUT_Y_H_A = 0x2B,
    OUT_Z_L_A = 0x2C,
    OUT_Z_H_A = 0x2D,
    /// See [`FifoControlRegisterA`](super::FifoControlRegisterA).
    FIFO_CTRL_REG_A = 0x2E,
    /// See [`FifoSourceRegisterA`](super::FifoSourceRegisterA).
    FIFO_SRC_REG_A = 0x2F,
    /// See [`Int1ConfigurationRegisterA`](super::Int1ConfigurationRegisterA).
    INT1_CFG_A = 0x30,
    /// See [`Int1SourceRegisterA`](super::Int1SourceRegisterA).
    INT1_SRC_A = 0x31,
    /// See [`Int1ThresholdRegisterA`](super::Int1ThresholdRegisterA).
    INT1_THS_A = 0x32,
    /// See [`Int1DurationRegisterA`](super::Int1DurationRegisterA).
    INT1_DURATION_A = 0x33,
    /// See [`Int1ConfigurationRegisterA`](super::Int1ConfigurationRegisterA).
    INT2_CFG_A = 0x34,
    /// See [`Int2SourceRegisterA`](super::Int2SourceRegisterA).
    INT2_SRC_A = 0x35,
    /// See [`Int2ThresholdRegisterA`](super::Int2ThresholdRegisterA).
    INT2_THS_A = 0x36,
    /// See [`Int2DurationRegisterA`](super::Int2DurationRegisterA).
    INT2_DURATION_A = 0x37,
    /// See [`ClickConfigurationRegisterA`](super::ClickConfigurationRegisterA).
    CLICK_CFG_A = 0x38,
    /// See [`ClickSourceRegisterA`](super::ClickSourceRegisterA).
    CLICK_SRC_A = 0x39,
    /// See [`ClickThresholdRegisterA`](super::ClickThresholdRegisterA).
    CLICK_THS_A = 0x3A,
    /// See [`TimeLimitRegisterA`](super::ClickTimeLimitRegisterA).
    TIME_LIMIT_A = 0x3B,
    /// See [`ClickTimeLatencyRegisterA`](super::ClickTimeLatencyRegisterA).
    TIME_LATENCY_A = 0x3C,
    /// See [`ClickTimeWindowRegisterA`](super::ClickTimeWindowRegisterA).
    TIME_WINDOW_A = 0x3D,
}

impl AccelerometerRegister {
    /// Returns the address of a register.
    pub const fn addr(&self) -> u8 {
        *self as u8
    }
}
