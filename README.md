# STMicroelectronics LSM303DLHC E-Compass Registers

[![Crates.io][crates-image]][crates-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
[![Safety Dance][safety-image]][safety-link]
![MSRV][msrv-image]
[![EUPL 1.2 licensed][license-eupl-image]][license-eupl-link]
[![Apache 2.0 licensed][license-apache-image]][license-apache-link]
[![MIT licensed][license-mit-image]][license-mit-link]

> A typed map of the LSM303DLHC's I²C registers.

Provides functions and named fields to read and modify the sensor registers. For example:

```rust
fn configure_sensor(sensor: &mut LSM303DLHC) {
    // configure the accelerometer to operate at 400 Hz
    sensor.write_register(
        ControlRegister1A::new()
            .with_output_data_rate(AccelOdr::Hz400)
            .with_low_power_enable(false)
            .with_x_enable(true)
            .with_y_enable(true)
            .with_z_enable(true),
    )?;

    // Reset other accelerometer control registers.
    sensor.write_register(ControlRegister2A::new())?;
    sensor.write_register(ControlRegister3A::new())?;
    sensor.write_register(ControlRegister4A::new())?;
    sensor.write_register(ControlRegister5A::new())?;
    sensor.write_register(ControlRegister6A::new())?;

    // Configure the magnetometer to operate in continuous mode.
    sensor.write_register(
        ModeRegisterM::new()
            .with_sleep_mode(false)
            .with_single_conversion(false),
    )?;

    // Enable the temperature sensor.
    sensor.write_register(
        CraRegisterM::new()
            .with_temp_en(true)
            .with_data_output_rate(MagOdr::Hz75),
    )?;
}
```

## Code of Conduct

We abide by the [Contributor Covenant][cc] and ask that you do as well.

## License

Copyright © 2024 Markus Mayer

Triple licensed under your choice of either of:

- European Union Public Licence, Version 1.2, ([LICENSE-EUPL](LICENSE-EUPL)
  or https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12)
- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

This work includes parts of Jorge Aparicio's [lsm303dlhc](https://github.com/japaric/lsm303dlhc) crate,
made available under MIT and Apache 2.0 licenses.


[crates-image]: https://img.shields.io/crates/v/lsm303dlhc-registers

[crates-link]: https://crates.io/crates/lsm303dlhc-registers

[docs-image]: https://docs.rs/lsm303dlhc-registers/badge.svg

[docs-link]: https://docs.rs/lsm303dlhc-registers/

[build-image]: https://github.com/sunsided/lsm303dlhc-registers/workflows/Rust/badge.svg

[build-link]: https://github.com/sunsided/lsm303dlhc-registers/actions

[safety-image]: https://img.shields.io/badge/unsafe-forbidden-success.svg

[safety-link]: https://github.com/rust-secure-code/safety-dance/

[msrv-image]: https://img.shields.io/badge/rustc-1.64+-blue.svg

[license-eupl-image]: https://img.shields.io/badge/license-EUPL_1.2-blue.svg

[license-apache-image]: https://img.shields.io/badge/license-Apache_2.0-blue.svg

[license-mit-image]: https://img.shields.io/badge/license-MIT-blue.svg

[license-apache-link]: https://github.com/sunsided/lsm303dlhc-registers/blob/develop/LICENSE-APACHE

[license-mit-link]: https://github.com/sunsided/lsm303dlhc-registers/blob/develop/LICENSE-MIT

[license-eupl-link]: https://github.com/sunsided/lsm303dlhc-registers/blob/develop/LICENSE-EUPL

[cc]: https://contributor-covenant.org
