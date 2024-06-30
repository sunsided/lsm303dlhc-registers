//! Contains register mappings.

use super::{AccelOdr, Sensitivity};
use bitfield_struct::bitfield;
use registers::accel::AccelerometerRegister;
use registers::mag::MagnetometerRegister;
pub use registers::register::{Register, WritableRegister};
use {FifoMode, MagOdr};
use {HighpassFilterMode, MagGain};

pub mod accel;
pub mod mag;
mod register;

/// [`CTRL_REG1_A`](accel::AccelerometerRegister::CTRL_REG1_A) (20h)
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

impl Register for ControlRegister1A {
    const DEV_ADDRESS: u8 = accel::ADDRESS;
    const REG_ADDRESS: u8 = AccelerometerRegister::CTRL_REG1_A.addr();

    fn from_bits(bits: u8) -> Self {
        Self::from_bits(bits)
    }

    fn to_bits(&self) -> u8 {
        self.into_bits()
    }
}

impl WritableRegister for ControlRegister1A {}

/// [`CTRL_REG2_A`](accel::AccelerometerRegister::CTRL_REG2_A) (21h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ControlRegister2A {
    /// High-pass filter mode selection.
    #[bits(2, access = RW)]
    pub hpm: HighpassFilterMode,

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

impl Register for ControlRegister2A {
    const DEV_ADDRESS: u8 = accel::ADDRESS;
    const REG_ADDRESS: u8 = AccelerometerRegister::CTRL_REG2_A.addr();

    fn from_bits(bits: u8) -> Self {
        Self::from_bits(bits)
    }

    fn to_bits(&self) -> u8 {
        self.into_bits()
    }
}

impl WritableRegister for ControlRegister2A {}

/// [`CTRL_REG3_A`](accel::AccelerometerRegister::CTRL_REG3_A) (22h)
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

impl Register for ControlRegister3A {
    const DEV_ADDRESS: u8 = accel::ADDRESS;
    const REG_ADDRESS: u8 = AccelerometerRegister::CTRL_REG3_A.addr();

    fn from_bits(bits: u8) -> Self {
        Self::from_bits(bits)
    }

    fn to_bits(&self) -> u8 {
        self.into_bits()
    }
}

impl WritableRegister for ControlRegister3A {}

/// [`CTRL_REG4_A`](accel::AccelerometerRegister::CTRL_REG4_A) (23h)
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

    #[bits(2, default = 0b00)]
    zeros_12: u8,

    /// SPI serial interface mode.
    ///
    /// * `false` - 4-wire interface
    /// * `true` - 3-wire interface
    #[bits(1, access = RW)]
    pub spi_serial_3wire: bool,
}

impl Register for ControlRegister4A {
    const DEV_ADDRESS: u8 = accel::ADDRESS;
    const REG_ADDRESS: u8 = AccelerometerRegister::CTRL_REG4_A.addr();

    fn from_bits(bits: u8) -> Self {
        Self::from_bits(bits)
    }

    fn to_bits(&self) -> u8 {
        self.into_bits()
    }
}

impl WritableRegister for ControlRegister4A {}

/// [`CTRL_REG5_A`](accel::AccelerometerRegister::CTRL_REG5_A) (24h)
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

impl Register for ControlRegister5A {
    const DEV_ADDRESS: u8 = accel::ADDRESS;
    const REG_ADDRESS: u8 = AccelerometerRegister::CTRL_REG5_A.addr();

    fn from_bits(bits: u8) -> Self {
        Self::from_bits(bits)
    }

    fn to_bits(&self) -> u8 {
        self.into_bits()
    }
}

impl WritableRegister for ControlRegister5A {}

/// [`CTRL_REG6_A`](accel::AccelerometerRegister::CTRL_REG6_A) (25h)
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

impl Register for ControlRegister6A {
    const DEV_ADDRESS: u8 = accel::ADDRESS;
    const REG_ADDRESS: u8 = AccelerometerRegister::CTRL_REG6_A.addr();

    fn from_bits(bits: u8) -> Self {
        Self::from_bits(bits)
    }

    fn to_bits(&self) -> u8 {
        self.into_bits()
    }
}

impl WritableRegister for ControlRegister6A {}

/// [`REFERENCE_A`](accel::AccelerometerRegister::REFERENCE_A)(27h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ReferenceRegisterA {
    /// Reference value for interrupt generation.
    #[bits(8, access = RO)]
    pub reference: u8,
}

impl Register for ReferenceRegisterA {
    const DEV_ADDRESS: u8 = accel::ADDRESS;
    const REG_ADDRESS: u8 = AccelerometerRegister::REFERENCE_A.addr();

    fn from_bits(bits: u8) -> Self {
        Self::from_bits(bits)
    }

    fn to_bits(&self) -> u8 {
        self.into_bits()
    }
}

impl WritableRegister for ReferenceRegisterA {}

/// [`STATUS_REG_A`](accel::AccelerometerRegister::STATUS_REG_A) (27h)
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

impl Register for StatusRegisterA {
    const DEV_ADDRESS: u8 = accel::ADDRESS;
    const REG_ADDRESS: u8 = AccelerometerRegister::STATUS_REG_A.addr();

    fn from_bits(bits: u8) -> Self {
        Self::from_bits(bits)
    }

    fn to_bits(&self) -> u8 {
        self.into_bits()
    }
}

/// [`FIFO_CTRL_REG_A`](accel::AccelerometerRegister::FIFO_CTRL_REG_A) (2Eh)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct FifoControlRegisterA {
    /// FIFO mode selection
    #[bits(2, access = RO, default = FifoMode::Bypass)]
    pub fifo_mode: FifoMode,

    /// Trigger selection
    ///
    /// * `false` - Trigger event linked to trigger signal on INT1
    /// * `true` - Trigger event linked to trigger signal on INT1
    #[bits(1, access = RO)]
    pub trigger_on_int2: bool,

    // TODO: document
    #[bits(5, access = RO)]
    pub fth: u8,
}

impl Register for FifoControlRegisterA {
    const DEV_ADDRESS: u8 = accel::ADDRESS;
    const REG_ADDRESS: u8 = AccelerometerRegister::FIFO_CTRL_REG_A.addr();

    fn from_bits(bits: u8) -> Self {
        Self::from_bits(bits)
    }

    fn to_bits(&self) -> u8 {
        self.into_bits()
    }
}

impl WritableRegister for FifoControlRegisterA {}

/// [`FIFO_CTRL_REG_A`](accel::AccelerometerRegister::FIFO_SRC_REG_A) (2Fh)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct FifoSourceRegisterA {
    #[bits(1, access = RO)]
    pub wtm: bool,

    #[bits(1, access = RO)]
    pub ovrn_fifo: bool,

    #[bits(1, access = RO)]
    pub empty: bool,

    #[bits(5, access = RO)]
    pub fss: u8,
}

impl Register for FifoSourceRegisterA {
    const DEV_ADDRESS: u8 = accel::ADDRESS;
    const REG_ADDRESS: u8 = AccelerometerRegister::FIFO_SRC_REG_A.addr();

    fn from_bits(bits: u8) -> Self {
        Self::from_bits(bits)
    }

    fn to_bits(&self) -> u8 {
        self.into_bits()
    }
}

/// [`INT1_CFG_A`](accel::AccelerometerRegister::INT1_CFG_A) (2Fh)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Int1ConfigurationRegisterA {
    /// AND/OR combination of interrupt events.
    #[bits(1, access = RW)]
    pub aoi: bool,

    /// 6-direction detection function enabled.
    #[bits(1, access = RW)]
    pub six_d: bool,

    /// Enable interrupt generation on Z high event or on direction recognition.
    #[bits(1, access = RW)]
    pub zhie_zupe: bool,

    /// Enable interrupt generation on Z low event or on direction recognition.
    #[bits(1, access = RW)]
    pub zlie_zdowne: bool,

    /// Enable interrupt generation on Y high event or on direction recognition.
    #[bits(1, access = RW)]
    pub yhie_yupe: bool,

    /// Enable interrupt generation on Y low event or on direction recognition.
    #[bits(1, access = RW)]
    pub ylie_ydowne: bool,

    /// Enable interrupt generation on X high event or on direction recognition.
    #[bits(1, access = RW)]
    pub xhie_xupe: bool,

    /// Enable interrupt generation on X low event or on direction recognition.
    #[bits(1, access = RW)]
    pub xlie_xdowne: bool,
}

impl Register for Int1ConfigurationRegisterA {
    const DEV_ADDRESS: u8 = accel::ADDRESS;
    const REG_ADDRESS: u8 = AccelerometerRegister::INT1_CFG_A.addr();

    fn from_bits(bits: u8) -> Self {
        Self::from_bits(bits)
    }

    fn to_bits(&self) -> u8 {
        self.into_bits()
    }
}

impl WritableRegister for Int1ConfigurationRegisterA {}

/// [`INT1_SRC_A`](accel::AccelerometerRegister::INT1_SRC_A) (31h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Int1SourceRegisterA {
    #[bits(1, default = false)]
    zero: bool,

    /// Interrupt active.
    #[bits(1, access = RO)]
    pub ia: bool,

    /// Z high.
    #[bits(1, access = RO)]
    pub z_high: bool,

    /// Z low.
    #[bits(1, access = RO)]
    pub z_low: bool,

    /// Y high.
    #[bits(1, access = RO)]
    pub y_high: bool,

    /// Y low.
    #[bits(1, access = RO)]
    pub y_low: bool,

    /// X high.
    #[bits(1, access = RO)]
    pub x_high: bool,

    /// X low.
    #[bits(1, access = RO)]
    pub x_low: bool,
}

impl Register for Int1SourceRegisterA {
    const DEV_ADDRESS: u8 = accel::ADDRESS;
    const REG_ADDRESS: u8 = AccelerometerRegister::INT1_SRC_A.addr();

    fn from_bits(bits: u8) -> Self {
        Self::from_bits(bits)
    }

    fn to_bits(&self) -> u8 {
        self.into_bits()
    }
}

/// [`INT1_SRC_A`](accel::AccelerometerRegister::INT1_THS_A) (32h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Int1ThresholdRegisterA {
    #[bits(1, default = false)]
    zero: bool,

    /// Interrupt 1 threshold.
    #[bits(7, access = RW, default = 0)]
    pub threshold: u8,
}

impl Register for Int1ThresholdRegisterA {
    const DEV_ADDRESS: u8 = accel::ADDRESS;
    const REG_ADDRESS: u8 = AccelerometerRegister::INT1_THS_A.addr();

    fn from_bits(bits: u8) -> Self {
        Self::from_bits(bits)
    }

    fn to_bits(&self) -> u8 {
        self.into_bits()
    }
}

impl WritableRegister for Int1ThresholdRegisterA {}

/// [`INT1_DURATION_A`](accel::AccelerometerRegister::INT1_DURATION_A) (33h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Int1DurationRegisterA {
    #[bits(1, default = false)]
    zero: bool,

    /// The minimum duration of the Interrupt 1 event to be recognized. Duration
    /// steps and maximum values depend on the ODR chosen.
    #[bits(7, access = RW, default = 0)]
    pub duration: u8,
}

impl Register for Int1DurationRegisterA {
    const DEV_ADDRESS: u8 = accel::ADDRESS;
    const REG_ADDRESS: u8 = AccelerometerRegister::INT1_DURATION_A.addr();

    fn from_bits(bits: u8) -> Self {
        Self::from_bits(bits)
    }

    fn to_bits(&self) -> u8 {
        self.into_bits()
    }
}

impl WritableRegister for Int1DurationRegisterA {}

/// [`INT2_CFG_A`](accel::AccelerometerRegister::INT2_CFG_A) (34h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Int2ConfigurationRegisterA {
    /// AND/OR combination of interrupt events.
    #[bits(1, access = RW, default = false)]
    pub aoi: bool,

    /// 6-direction detection function enabled
    #[bits(1, access = RW, default = false)]
    pub six_d: bool,

    /// Enable interrupt generation on Z high event
    ///
    /// * `false` - disable interrupt request
    /// * `true` - enable interrupt request on measured accel. value higher than preset threshold
    #[bits(1, access = RW, default = false)]
    pub zhie: bool,

    /// Enable interrupt generation on Z low event
    ///
    /// * `false` - disable interrupt request
    /// * `true` - enable interrupt request on measured accel. value lower than preset threshold
    #[bits(1, access = RW, default = false)]
    pub zlie: bool,

    /// Enable interrupt generation on Y high event
    ///
    /// * `false` - disable interrupt request
    /// * `true` - enable interrupt request on measured accel. value higher than preset threshold
    #[bits(1, access = RW, default = false)]
    pub yhie: bool,

    /// Enable interrupt generation on Y low event
    ///
    /// * `false` - disable interrupt request
    /// * `true` - enable interrupt request on measured accel. value lower than preset threshold
    #[bits(1, access = RW, default = false)]
    pub ylie: bool,

    /// Enable interrupt generation on X high event
    ///
    /// * `false` - disable interrupt request
    /// * `true` - enable interrupt request on measured accel. value higher than preset threshold
    #[bits(1, access = RW, default = false)]
    pub xhie: bool,

    /// Enable interrupt generation on X low event
    ///
    /// * `false` - disable interrupt request
    /// * `true` - enable interrupt request on measured accel. value lower than preset threshold
    #[bits(1, access = RW, default = false)]
    pub xlie: bool,
}

impl Register for Int2ConfigurationRegisterA {
    const DEV_ADDRESS: u8 = accel::ADDRESS;
    const REG_ADDRESS: u8 = AccelerometerRegister::INT2_CFG_A.addr();

    fn from_bits(bits: u8) -> Self {
        Self::from_bits(bits)
    }

    fn to_bits(&self) -> u8 {
        self.into_bits()
    }
}

impl WritableRegister for Int2ConfigurationRegisterA {}

/// [`INT2_SRC_A`](accel::AccelerometerRegister::INT2_SRC_A) (35h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Int2SourceRegisterA {
    #[bits(1, default = false)]
    zero: bool,

    /// Interrupt active.
    #[bits(1, access = RO)]
    pub ia: bool,

    /// Z high.
    #[bits(1, access = RO)]
    pub z_high: bool,

    /// Z low.
    #[bits(1, access = RO)]
    pub z_low: bool,

    /// Y high.
    #[bits(1, access = RO)]
    pub y_high: bool,

    /// Y low.
    #[bits(1, access = RO)]
    pub y_low: bool,

    /// X high.
    #[bits(1, access = RO)]
    pub x_high: bool,

    /// X low.
    #[bits(1, access = RO)]
    pub x_low: bool,
}

impl Register for Int2SourceRegisterA {
    const DEV_ADDRESS: u8 = accel::ADDRESS;
    const REG_ADDRESS: u8 = AccelerometerRegister::INT2_SRC_A.addr();

    fn from_bits(bits: u8) -> Self {
        Self::from_bits(bits)
    }

    fn to_bits(&self) -> u8 {
        self.into_bits()
    }
}

/// [`INT2_SRC_A`](accel::AccelerometerRegister::INT2_THS_A) (36h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Int2ThresholdRegisterA {
    #[bits(1, default = false)]
    zero: bool,

    /// Interrupt 2 threshold.
    #[bits(7, access = RW, default = 0)]
    pub threshold: u8,
}

impl Register for Int2ThresholdRegisterA {
    const DEV_ADDRESS: u8 = accel::ADDRESS;
    const REG_ADDRESS: u8 = AccelerometerRegister::INT2_THS_A.addr();

    fn from_bits(bits: u8) -> Self {
        Self::from_bits(bits)
    }

    fn to_bits(&self) -> u8 {
        self.into_bits()
    }
}

impl WritableRegister for Int2ThresholdRegisterA {}

/// [`INT2_DURATION_A`](accel::AccelerometerRegister::INT2_DURATION_A) (37h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Int2DurationRegisterA {
    #[bits(1, default = false)]
    zero: bool,

    /// The minimum duration of the Interrupt 1 event to be recognized. Duration
    /// steps and maximum values depend on the ODR chosen.
    #[bits(7, access = RW, default = 0)]
    pub duration: u8,
}

impl Register for Int2DurationRegisterA {
    const DEV_ADDRESS: u8 = accel::ADDRESS;
    const REG_ADDRESS: u8 = AccelerometerRegister::INT2_DURATION_A.addr();

    fn from_bits(bits: u8) -> Self {
        Self::from_bits(bits)
    }

    fn to_bits(&self) -> u8 {
        self.into_bits()
    }
}

impl WritableRegister for Int2DurationRegisterA {}

/// [`CLICK_CFG_A`](accel::AccelerometerRegister::CLICK_CFG_A) (38h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ClickConfigurationRegisterA {
    #[bits(2)]
    __: u8,

    /// Enable interrupt double-click in Z-axis.
    ///
    /// * `false` - disable interrupt request
    /// * `true` - enable interrupt request on measured accel. value higher than preset threshold
    #[bits(1, access = RW)]
    pub zd: bool,

    /// Enable interrupt single-click in Z-axis.
    ///
    /// * `false` - disable interrupt request
    /// * `true` - enable interrupt request on measured accel. value higher than preset threshold
    #[bits(1, access = RW)]
    pub zs: bool,

    /// Enable interrupt double-click in Y-axis.
    ///
    /// * `false` - disable interrupt request
    /// * `true` - enable interrupt request on measured accel. value higher than preset threshold
    #[bits(1, access = RW)]
    pub yd: bool,

    /// Enable interrupt single-click in Y-axis.
    ///
    /// * `false` - disable interrupt request
    /// * `true` - enable interrupt request on measured accel. value higher than preset threshold
    #[bits(1, access = RW)]
    pub ys: bool,

    /// Enable interrupt double-click in X-axis.
    ///
    /// * `false` - disable interrupt request
    /// * `true` - enable interrupt request on measured accel. value higher than preset threshold
    #[bits(1, access = RW)]
    pub xd: bool,

    /// Enable interrupt single-click in X-axis.
    ///
    /// * `false` - disable interrupt request
    /// * `true` - enable interrupt request on measured accel. value higher than preset threshold
    #[bits(1, access = RW)]
    pub xs: bool,
}

impl Register for ClickConfigurationRegisterA {
    const DEV_ADDRESS: u8 = accel::ADDRESS;
    const REG_ADDRESS: u8 = AccelerometerRegister::CLICK_CFG_A.addr();

    fn from_bits(bits: u8) -> Self {
        Self::from_bits(bits)
    }

    fn to_bits(&self) -> u8 {
        self.into_bits()
    }
}

impl WritableRegister for ClickConfigurationRegisterA {}

/// [`CLICK_SRC_A`](accel::AccelerometerRegister::CLICK_SRC_A) (39h)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ClickSourceRegisterA {
    #[bits(1)]
    __: bool,

    /// Interrupt active.
    #[bits(1, access = RO)]
    pub ia: bool,

    /// Double-click enable.
    ///
    /// * `false` - double-click detection disable
    /// * `true` - double-click detection enable
    #[bits(1, access = RO)]
    pub dclick: bool,

    /// Single-click enable.
    ///
    /// * `false` - single-click detection disable
    /// * `true` - signle-click detection enable
    #[bits(1, access = RO)]
    pub sclick: bool,

    /// Click sign.
    #[bits(1, access = RO)]
    pub sign_negative: bool,

    /// Z-click detection
    ///
    /// * `false` - no interrupt
    /// * `true` - Z high event has occurred
    #[bits(1, access = RO)]
    pub z: bool,

    /// Y-click detection
    ///
    /// * `false` - no interrupt
    /// * `true` - Y high event has occurred
    #[bits(1, access = RO)]
    pub y: bool,

    /// X-click detection
    ///
    /// * `false` - no interrupt
    /// * `true` - X high event has occurred
    #[bits(1, access = RO)]
    pub x: bool,
}

impl Register for ClickSourceRegisterA {
    const DEV_ADDRESS: u8 = accel::ADDRESS;
    const REG_ADDRESS: u8 = AccelerometerRegister::CLICK_SRC_A.addr();

    fn from_bits(bits: u8) -> Self {
        Self::from_bits(bits)
    }

    fn to_bits(&self) -> u8 {
        self.into_bits()
    }
}

/// [`CLICK_THS_A`](accel::AccelerometerRegister::CLICK_THS_A) (3Ah)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ClickThresholdRegisterA {
    #[bits(1, default = false)]
    zero: bool,

    /// Click threshold.
    ///
    /// 1 LSB = full-scale / 128.
    /// Ths6 through Ths0 define the threshold which is used by the system to start the
    /// click-detection procedure. The threshold value is expressed over 7 bits
    /// as an unsigned number.
    #[bits(7, access = RW, default = 0)]
    pub threshold: u8,
}

impl Register for ClickThresholdRegisterA {
    const DEV_ADDRESS: u8 = accel::ADDRESS;
    const REG_ADDRESS: u8 = AccelerometerRegister::CLICK_THS_A.addr();

    fn from_bits(bits: u8) -> Self {
        Self::from_bits(bits)
    }

    fn to_bits(&self) -> u8 {
        self.into_bits()
    }
}

impl WritableRegister for ClickThresholdRegisterA {}

/// [`TIME_LIMIT_A`](accel::AccelerometerRegister::TIME_LIMIT_A) (3Bh)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ClickTimeLimitRegisterA {
    #[bits(1, default = false)]
    zero: bool,

    /// Click time limit.
    ///
    /// 1 LSB = 1/ODR. TLI6 through TLI0 define the maximum time interval that can elapse
    /// between the start of the click-detection procedure (the acceleration on the selected channel
    /// exceeds the programmed threshold) and when the acceleration falls below the threshold.
    #[bits(7, access = RW, default = 0)]
    pub time_limit: u8,
}

impl Register for ClickTimeLimitRegisterA {
    const DEV_ADDRESS: u8 = accel::ADDRESS;
    const REG_ADDRESS: u8 = AccelerometerRegister::TIME_LIMIT_A.addr();

    fn from_bits(bits: u8) -> Self {
        Self::from_bits(bits)
    }

    fn to_bits(&self) -> u8 {
        self.into_bits()
    }
}

impl WritableRegister for ClickTimeLimitRegisterA {}

/// [`TIME_LATENCY_A`](accel::AccelerometerRegister::TIME_LATENCY_A) (3Ch)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ClickTimeLatencyRegisterA {
    /// Double-click time latency.
    ///
    /// 1 LSB = 1/ODR. TLA7 through TLA0 define the time interval that starts after the first click
    /// detection where the click-detection procedure is disabled, in cases where the device is
    /// configured for double-click detection.
    #[bits(8, access = RW, default = 0)]
    pub time_latency: u8,
}

impl Register for ClickTimeLatencyRegisterA {
    const DEV_ADDRESS: u8 = accel::ADDRESS;
    const REG_ADDRESS: u8 = AccelerometerRegister::TIME_LATENCY_A.addr();

    fn from_bits(bits: u8) -> Self {
        Self::from_bits(bits)
    }

    fn to_bits(&self) -> u8 {
        self.into_bits()
    }
}

impl WritableRegister for ClickTimeLatencyRegisterA {}

/// [`TIME_WINDOW_A`](accel::AccelerometerRegister::TIME_WINDOW_A) (3Dh)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ClickTimeWindowRegisterA {
    /// Double-click time window.
    ///
    /// 1 LSB = 1/ODR. TW7 through TW0 define the maximum interval of time that can elapse
    /// after the end of the latency interval in which the click detection procedure can start, in cases
    /// where the device is configured for double-click detection.
    #[bits(8, access = RW, default = 0)]
    pub time_window: u8,
}

impl Register for ClickTimeWindowRegisterA {
    const DEV_ADDRESS: u8 = accel::ADDRESS;
    const REG_ADDRESS: u8 = AccelerometerRegister::TIME_WINDOW_A.addr();

    fn from_bits(bits: u8) -> Self {
        Self::from_bits(bits)
    }

    fn to_bits(&self) -> u8 {
        self.into_bits()
    }
}

impl WritableRegister for ClickTimeWindowRegisterA {}

/// [`CRA_REG_M`](mag::MagnetometerRegister::CRA_REG_M) (00h)
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
    const DEV_ADDRESS: u8 = mag::ADDRESS;
    const REG_ADDRESS: u8 = MagnetometerRegister::CRA_REG_M.addr();

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
/// [`CRB_REG_M`](mag::MagnetometerRegister::CRB_REG_M) (01h)
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
    const DEV_ADDRESS: u8 = mag::ADDRESS;
    const REG_ADDRESS: u8 = MagnetometerRegister::CRB_REG_M.addr();

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
/// [`MR_REG_M`](mag::MagnetometerRegister::MR_REG_M) (09h)
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
    const DEV_ADDRESS: u8 = mag::ADDRESS;
    const REG_ADDRESS: u8 = MagnetometerRegister::MR_REG_M.addr();

    fn from_bits(bits: u8) -> Self {
        Self::from_bits(bits)
    }

    fn to_bits(&self) -> u8 {
        self.into_bits()
    }
}

impl WritableRegister for ModeRegisterM {}

/// [`SR_REG_M`](mag::MagnetometerRegister::SR_REG_M) (09h)
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
    const DEV_ADDRESS: u8 = mag::ADDRESS;
    const REG_ADDRESS: u8 = MagnetometerRegister::SR_REG_M.addr();

    fn from_bits(bits: u8) -> Self {
        Self::from_bits(bits)
    }

    fn to_bits(&self) -> u8 {
        self.into_bits()
    }
}

/// [`IRA_REG_M`](mag::MagnetometerRegister::IRA_REG_M) (0Ah)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct IRARegisterM {
    #[bits(8, access = RO)]
    pub value: u8,
}

impl Register for IRARegisterM {
    const DEV_ADDRESS: u8 = mag::ADDRESS;
    const REG_ADDRESS: u8 = MagnetometerRegister::IRA_REG_M.addr();

    fn from_bits(bits: u8) -> Self {
        Self::from_bits(bits)
    }

    fn to_bits(&self) -> u8 {
        self.into_bits()
    }
}

/// [`IRB_REG_M`](mag::MagnetometerRegister::IRB_REG_M) (0Bh)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct IRBRegisterM {
    #[bits(8, access = RO)]
    pub value: u8,
}

impl Register for IRBRegisterM {
    const DEV_ADDRESS: u8 = mag::ADDRESS;
    const REG_ADDRESS: u8 = MagnetometerRegister::IRB_REG_M.addr();

    fn from_bits(bits: u8) -> Self {
        Self::from_bits(bits)
    }

    fn to_bits(&self) -> u8 {
        self.into_bits()
    }
}

/// [`IRC_REG_M`](mag::MagnetometerRegister::IRC_REG_M) (0Ch)
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct IRCRegisterM {
    #[bits(8, access = RO)]
    pub value: u8,
}

impl Register for IRCRegisterM {
    const DEV_ADDRESS: u8 = mag::ADDRESS;
    const REG_ADDRESS: u8 = MagnetometerRegister::IRC_REG_M.addr();

    fn from_bits(bits: u8) -> Self {
        Self::from_bits(bits)
    }

    fn to_bits(&self) -> u8 {
        self.into_bits()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::unusual_byte_groupings)]
    fn status_register_1a() {
        let reg = ControlRegister1A::new()
            .with_output_data_rate(AccelOdr::Hz400)
            .with_low_power_enable(false)
            .with_x_enable(true)
            .with_y_enable(true)
            .with_z_enable(true);

        assert_eq!(reg.into_bits(), 0b0111_0_111);
    }
}
