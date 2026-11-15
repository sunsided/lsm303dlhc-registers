# Changelog

All notable changes to this project will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `ReservedBitPattern` error type. `ControlRegister1A` and `ConfigurationBRegisterM` implement
  `TryFromBits<u8>` and reject reserved output data rate (`1010`–`1111`) and gain (`000`) patterns.
- `AccelOdr::try_from_bits` and `MagGain::try_from_bits`.
- `Debug` for the `accel::RegisterAddress` and `mag::RegisterAddress` enums.

### Changed

- [**breaking**] Updated to `hardware-registers` 0.4. `Register` now requires
  `ReadableI2CRegister8` and `TryFromBits<u8>`, as 0.4 no longer implies readability.
  The explicit `TryFromBits<u8>` bound lets generic drivers call `R::try_from_bits`.
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
