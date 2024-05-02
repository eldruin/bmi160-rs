use bmi160::{interface, Bmi160, SlaveAddr};
use embedded_hal_mock::eh1::{
    i2c::{Mock as I2cMock, Transaction as I2cTrans},
    spi::{Mock as SpiMock, Transaction as SpiTrans},
};

pub struct Register;
#[allow(unused)]
impl Register {
    pub const CHIPID: u8 = 0x00;
    pub const PMU_STATUS: u8 = 0x03;
    pub const MAG: u8 = 0x04;
    pub const GYR: u8 = 0x0C;
    pub const ACC: u8 = 0x12;
    pub const STATUS: u8 = 0x1B;
    pub const CMD: u8 = 0x7E;
    pub const ACC_RANGE: u8 = 0x41;
    pub const GYR_RANGE: u8 = 0x43;
}

pub const DEV_ADDR: u8 = 0x68;

#[allow(unused)]
pub fn new_spi(transactions: &[SpiTrans<u8>]) -> Bmi160<interface::SpiInterface<SpiMock<u8>>> {
    Bmi160::new_with_spi(SpiMock::new(transactions))
}

#[allow(unused)]
pub fn destroy_spi(imu: Bmi160<interface::SpiInterface<SpiMock<u8>>>) {
    let mut spi = imu.destroy();
    spi.done();
}

#[allow(unused)]
pub fn new_i2c(transactions: &[I2cTrans]) -> Bmi160<interface::I2cInterface<I2cMock>> {
    Bmi160::new_with_i2c(I2cMock::new(transactions), SlaveAddr::default())
}

#[allow(unused)]
pub fn destroy_i2c(imu: Bmi160<interface::I2cInterface<I2cMock>>) {
    imu.destroy().done();
}
