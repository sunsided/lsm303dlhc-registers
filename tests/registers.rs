use lsm303dlhc_registers::ReservedBitPattern;
use lsm303dlhc_registers::accel::{AccelOdr, ControlRegister1A, ControlRegister4A};
use lsm303dlhc_registers::mag::{ConfigurationBRegisterM, MagGain};
use lsm303dlhc_registers::prelude::*;

#[test]
fn addresses() {
    assert_eq!(
        ControlRegister1A::DEFAULT_DEVICE_ADDRESS,
        DeviceAddress7::new(0x19)
    );
    assert_eq!(
        ControlRegister1A::REGISTER_ADDRESS,
        RegisterAddress8::new(0x20)
    );
    assert_eq!(
        ConfigurationBRegisterM::DEFAULT_DEVICE_ADDRESS,
        DeviceAddress7::new(0x1E)
    );
    assert_eq!(
        ConfigurationBRegisterM::REGISTER_ADDRESS,
        RegisterAddress8::new(0x01)
    );
}

#[test]
fn accel_odr_rejects_reserved_patterns() {
    let reg = ControlRegister1A::try_from_bits(0b0111_0111).unwrap();
    assert_eq!(reg.output_data_rate(), AccelOdr::Hz400);
    assert_eq!(reg.to_bits(), 0b0111_0111);

    assert_eq!(
        ControlRegister1A::try_from_bits(0b1010_0111),
        Err(ReservedBitPattern(0b1010_0111))
    );
}

#[test]
fn mag_gain_rejects_reserved_pattern() {
    let reg = ConfigurationBRegisterM::try_from_bits(0b1110_0000).unwrap();
    assert_eq!(reg.gain(), MagGain::Gauss8_1);
    assert_eq!(
        ConfigurationBRegisterM::try_from_bits(0),
        Err(ReservedBitPattern(0))
    );
}

#[test]
fn infallible_registers_decode_every_pattern() {
    for bits in 0..=u8::MAX {
        assert_eq!(ControlRegister4A::from_bits(bits).to_bits(), bits);
        assert!(ControlRegister4A::try_from_bits(bits).is_ok());
    }
}

#[test]
fn byte_pairs_combine_into_i16() {
    use lsm303dlhc_registers::{accel, mag};

    let (lo, hi) = (accel::OutXLowA::from(0x34), accel::OutXHighA::from(0xFE));
    assert_eq!(lo + hi, -460);
    assert_eq!(hi + lo, -460);

    let (lo, hi) = (mag::OutZLowM::from(0x01), mag::OutZHighM::from(0x80));
    assert_eq!(lo + hi, i16::MIN + 1);
    assert_eq!(hi + lo, i16::MIN + 1);
}
