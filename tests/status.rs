use bmi160::Status;
mod common;
use crate::common::{destroy_i2c, new_i2c, Register, DEV_ADDR};
use embedded_hal_mock::i2c::Transaction as I2cTrans;

macro_rules! get_st_test {
    ($name:ident, $st:expr, $drdy_acc:expr, $drdy_gyro:expr, $drdy_magnet:expr,
     $nvm_rdy:expr, $foc_rdy:expr, $mag_man:expr, $gyr_self_test:expr) => {
        #[test]
        fn $name() {
            let mut imu = new_i2c(&[I2cTrans::write_read(
                DEV_ADDR,
                vec![Register::STATUS],
                vec![$st],
            )]);
            let st = imu.status().unwrap();
            assert_eq!(
                Status {
                    accel_data_ready: $drdy_acc,
                    gyro_data_ready: $drdy_gyro,
                    magnet_data_ready: $drdy_magnet,
                    nvm_ready: $nvm_rdy,
                    foc_ready: $foc_rdy,
                    magnet_manual_op: $mag_man,
                    gyro_self_test_ok: $gyr_self_test
                },
                st
            );
            destroy_i2c(imu);
        }
    };
}

get_st_test!(
    accel_drdy,
    1 << 7,
    true,
    false,
    false,
    false,
    false,
    false,
    false
);

get_st_test!(
    gyro_drdy,
    1 << 6,
    false,
    true,
    false,
    false,
    false,
    false,
    false
);

get_st_test!(
    magnet_drdy,
    1 << 5,
    false,
    false,
    true,
    false,
    false,
    false,
    false
);

get_st_test!(
    nvm_rdy,
    1 << 4,
    false,
    false,
    false,
    true,
    false,
    false,
    false
);

get_st_test!(
    foc_rdy,
    1 << 3,
    false,
    false,
    false,
    false,
    true,
    false,
    false
);

get_st_test!(
    magnet_manual_op,
    1 << 2,
    false,
    false,
    false,
    false,
    false,
    true,
    false
);

get_st_test!(
    gyro_self_test_ok,
    1 << 1,
    false,
    false,
    false,
    false,
    false,
    false,
    true
);

get_st_test!(all, 0b1111_1110, true, true, true, true, true, true, true);
