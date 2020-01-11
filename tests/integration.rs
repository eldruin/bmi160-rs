mod common;
use crate::common::{destroy_i2c, destroy_spi, new_i2c, new_spi};
use embedded_hal_mock::pin::Mock as PinMock;

#[test]
fn can_create_and_destroy_i2c() {
    let imu = new_i2c(&[]);
    destroy_i2c(imu);
}

#[test]
fn can_create_and_destroy_spi() {
    let imu = new_spi(&[], PinMock::new(&[]));
    destroy_spi(imu);
}
