use bmi160::{AccelerometerRange, GyroscopeRange};
mod common;
use crate::common::{destroy_i2c, new_i2c, Register, DEV_ADDR};
use embedded_hal_mock::eh1::i2c::Transaction as I2cTrans;

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
set_accel_range_test!(set_accel_range_g2, G2, 0b0000_0011);
set_accel_range_test!(set_accel_range_g4, G4, 0b0000_0101);
set_accel_range_test!(set_accel_range_g8, G8, 0b0000_1000);

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
set_gyro_range_test!(set_gyro_range_2000, Scale2000, 0b0000_0000);
set_gyro_range_test!(set_gyro_range_1000, Scale1000, 0b0000_0001);
set_gyro_range_test!(set_gyro_range_500, Scale500, 0b0000_0010);
set_gyro_range_test!(set_gyro_range_250, Scale250, 0b0000_0011);
set_gyro_range_test!(set_gyro_range_125, Scale125, 0b0000_0100);
