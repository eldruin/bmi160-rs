use bmi160::{interface, Bmi160, SlaveAddr};
use embedded_hal_mock::{
    i2c::{Mock as I2cMock, Transaction as I2cTrans},
    pin::{Mock as PinMock, State as PinState, Transaction as PinTrans},
    spi::{Mock as SpiMock, Transaction as SpiTrans},
};

pub fn default_cs() -> PinMock {
    PinMock::new(&[PinTrans::set(PinState::Low), PinTrans::set(PinState::High)])
}
pub fn new_spi(
    transactions: &[SpiTrans],
    cs: PinMock,
) -> Bmi160<interface::SpiInterface<SpiMock, PinMock>> {
    Bmi160::new_with_spi(SpiMock::new(transactions), cs)
}

pub fn destroy_spi(imu: Bmi160<interface::SpiInterface<SpiMock, PinMock>>) {
    let (mut spi, mut cs) = imu.destroy();
    spi.done();
    cs.done();
}

pub fn new_i2c(transactions: &[I2cTrans]) -> Bmi160<interface::I2cInterface<I2cMock>> {
    Bmi160::new_with_i2c(I2cMock::new(transactions), SlaveAddr::default())
}

pub fn destroy_i2c(imu: Bmi160<interface::I2cInterface<I2cMock>>) {
    imu.destroy().done();
}
