# Rust BMI160 Inertial Measurement Unit Driver

[![crates.io](https://img.shields.io/crates/v/bmi160.svg)](https://crates.io/crates/bmi160)
[![Docs](https://docs.rs/bmi160/badge.svg)](https://docs.rs/bmi160)
![MSRV](https://img.shields.io/badge/rustc-1.62+-blue.svg)
[![Build Status](https://github.com/eldruin/bmi160-rs/workflows/Build/badge.svg)](https://github.com/eldruin/bmi160-rs/actions?query=workflow%3ABuild)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/bmi160-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/bmi160-rs?branch=master)

This is a platform agnostic Rust driver for the BMI160 small, low-power
inertial measurement unit using the [`embedded-hal`] traits.

This driver allows you to:
- Get the latest sensor data. See: `data()`.
- Set the accelerometer, gyroscope and magnetometer power mode. See: `set_accel_power_mode()`.
- Set the accelerometer and gyro range, See: `set_accel_range()` and `set_gyro_range()`.
- Get the sensor status. See: `status()`.
- Get power mode. See: `power_mode()`.
- Get chip ID. See: `chip_id()`.

<!-- TODO
[Introductory blog post]()
-->

The BMI160 is an inertial measurement unit (IMU) consisting of a
state-of-the-art 3-axis, low-g accelerometer and a low power 3-axis
gyroscope. It has been designed for low power, high precision 6-axis and
9-axis applications in mobile phones, tablets, wearable devices, remote
controls, game controllers, head-mounted devices and toys.

The BMI160 is available in a compact 14-pin 2.5 × 3.0 × 0.83 mm3 LGA
package. When accelerometer and gyroscope are in full operation mode, power
consumption is typically 925 μA, enabling always-on applications in
battery driven devices.

Further Bosch Sensortec sensors, e.g. geomagnetic (BMM150) can be connected
as slave via a secondary I2C interface. In this configuration, the BMI160
controls the data acquisition of the external sensor and the synchronized
data of all sensors is stored the register data and can be additionally
stored in the built-in FIFO.

Besides the flexible primary interface (I2C or SPI) that is used to connect
to the host, BMI160 provides an additional secondary interface. This
secondary interface can be used in SPI mode for OIS (optical image
stabilization) applications in conjunction with camera modules, or in
advanced gaming use cases.

[Datasheet](https://www.bosch-sensortec.com/media/boschsensortec/downloads/datasheets/bst-bmi160-ds000.pdf)

## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the device.

Please find additional examples using hardware in this repository: [driver-examples]

[driver-examples]: https://github.com/eldruin/driver-examples

```rust
extern crate linux_embedded_hal as hal;
use bmi160::{AccelerometerPowerMode, Bmi160, GyroscopePowerMode, SensorSelector, SlaveAddr};

fn main() {
    let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
    let address = SlaveAddr::default();
    let mut imu = Bmi160::new_with_i2c(dev, address);
    imu.set_accel_power_mode(AccelerometerPowerMode::Normal)
        .unwrap();
    imu.set_gyro_power_mode(GyroscopePowerMode::Normal).unwrap();
    loop {
        let data = imu.data(SensorSelector::new().accel().gyro()).unwrap();
        let accel = data.accel.unwrap();
        let gyro = data.gyro.unwrap();
        println!(
            "Accelerometer: x {:5} y {:5} z {:5}, \
             Gyroscope: x {:5} y {:5} z {:5}",
            accel.x, accel.y, accel.z, gyro.x, gyro.y, gyro.z
        );
    }
}
```

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.62 and up. It *might*
compile with older versions but that may change in any new patch release.

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/eldruin/bmi160-rs/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
