//! Register addresses, power-on defaults and bit positions, checked against the LSM303DLHC
//! datasheet (DocID018771 Rev 2): Table 17 (register map) and Tables 18-85 (register layouts).

use lsm303dlhc_registers::accel as a;
use lsm303dlhc_registers::mag as m;
use lsm303dlhc_registers::prelude::*;

/// Checks the register address and that `new()` yields the Table 17 default.
macro_rules! default {
    ($reg:ty, $addr:expr, $default:expr) => {
        assert_eq!(
            <$reg>::REGISTER_ADDRESS,
            RegisterAddress8::new($addr),
            "{} address",
            stringify!($reg)
        );
        assert_eq!(
            <$reg>::new().to_bits(),
            $default,
            "{} default",
            stringify!($reg)
        );
    };
}

#[test]
fn accelerometer_addresses_and_defaults() {
    default!(a::ControlRegister1A, 0x20, 0b0000_0111);
    default!(a::ControlRegister2A, 0x21, 0);
    default!(a::ControlRegister3A, 0x22, 0);
    default!(a::ControlRegister4A, 0x23, 0);
    default!(a::ControlRegister5A, 0x24, 0);
    default!(a::ControlRegister6A, 0x25, 0);
    default!(a::ReferenceRegisterA, 0x26, 0);
    default!(a::StatusRegisterA, 0x27, 0);
    default!(a::OutXLowA, 0x28, 0);
    default!(a::OutXHighA, 0x29, 0);
    default!(a::OutYLowA, 0x2A, 0);
    default!(a::OutYHighA, 0x2B, 0);
    default!(a::OutZLowA, 0x2C, 0);
    default!(a::OutZHighA, 0x2D, 0);
    default!(a::FifoControlRegisterA, 0x2E, 0);
    default!(a::FifoSourceRegisterA, 0x2F, 0);
    default!(a::Int1ConfigurationRegisterA, 0x30, 0);
    default!(a::Int1SourceRegisterA, 0x31, 0);
    default!(a::Int1ThresholdRegisterA, 0x32, 0);
    default!(a::Int1DurationRegisterA, 0x33, 0);
    default!(a::Int2ConfigurationRegisterA, 0x34, 0);
    default!(a::Int2SourceRegisterA, 0x35, 0);
    default!(a::Int2ThresholdRegisterA, 0x36, 0);
    default!(a::Int2DurationRegisterA, 0x37, 0);
    default!(a::ClickConfigurationRegisterA, 0x38, 0);
    default!(a::ClickSourceRegisterA, 0x39, 0);
    default!(a::ClickThresholdRegisterA, 0x3A, 0);
    default!(a::ClickTimeLimitRegisterA, 0x3B, 0);
    default!(a::ClickTimeLatencyRegisterA, 0x3C, 0);
    default!(a::ClickTimeWindowRegisterA, 0x3D, 0);
}

#[test]
fn magnetometer_addresses_and_defaults() {
    default!(m::ConfigurationARegisterM, 0x00, 0b0001_0000);
    default!(m::ConfigurationBRegisterM, 0x01, 0b0010_0000);
    default!(m::ModeRegisterM, 0x02, 0b0000_0011);
    default!(m::OutXHighM, 0x03, 0);
    default!(m::OutXLowM, 0x04, 0);
    default!(m::OutZHighM, 0x05, 0);
    default!(m::OutZLowM, 0x06, 0);
    default!(m::OutYHighM, 0x07, 0);
    default!(m::OutYLowM, 0x08, 0);
    default!(m::StatusRegisterM, 0x09, 0);
    default!(m::IdentificationARegisterM, 0x0A, 0b0100_1000);
    default!(m::IdentificationBRegisterM, 0x0B, 0b0011_0100);
    default!(m::IdentificationCRegisterM, 0x0C, 0b0011_0011);
    default!(m::TemperatureOutHighM, 0x31, 0);
    default!(m::TemperatureOutLowM, 0x32, 0);
}

#[test]
fn device_addresses() {
    assert_eq!(a::DEFAULT_DEVICE_ADDRESS, 0b001_1001);
    assert_eq!(m::DEFAULT_DEVICE_ADDRESS, 0b001_1110);
}

#[test]
fn accelerometer_control_bit_positions() {
    let r1 = a::ControlRegister1A::from(0);
    assert_eq!(
        r1.with_output_data_rate(a::AccelOdr::LpHz5376NormalHz1344)
            .into_bits(),
        0x90
    );
    assert_eq!(r1.with_low_power_enable(true).into_bits(), 0x08);
    assert_eq!(r1.with_z_enable(true).into_bits(), 0x04);
    assert_eq!(r1.with_y_enable(true).into_bits(), 0x02);
    assert_eq!(r1.with_x_enable(true).into_bits(), 0x01);

    let r2 = a::ControlRegister2A::new();
    assert_eq!(
        r2.with_hpm(a::HighpassFilterMode::AutoresetOnInterrupt)
            .into_bits(),
        0xC0
    );
    assert_eq!(r2.with_hpcf(0b11).into_bits(), 0x30);
    assert_eq!(r2.with_fds(true).into_bits(), 0x08);
    assert_eq!(r2.with_hpclick(true).into_bits(), 0x04);
    assert_eq!(r2.with_hpis2(true).into_bits(), 0x02);
    assert_eq!(r2.with_hpis1(true).into_bits(), 0x01);

    let r3 = a::ControlRegister3A::new();
    assert_eq!(r3.with_i1click(true).into_bits(), 0x80);
    assert_eq!(r3.with_i1aoi1(true).into_bits(), 0x40);
    assert_eq!(r3.with_i1aoi2(true).into_bits(), 0x20);
    assert_eq!(r3.with_i1drdy1(true).into_bits(), 0x10);
    assert_eq!(r3.with_i1drdy2(true).into_bits(), 0x08);
    assert_eq!(r3.with_i1wtm(true).into_bits(), 0x04);
    assert_eq!(r3.with_i1overrun(true).into_bits(), 0x02);

    let r4 = a::ControlRegister4A::new();
    assert_eq!(r4.with_block_data_update(true).into_bits(), 0x80);
    assert_eq!(r4.with_big_endian(true).into_bits(), 0x40);
    assert_eq!(r4.with_full_scale(a::Sensitivity::G12).into_bits(), 0x30);
    assert_eq!(r4.with_full_scale(a::Sensitivity::G2).into_bits(), 0x10);
    assert_eq!(r4.with_high_resolution(true).into_bits(), 0x08);
    assert_eq!(r4.with_spi_serial_3wire(true).into_bits(), 0x01);

    let r5 = a::ControlRegister5A::new();
    assert_eq!(r5.with_boot(true).into_bits(), 0x80);
    assert_eq!(r5.with_fifo_enable(true).into_bits(), 0x40);
    assert_eq!(r5.with_lir_int1(true).into_bits(), 0x08);
    assert_eq!(r5.with_d4d_int1(true).into_bits(), 0x04);
    assert_eq!(r5.with_lir_int2(true).into_bits(), 0x02);
    assert_eq!(r5.with_d4d_int2(true).into_bits(), 0x01);

    let r6 = a::ControlRegister6A::new();
    assert_eq!(r6.with_i2click_en(true).into_bits(), 0x80);
    assert_eq!(r6.with_i2int1(true).into_bits(), 0x40);
    assert_eq!(r6.with_i2int2(true).into_bits(), 0x20);
    assert_eq!(r6.with_boot_i1(true).into_bits(), 0x10);
    assert_eq!(r6.with_p2_active(true).into_bits(), 0x08);
    assert_eq!(r6.with_active_low(true).into_bits(), 0x02);
}

#[test]
fn accelerometer_status_and_fifo_bit_positions() {
    let s = |bits| a::StatusRegisterA::from(bits);
    assert!(s(0x80).zyx_overrun() && !s(0x80).z_overrun());
    assert!(s(0x40).z_overrun());
    assert!(s(0x20).y_overrun());
    assert!(s(0x10).x_overrun());
    assert!(s(0x08).xyz_data_available() && !s(0x08).x_overrun());
    assert!(s(0x04).z_data_available());
    assert!(s(0x02).y_data_available());
    assert!(s(0x01).x_data_available());

    let fifo = a::FifoControlRegisterA::new();
    assert_eq!(fifo.with_fifo_mode(a::FifoMode::Trigger).into_bits(), 0xC0);
    assert_eq!(fifo.with_fifo_mode(a::FifoMode::FIFO).into_bits(), 0x40);
    assert_eq!(fifo.with_trigger_on_int2(true).into_bits(), 0x20);
    assert_eq!(fifo.with_fth(0x1F).into_bits(), 0x1F);

    let src = a::FifoSourceRegisterA::from(0b1010_0101);
    assert!(src.wtm() && !src.ovrn_fifo() && src.empty());
    assert_eq!(src.fss(), 0b0_0101);
    assert!(a::FifoSourceRegisterA::from(0x40).ovrn_fifo());
}

#[test]
fn accelerometer_interrupt_and_click_bit_positions() {
    let cfg1 = a::Int1ConfigurationRegisterA::new();
    assert_eq!(cfg1.with_aoi(true).into_bits(), 0x80);
    assert_eq!(cfg1.with_six_d(true).into_bits(), 0x40);
    assert_eq!(cfg1.with_zhie_zupe(true).into_bits(), 0x20);
    assert_eq!(cfg1.with_xlie_xdowne(true).into_bits(), 0x01);

    let cfg2 = a::Int2ConfigurationRegisterA::new();
    assert_eq!(cfg2.with_aoi(true).into_bits(), 0x80);
    assert_eq!(cfg2.with_six_d(true).into_bits(), 0x40);
    assert_eq!(cfg2.with_zhie(true).into_bits(), 0x20);
    assert_eq!(cfg2.with_xlie(true).into_bits(), 0x01);

    let src1 = a::Int1SourceRegisterA::from(0x40);
    assert!(src1.ia() && !src1.z_high());
    assert!(a::Int1SourceRegisterA::from(0x20).z_high());
    assert!(a::Int1SourceRegisterA::from(0x01).x_low());
    let src2 = a::Int2SourceRegisterA::from(0x40);
    assert!(src2.ia() && !src2.z_high());
    assert!(a::Int2SourceRegisterA::from(0x01).x_low());

    assert_eq!(
        a::Int1ThresholdRegisterA::new()
            .with_threshold(0x7F)
            .into_bits(),
        0x7F
    );
    assert_eq!(
        a::Int1DurationRegisterA::new()
            .with_duration(0x7F)
            .into_bits(),
        0x7F
    );
    assert_eq!(
        a::Int2ThresholdRegisterA::new()
            .with_threshold(0x7F)
            .into_bits(),
        0x7F
    );
    assert_eq!(
        a::Int2DurationRegisterA::new()
            .with_duration(0x7F)
            .into_bits(),
        0x7F
    );

    let click = a::ClickConfigurationRegisterA::new();
    assert_eq!(click.with_zd(true).into_bits(), 0x20);
    assert_eq!(click.with_zs(true).into_bits(), 0x10);
    assert_eq!(click.with_yd(true).into_bits(), 0x08);
    assert_eq!(click.with_ys(true).into_bits(), 0x04);
    assert_eq!(click.with_xd(true).into_bits(), 0x02);
    assert_eq!(click.with_xs(true).into_bits(), 0x01);

    let c = |bits| a::ClickSourceRegisterA::from(bits);
    assert!(c(0x40).ia() && !c(0x40).dclick());
    assert!(c(0x20).dclick());
    assert!(c(0x10).sclick());
    assert!(c(0x08).sign_negative());
    assert!(c(0x04).z() && c(0x02).y() && c(0x01).x());

    assert_eq!(
        a::ClickThresholdRegisterA::new()
            .with_threshold(0x7F)
            .into_bits(),
        0x7F
    );
    assert_eq!(
        a::ClickTimeLimitRegisterA::new()
            .with_time_limit(0x7F)
            .into_bits(),
        0x7F
    );
    assert_eq!(
        a::ClickTimeLatencyRegisterA::new()
            .with_time_latency(0xFF)
            .into_bits(),
        0xFF
    );
    assert_eq!(
        a::ClickTimeWindowRegisterA::new()
            .with_time_window(0xFF)
            .into_bits(),
        0xFF
    );
    assert_eq!(
        a::ReferenceRegisterA::new()
            .with_reference(0xFF)
            .into_bits(),
        0xFF
    );
}

#[test]
fn magnetometer_bit_positions() {
    let cra = m::ConfigurationARegisterM::new();
    assert_eq!(cra.with_temp_en(true).into_bits(), 0x90);
    assert_eq!(
        cra.with_data_output_rate(m::MagOdr::Hz220).into_bits(),
        0x1C
    );
    assert_eq!(
        cra.with_data_output_rate(m::MagOdr::Hz0_75).into_bits(),
        0x00
    );

    let crb = m::ConfigurationBRegisterM::new();
    assert_eq!(crb.with_gain(m::MagGain::Gauss8_1).into_bits(), 0xE0);

    // Table 78: MD1:MD0 = 00 continuous, 01 single, 1x sleep.
    let mr = m::ModeRegisterM::new();
    assert_eq!(
        mr.with_sleep_mode(false)
            .with_single_conversion(false)
            .into_bits(),
        0x00
    );
    assert_eq!(mr.with_sleep_mode(false).into_bits(), 0x01);
    assert_eq!(mr.with_single_conversion(false).into_bits(), 0x02);

    assert!(m::StatusRegisterM::from(0x02).do_lock());
    assert!(m::StatusRegisterM::from(0x01).data_ready());
    assert!(!m::StatusRegisterM::from(0x01).do_lock());

    // Table 85: TEMP[3:0] in bits 7:4.
    assert_eq!(m::TemperatureOutLowM::from(0xF0).value(), 0xF);
    assert_eq!(m::TemperatureOutLowM::from(0x0F).value(), 0);
}
