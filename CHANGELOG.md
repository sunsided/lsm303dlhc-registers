# Changelog

All notable changes to this project will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- Documentation: magnetometer output registers no longer call their value an acceleration,
  `TEMP_OUT_H_M`/`TEMP_OUT_L_M` show their addresses (31h/32h), and `INT2_CFG_A` links to
  `Int2ConfigurationRegisterA`.

### Internal

- Removed `#![deny(warnings)]`, so new compiler warnings cannot break docs.rs builds; CI
  still denies warnings.
- Added tests that check every register address, power-on default and field position
  against the datasheet (DocID018771 Rev 2, Tables 17-85).

## [0.2.0] - 2026-11-15

[0.2.0]: https://github.com/sunsided/lsm303dlhc-registers/releases/tag/v0.2.0

### Added

- `ReservedBitPattern` error type. `ControlRegister1A` and `ConfigurationBRegisterM` implement
  `TryFromBits<u8>` and reject reserved output data rate (`1010`–`1111`) and gain (`000`) patterns.
- `AccelOdr::try_from_bits` and `MagGain::try_from_bits`.
- `Sensitivity::milli_g_per_lsb`, `MagGain::lsb_per_gauss_xy`, `MagGain::lsb_per_gauss_z` and
  `mag::TEMPERATURE_LSB_PER_CELSIUS` for conversion to physical units.
- `Debug` for the `accel::RegisterAddress` and `mag::RegisterAddress` enums.

### Changed

- [**breaking**] Updated to `hardware-registers` 0.4.1. `Register` now requires
  `ReadableI2CRegister8`, as 0.4 no longer implies readability.
- [**breaking**] `ControlRegister1A` and `ConfigurationBRegisterM` no longer implement `FromBits<u8>`;
  use `TryFromBits<u8>`.
- [**breaking**] Updated to `defmt` 1.0 and `bitfield-struct` 0.13.
- [**breaking**] Edition 2024; the minimum supported Rust version is now 1.85.
- [**breaking**] Renamed `AccelOdr::LpHz1620NormalHz5376` to `AccelOdr::LpHz5376NormalHz1344`
  to match the datasheet: 1.344 kHz in normal mode, 5.376 kHz in low-power mode.

## [0.1.4] - 2024-07-02

[0.1.4]: https://github.com/sunsided/lsm303dlhc-registers/releases/tag/v0.1.4

### Fixed

- Fixed missing writes on accelerometer reference register.

## [0.1.3] - 2024-07-02

[0.1.3]: https://github.com/sunsided/lsm303dlhc-registers/releases/tag/v0.1.3

### Internal

- Improved documentation on the `ControlRegister3A::i1drdy1` field (accelerometer data ready interrupt).

## [0.1.2] - 2024-07-02

[0.1.2]: https://github.com/sunsided/lsm303dlhc-registers/releases/tag/v0.1.2

### Fixed

- Applied some field defaults for `MR_REG_M`, `CRA_REG_M` and `CRB_REG_M`.

## [0.1.1] - 2024-07-01

[0.1.1]: https://github.com/sunsided/lsm303dlhc-registers/releases/tag/v0.1.1

### Changed

- Renames some fields and types to more accurately describe functionality.

## [0.1.0] - 2024-07-01

[0.1.0]: https://github.com/sunsided/lsm303dlhc-registers/releases/tag/v0.1.0

### Added

- Initial release providing the register types based on `bitfields` and `hardware-registers`.
