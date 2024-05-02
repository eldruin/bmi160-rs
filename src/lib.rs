//! This is a platform agnostic Rust driver for the BMI160
//! inertial measurement unit using the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Get the latest sensor data. See: [`data()`].
//! - Set the accelerometer, gyroscope and magnetometer power mode. See: [`set_accel_power_mode()`].
//! - Get the sensor status. See: [`status()`].
//! - Get power mode. See: [`power_mode()`].
//! - Get chip ID. See: [`chip_id()`].
//!
//! [`data()`]: struct.Bmi160.html#method.data
//! [`set_accel_power_mode()`]: struct.Bmi160.html#method.set_accel_power_mode
//! [`status()`]: struct.Bmi160.html#method.status
//! [`power_mode()`]: struct.Bmi160.html#method.power_mode
//! [`chip_id()`]: struct.Bmi160.html#method.chip_id
//!
//! <!-- TODO
//! [Introductory blog post](TODO)
//! -->
//!
//! ## The devices
//!
//! The BMI160 is an inertial measurement unit (IMU) consisting of a
//! state-of-the-art 3-axis, low-g accelerometer and a low power 3-axis
//! gyroscope. It has been designed for low power, high precision 6-axis and
//! 9-axis applications in mobile phones, tablets, wearable devices, remote
//! controls, game controllers, head-mounted devices and toys.
//!
//! The BMI160 is available in a compact 14-pin 2.5 × 3.0 × 0.83 mm3 LGA
//! package. When accelerometer and gyroscope are in full operation mode, power
//! consumption is typically 925 μA, enabling always-on applications in
//! battery driven devices.
//!
//! Further Bosch Sensortec sensors, e.g. geomagnetic (BMM150) can be connected
//! as slave via a secondary I2C interface. In this configuration, the BMI160
//! controls the data acquisition of the external sensor and the synchronized
//! data of all sensors is stored the register data and can be additionally
//! stored in the built-in FIFO.
//!
//! Besides the flexible primary interface (I2C or SPI) that is used to connect
//! to the host, BMI160 provides an additional secondary interface. This
//! secondary interface can be used in SPI mode for OIS (optical image
//! stabilization) applications in conjunction with camera modules, or in
//! advanced gaming use cases.
//!
//! [Datasheet](https://www.bosch-sensortec.com/media/boschsensortec/downloads/datasheets/bst-bmi160-ds000.pdf)
//!
//! ## Usage examples (see also examples folder)
//!
//! To use this driver, import this crate and an `embedded_hal` implementation,
//! then create an instance of the driver either in I2C or SPI mode.
//!
//! Please find additional examples using hardware in this repository: [driver-examples]
//!
//! [driver-examples]: https://github.com/eldruin/driver-examples
//!
//! ### Create an instance of the driver in I2C mode and print the chip id
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! use bmi160::{Bmi160, SlaveAddr};
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let mut imu = Bmi160::new_with_i2c(dev, address);
//! let id = imu.chip_id().unwrap_or(0);
//! println!("Chip ID: {}", id);
//! # }
//! ```
//!
//! ### Create an instance of the driver in SPI mode and print the chip id
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! use bmi160::Bmi160;
//!
//! # fn main() {
//! let spi = hal::SpidevDevice::open("/dev/spidev0.0").unwrap();
//! let mut imu = Bmi160::new_with_spi(spi);
//! let id = imu.chip_id().unwrap_or(0);
//! println!("Chip ID: {}", id);
//! # }
//! ```
//!
//! ### Enable accelerometer and gyroscope and read data
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! use bmi160::{
//!     Bmi160, AccelerometerPowerMode, GyroscopePowerMode, SlaveAddr,
//!     SensorSelector
//! };
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let mut imu = Bmi160::new_with_i2c(dev, address);
//! imu.set_accel_power_mode(AccelerometerPowerMode::Normal).unwrap();
//! imu.set_gyro_power_mode(GyroscopePowerMode::Normal).unwrap();
//! loop {
//!     let data = imu.data(SensorSelector::new().accel().gyro()).unwrap();
//!     let accel = data.accel.unwrap();
//!     let gyro = data.gyro.unwrap();
//!     println!(
//!         "Accelerometer: x {:5} y {:5} z {:5}, \
//!          Gyroscope: x {:5} y {:5} z {:5}",
//!         accel.x, accel.y, accel.z, gyro.x, gyro.y, gyro.z);
//! }
//! # }
//! ```

#![deny(unsafe_code, missing_docs)]
#![no_std]

mod device_impl;
pub mod interface;
mod types;
pub use crate::interface::SlaveAddr;
pub use crate::types::{
    AccelerometerPowerMode, AccelerometerRange, Data, Error, GyroscopePowerMode, GyroscopeRange,
    MagnetometerData, MagnetometerPowerMode, Sensor3DData, SensorPowerMode, SensorSelector, Status,
};
mod register_address;
use crate::register_address::{BitFlags, Register};
mod read_sensor_data;

/// BMI160 device driver
#[derive(Debug)]
pub struct Bmi160<DI> {
    /// Digital interface: I2C or SPI
    iface: DI,
    accel_range: AccelerometerRange,
    gyro_range: GyroscopeRange,
}

mod private {
    use super::interface;
    pub trait Sealed {}

    impl<SPI> Sealed for interface::SpiInterface<SPI> {}
    impl<I2C> Sealed for interface::I2cInterface<I2C> {}
}
