# STMicroelectronics LSM303DLHC E-Compass Registers

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

[cc]: https://contributor-covenant.org
