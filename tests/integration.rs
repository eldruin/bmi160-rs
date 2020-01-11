use bmi160::{AccelerometerPowerMode, GyroscopePowerMode, MagnetometerPowerMode, SensorPowerMode};
mod common;
use crate::common::{destroy_i2c, destroy_spi, new_i2c, new_spi, Register, DEV_ADDR};
use embedded_hal_mock::{i2c::Transaction as I2cTrans, pin::Mock as PinMock};

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

#[test]
fn can_get_chip_id() {
    let chip_id = 0b11010001;
    let mut imu = new_i2c(&[I2cTrans::write_read(
        DEV_ADDR,
        vec![Register::CHIPID],
        vec![chip_id],
    )]);
    let id = imu.chip_id().unwrap();
    assert_eq!(chip_id, id);
    destroy_i2c(imu);
}

macro_rules! get_pm_test {
    ($name:ident, $pmu:expr, $accel:ident, $gyro:ident, $magnet:ident) => {
        #[test]
        fn $name() {
            let mut imu = new_i2c(&[I2cTrans::write_read(
                DEV_ADDR,
                vec![Register::PMU_STATUS],
                vec![$pmu],
            )]);
            let pm = imu.power_mode().unwrap();
            assert_eq!(
                SensorPowerMode {
                    accel: AccelerometerPowerMode::$accel,
                    gyro: GyroscopePowerMode::$gyro,
                    magnet: MagnetometerPowerMode::$magnet,
                },
                pm
            );
            destroy_i2c(imu);
        }
    };
}

get_pm_test!(pm_all_normal, 0b0001_0101, Normal, Normal, Normal);
get_pm_test!(pm_all_suspend, 0, Suspend, Suspend, Suspend);
get_pm_test!(accel_lp, 0b0010_0101, LowPower, Normal, Normal);
get_pm_test!(gyro_fast_start, 0b0001_1101, Normal, FastStartUp, Normal);
get_pm_test!(magnet_lp, 0b0001_0110, Normal, Normal, LowPower);
