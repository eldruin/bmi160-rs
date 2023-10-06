use bmi160::{AccelerometerRange, GyroscopeRange};
mod common;
use crate::common::{destroy_i2c, new_i2c, Register, DEV_ADDR};
use embedded_hal_mock::i2c::Transaction as I2cTrans;

macro_rules! set_range_test {
    ($name:ident, $method:ident, $st:ident::$variant:ident, $cmd:expr, $reg:ident) => {
        #[test]
        fn $name() {
            let mut imu = new_i2c(&[I2cTrans::write(DEV_ADDR, vec![Register::$reg, $cmd])]);
            imu.$method($st::$variant).unwrap();
            destroy_i2c(imu);
        }
    };
}

macro_rules! set_accel_range_test {
    ($name:ident, $variant:ident, $cmd:expr) => {
        set_range_test!(
            $name,
            set_accel_range,
            AccelerometerRange::$variant,
            $cmd,
            ACC_RANGE
        );
    };
}
set_accel_range_test!(set_accel_range_2g, Range2g, 0b0011);
set_accel_range_test!(set_accel_range_4g, Range4g, 0b0101);
set_accel_range_test!(set_accel_range_8g, Range8g, 0b1000);

macro_rules! set_gyro_range_test {
    ($name:ident, $variant:ident, $cmd:expr) => {
        set_range_test!(
            $name,
            set_gyro_range,
            GyroscopeRange::$variant,
            $cmd,
            GYR_RANGE
        );
    };
}
set_gyro_range_test!(set_gyro_range_125s, Range125s, 0b100);
set_gyro_range_test!(set_gyro_range_250s, Range250s, 0b011);
set_gyro_range_test!(set_gyro_range_500s, Range500s, 0b010);
set_gyro_range_test!(set_gyro_range_1000s, Range1000s, 0b001);
set_gyro_range_test!(set_gyro_range_2000s, Range2000s, 0b000);
