//! This is a platform agnostic Rust driver for the BMI160
//! inertial measurement unit using the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! <!--TODO
//! This driver allows you to:
//! -->
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

#![deny(unsafe_code, missing_docs)]
#![no_std]

mod device_impl;
pub mod interface;
mod types;
pub use crate::types::{
    AccelerometerPowerMode, Error, GyroscopePowerMode, MagnetometerPowerMode, SensorPowerMode,
};
pub use interface::SlaveAddr;
mod register_address;
use register_address::Register;

/// BMI160 device driver
#[derive(Debug)]
pub struct Bmi160<DI> {
    /// Digital interface: I2C or SPI
    iface: DI,
}

mod private {
    use super::interface;
    pub trait Sealed {}

    impl<SPI, CS> Sealed for interface::SpiInterface<SPI, CS> {}
    impl<I2C> Sealed for interface::I2cInterface<I2C> {}
}
