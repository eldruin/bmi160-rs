use bmi160::scaling::{DataScaled, Sensor3DDataScaled};
use bmi160::{
    AccelerometerRange, Data, GyroscopeRange, MagnetometerData, Sensor3DData, SensorSelector,
};
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

mod get_sensor_data {
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
    }

    #[test]
    fn acc_and_gyr_scaled_default() {
        let mut imu = new_i2c(&[I2cTrans::write_read(
            DEV_ADDR,
            vec![Register::MAG],
            BUFFER.to_vec(),
        )]);
        let result = imu.scaled_data(SensorSelector::all()).unwrap();
        let expected = DataScaled {
            gyro: Some(Sensor3DDataScaled {
                x: 156.64635,
                y: 187.98781,
                z: 219.32927,
            }),
            accel: Some(Sensor3DDataScaled {
                x: 0.25091553,
                y: 0.2822876,
                z: 0.31365967,
            }),
            time: Some(0x171615),
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn acc_and_gyr_scaled_changed() {
        let mut imu = new_i2c(&[
            I2cTrans::write(DEV_ADDR, vec![Register::GYR_RANGE, 0b010]),
            I2cTrans::write(DEV_ADDR, vec![Register::ACC_RANGE, 0b0101]),
            I2cTrans::write_read(DEV_ADDR, vec![Register::MAG], BUFFER.to_vec()),
        ]);
        imu.set_gyro_range(GyroscopeRange::Range500s);
        imu.set_accel_range(AccelerometerRange::Range4g);
        let result = imu.scaled_data(SensorSelector::all()).unwrap();
        let expected = DataScaled {
            gyro: Some(Sensor3DDataScaled {
                x: 39.161587,
                y: 46.996952,
                z: 54.832317,
            }),
            accel: Some(Sensor3DDataScaled {
                x: 0.50183105,
                y: 0.5645752,
                z: 0.62731934,
            }),
            time: Some(0x171615),
        };
        assert_eq!(result, expected);
    }
}
