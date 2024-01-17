use bmi160::{AccelerometerPowerMode, GyroscopePowerMode, MagnetometerPowerMode, SensorPowerMode};
mod common;
use crate::common::{destroy_i2c, new_i2c, Register, DEV_ADDR};
use embedded_hal_mock::eh1::i2c::Transaction as I2cTrans;

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

mod get_pm {
    use super::*;
    get_pm_test!(pm_all_normal, 0b0001_0101, Normal, Normal, Normal);
    get_pm_test!(pm_all_suspend, 0, Suspend, Suspend, Suspend);
    get_pm_test!(accel_lp, 0b0010_0101, LowPower, Normal, Normal);
    get_pm_test!(gyro_fast_start, 0b0001_1101, Normal, FastStartUp, Normal);
    get_pm_test!(magnet_lp, 0b0001_0110, Normal, Normal, LowPower);
}

macro_rules! set_pm_test {
    ($name:ident, $method:ident, $st:ident::$variant:ident, $cmd:expr) => {
        #[test]
        fn $name() {
            let mut imu = new_i2c(&[I2cTrans::write(DEV_ADDR, vec![Register::CMD, $cmd])]);
            imu.$method($st::$variant).unwrap();
            destroy_i2c(imu);
        }
    };
}

macro_rules! set_accel_pm_test {
    ($name:ident, $variant:ident, $cmd:expr) => {
        set_pm_test!(
            $name,
            set_accel_power_mode,
            AccelerometerPowerMode::$variant,
            $cmd
        );
    };
}
set_accel_pm_test!(set_accel_pm_susp, Suspend, 0b0001_0000);
set_accel_pm_test!(set_accel_pm_norm, Normal, 0b0001_0001);
set_accel_pm_test!(set_accel_pm_lowp, LowPower, 0b0001_0010);

macro_rules! set_gyro_pm_test {
    ($name:ident, $variant:ident, $cmd:expr) => {
        set_pm_test!(
            $name,
            set_gyro_power_mode,
            GyroscopePowerMode::$variant,
            $cmd
        );
    };
}
set_gyro_pm_test!(set_gyro_pm_susp, Suspend, 0b0001_0100);
set_gyro_pm_test!(set_gyro_pm_norm, Normal, 0b0001_0101);
set_gyro_pm_test!(set_gyro_pm_lowp, FastStartUp, 0b0001_0111);

macro_rules! set_magnet_pm_test {
    ($name:ident, $variant:ident, $cmd:expr) => {
        set_pm_test!(
            $name,
            set_magnet_power_mode,
            MagnetometerPowerMode::$variant,
            $cmd
        );
    };
}
set_magnet_pm_test!(set_magnet_pm_susp, Suspend, 0b0001_1000);
set_magnet_pm_test!(set_magnet_pm_norm, Normal, 0b0001_1001);
set_magnet_pm_test!(set_magnet_pm_lowp, LowPower, 0b0001_1010);
