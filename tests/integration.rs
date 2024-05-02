use bmi160::{Data, MagnetometerData, Sensor3DData, Sensor3DDataScaled, SensorSelector};
mod common;
use crate::common::{destroy_i2c, destroy_spi, new_i2c, new_spi, Register, DEV_ADDR};
use embedded_hal_mock::eh1::i2c::Transaction as I2cTrans;

#[test]
fn can_create_and_destroy_i2c() {
    let imu = new_i2c(&[]);
    destroy_i2c(imu);
}

#[test]
fn can_create_and_destroy_spi() {
    let imu = new_spi(&[]);
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

mod get_sensor_data {
    use bmi160::DataScaled;

    use super::*;

    const EMPTY: Data = Data {
        accel: None,
        gyro: None,
        magnet: None,
        time: None,
    };

    const BUFFER: [u8; 23] = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
    ];

    #[test]
    fn empty() {
        let mut imu = new_i2c(&[]);
        let result = imu.data(SensorSelector::new()).unwrap();
        assert_eq!(result, EMPTY);
        destroy_i2c(imu);
    }

    #[test]
    fn all() {
        let mut imu = new_i2c(&[I2cTrans::write_read(
            DEV_ADDR,
            vec![Register::MAG],
            BUFFER.to_vec(),
        )]);
        let result = imu.data(SensorSelector::all()).unwrap();
        let expected = Data {
            magnet: Some(MagnetometerData {
                axes: Sensor3DData {
                    x: 0x0201,
                    y: 0x0403,
                    z: 0x0605,
                },
                hall_resistence: 0x0807,
            }),
            gyro: Some(Sensor3DData {
                x: 0x0A09,
                y: 0x0C0B,
                z: 0x0E0D,
            }),
            accel: Some(Sensor3DData {
                x: 0x100F,
                y: 0x1211,
                z: 0x1413,
            }),
            time: Some(0x171615),
        };
        assert_eq!(result, expected);
        destroy_i2c(imu);
    }

    #[test]
    fn accel_and_time() {
        let mut imu = new_i2c(&[I2cTrans::write_read(
            DEV_ADDR,
            vec![Register::ACC],
            BUFFER[14..].to_vec(),
        )]);
        let result = imu.data(SensorSelector::new().accel().time()).unwrap();
        let expected = Data {
            magnet: None,
            gyro: None,
            accel: Some(Sensor3DData {
                x: 0x100F,
                y: 0x1211,
                z: 0x1413,
            }),
            time: Some(0x171615),
        };
        assert_eq!(result, expected);
        destroy_i2c(imu);
    }

    #[test]
    fn only_gyro() {
        let mut imu = new_i2c(&[I2cTrans::write_read(
            DEV_ADDR,
            vec![Register::GYR],
            BUFFER[8..14].to_vec(),
        )]);
        let result = imu.data(SensorSelector::new().gyro()).unwrap();
        let expected = Data {
            magnet: None,
            gyro: Some(Sensor3DData {
                x: 0x0A09,
                y: 0x0C0B,
                z: 0x0E0D,
            }),
            accel: None,
            time: None,
        };
        assert_eq!(result, expected);
        destroy_i2c(imu);
    }

    #[test]
    fn all_scaled() {
        let mut imu = new_i2c(&[I2cTrans::write_read(
            DEV_ADDR,
            vec![Register::MAG],
            BUFFER.to_vec(),
        )]);
        let result = imu.data_scaled(SensorSelector::all()).unwrap();
        let expected = DataScaled {
            magnet: Some(MagnetometerData {
                axes: Sensor3DData {
                    x: 0x0201,
                    y: 0x0403,
                    z: 0x0605,
                },
                hall_resistence: 0x0807,
            }),
            gyro: Some(Sensor3DDataScaled {
                x: 0x0A09 as f32 * (1. / 16.4),
                y: 0x0C0B as f32 * (1. / 16.4),
                z: 0x0E0D as f32 * (1. / 16.4),
            }),
            accel: Some(Sensor3DDataScaled {
                x: 0x100F as f32 * (1. / 16384.),
                y: 0x1211 as f32 * (1. / 16384.),
                z: 0x1413 as f32 * (1. / 16384.),
            }),
            time: Some(0x171615),
        };
        assert_eq!(result, expected);
        destroy_i2c(imu);
    }
}
