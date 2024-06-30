//! Contains register mappings.

use super::{AccelOdr, Sensitivity};
use bitfield_struct::bitfield;
use MagOdr;

pub mod accel;
pub mod mag;

/// [`CTRL_REG1_A`](accel::Register::CTRL_REG1_A) (20h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ControlRegister1A {
    /// Data rate selection.
    #[bits(4, access = RW)]
    pub output_data_rate: AccelOdr,

    /// Low-power mode enable.
    #[bits(1, access = RW)]
    pub low_power_enable: bool,

    /// Z-axis enable.
    #[bits(1, access = RW, default = true)]
    pub z_enable: bool,

    /// Y-axis enable.
    #[bits(1, access = RW, default = true)]
    pub y_enable: bool,

    /// X-axis enable.
    #[bits(1, access = RW, default = true)]
    pub x_enable: bool,
}

/// [`CTRL_REG2_A`](accel::Register::CTRL_REG2_A) (21h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ControlRegister2A {
    /// High-pass filter mode selection.
    #[bits(2, access = RW)]
    pub hpm: u8, // TODO: Add enum

    /// High-pass filter Cutoff frequency selection
    #[bits(2, access = RW)]
    pub hpcf: u8, // TODO: Add enum

    /// Filter data selection
    #[bits(1, access = RW)]
    pub fds: bool,

    /// High-pass filter enabled for click function
    #[bits(1, access = RW)]
    pub hpclick: bool,

    /// High-pass filter enabled for AOI function on Interrupt 2
    #[bits(1, access = RW)]
    pub hpis2: bool,

    /// High-pass filter enabled for AOI function on Interrupt 1
    #[bits(1, access = RW)]
    pub hpis1: bool,
}

/// [`CTRL_REG3_A`](accel::Register::CTRL_REG3_A) (22h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ControlRegister3A {
    /// Enable CLICK interrupt on INT1
    #[bits(1, access = RW)]
    pub i1click: bool,

    /// Enable AOI1 interrupt on INT1
    #[bits(1, access = RW)]
    pub i1aoi1: bool,

    /// Enable AOI2 interrupt on INT1
    #[bits(1, access = RW)]
    pub i1aoi2: bool,

    /// Enable DRDY1 interrupt on INT1
    #[bits(1, access = RW)]
    pub i1drdy1: bool,

    /// Enable DRDY2 interrupt on INT1
    #[bits(1, access = RW)]
    pub i1dry2: bool,

    /// Enable FIFO watermark interrupt on INT1
    #[bits(1, access = RW)]
    pub i1wtm: bool,

    /// Enable FIFO overrun interrupt on INT1
    #[bits(1, access = RW)]
    pub i1overrun: bool,

    #[bits(1)]
    __: bool,
}

/// [`CTRL_REG4_A`](accel::Register::CTRL_REG4_A) (23h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ControlRegister4A {
    /// Block data update.
    ///
    /// * `false` - continuous update
    /// * `true` - output registers not updated until MSB and LSB
    // have been read
    #[bits(1, access = RW)]
    pub block_data_update: bool,

    /// Big/little endian data selection.
    ///
    /// * `false` - data LSB @ lower address
    /// * `true` - data MSB @ lower address
    #[bits(1, access = RW)]
    pub big_endian: bool,

    /// Full-scale selection
    #[bits(2, access = RW)]
    pub full_scale: Sensitivity,

    /// High-resolution output mode.
    #[bits(1, access = RW)]
    pub high_resolution: bool,

    #[bits(2)]
    __: u8,

    /// SPI serial interface mode.
    ///
    /// * `false` - 4-wire interface
    /// * `true` - 3-wire interface
    #[bits(1, access = RW)]
    pub spi_serial_3wire: bool,
}

/// [`CTRL_REG5_A`](accel::Register::CTRL_REG5_A) (24h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ControlRegister5A {
    /// Reboot memory content
    // have been read
    #[bits(1, access = RW)]
    pub boot: bool,

    /// Enable FIFO
    // have been read
    #[bits(1, access = RW)]
    pub fifo_enable: bool,

    #[bits(2)]
    __: u8,

    /// Latch interrupt request on INT1_SRC register, with INT1_SRC register cleared
    /// by reading INT1_SRC itself.
    // have been read
    #[bits(1, access = RW)]
    pub lir_int1: bool,

    /// 4D enable: 4D detection is enabled on INT1 when 6D bit on INT1_CFG is set to 1.
    // have been read
    #[bits(1, access = RW)]
    pub d4d_int1: bool,

    /// Latch interrupt request on INT2_SRC register, with INT2_SRC register cleared
    /// by reading INT2_SRC itself.
    // have been read
    #[bits(1, access = RW)]
    pub lir_int2: bool,

    /// 4D enable: 4D detection is enabled on INT2 when 6D bit on INT2_CFG is set to 1.
    // have been read
    #[bits(1, access = RW)]
    pub d4d_int2: bool,
}

/// [`CTRL_REG6_A`](accel::Register::CTRL_REG6_A) (25h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ControlRegister6A {
    /// CLICK interrupt enable on PAD2.
    // have been read
    #[bits(1, access = RW)]
    pub i2click_en: bool,

    /// Interrupt 1 on PAD2
    // have been read
    #[bits(1, access = RW)]
    pub i2int1: bool,

    /// Interrupt 2 on PAD2
    // have been read
    #[bits(1, access = RW)]
    pub i2int2: bool,

    /// Reboot memory content on PAD2
    // have been read
    #[bits(1, access = RW)]
    pub boot_i1: bool,

    /// Active functions status on PAD2
    // have been read
    #[bits(1, access = RW)]
    pub p2_active: bool,

    #[bits(1)]
    __: u8,

    /// Interrupt active low
    ///
    /// * `false` - Interrupt is active high
    /// * `true` - Interrupt is active low
    // have been read
    #[bits(1, access = RW)]
    pub active_low: bool,

    #[bits(1)]
    __: u8,
}

/// [`STATUS_REG_A`](accel::Register::STATUS_REG_A)(27h)
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

/// [`CRA_REG_M`](mag::Register::CRA_REG_M) (09h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct CraRegisterM {
    /// Temperature sensor enabled.
    #[bits(1, access = RO)]
    pub temp_en: bool,

    /// Must be zero for correct operation of the device.
    #[bits(2, default = 0)]
    zeros_56: u8,

    /// Data output rate bits. These bits set the rate at which data is written to all three data
    /// output registers.
    #[bits(3, access = RO, default = MagOdr::Hz75)]
    pub data_output_rate: MagOdr,

    /// Must be zero for correct operation of the device.
    #[bits(2, default = 0)]
    zeros_01: u8,
}

/// [`SR_REG_M`](mag::Register::SR_REG_M) (09h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct StatusRegisterM {
    #[bits(6)]
    __: u8,

    /// Data output register lock. Once a new set of measurements is available, this bit is
    /// set when the first magnetic file data register has been read.
    #[bits(1, access = RO)]
    pub lock: bool,

    /// Data-ready bit. This bit is when a new set of measurements is available.
    #[bits(1, access = RO)]
    pub data_ready: bool,
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
