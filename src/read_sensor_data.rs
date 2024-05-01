use crate::{
    interface::{ReadData, WriteData},
    types::{DataScaled, Sensor3DDataScaled},
    Bmi160, Data, Error, MagnetometerData, Register, Sensor3DData, SensorSelector,
};

impl<DI, CommE> Bmi160<DI>
where
    DI: ReadData<Error = Error<CommE>> + WriteData<Error = Error<CommE>>,
{
    /// Read latest sensor data
    pub fn data(&mut self, selector: SensorSelector) -> Result<Data, Error<CommE>> {
        let result = if selector != SensorSelector::new() {
            let (begin, end) = get_data_addresses(selector);
            let mut data = [0_u8; 24];
            data[0] = begin;
            let len = (1 + end - begin) as usize;
            self.iface.read_data(&mut data[0..len])?;
            get_data(selector, &data[1..], (begin - Register::MAG) as usize)
        } else {
            Data {
                accel: None,
                gyro: None,
                magnet: None,
                time: None,
            }
        };
        Ok(result)
    }

    /// Read latest sensor data and scale it using the gyroscope and accelerometer ranges
    pub fn data_scaled(&mut self, selector: SensorSelector) -> Result<DataScaled, Error<CommE>> {
        let raw_data = self.data(selector)?;

        let accel_multiplier = self.accel_range.multiplier();
        let gyro_multiplier = self.gyro_range.multiplier();

        Ok(DataScaled {
            accel: raw_data.accel.map(|d| Sensor3DDataScaled {
                x: d.x as f32 * accel_multiplier,
                y: d.y as f32 * accel_multiplier,
                z: d.z as f32 * accel_multiplier,
            }),
            gyro: raw_data.gyro.map(|d| Sensor3DDataScaled {
                x: d.x as f32 * gyro_multiplier,
                y: d.y as f32 * gyro_multiplier,
                z: d.z as f32 * gyro_multiplier,
            }),
            magnet: raw_data.magnet,
            time: raw_data.time,
        })
    }
}

fn get_data(selector: SensorSelector, data: &[u8], data_offset: usize) -> Data {
    let mut result = Data {
        accel: None,
        gyro: None,
        magnet: None,
        time: None,
    };
    if selector.magnet {
        result.magnet = Some(MagnetometerData {
            axes: get_sensor3d_data(&data[0..6]),
            hall_resistence: (u16::from(data[6]) | (u16::from(data[7]) << 8)),
        });
    }
    if selector.gyro {
        result.gyro = Some(get_sensor3d_data(&data[8 - data_offset..14 - data_offset]));
    }
    if selector.accel {
        result.accel = Some(get_sensor3d_data(&data[14 - data_offset..20 - data_offset]));
    }
    if selector.time {
        result.time = Some(
            u32::from(data[20 - data_offset])
                | (u32::from(data[21 - data_offset]) << 8)
                | (u32::from(data[22 - data_offset]) << 16),
        );
    }
    result
}

fn get_sensor3d_data(data: &[u8]) -> Sensor3DData {
    Sensor3DData {
        x: (u16::from(data[0]) | (u16::from(data[1]) << 8)) as i16,
        y: (u16::from(data[2]) | (u16::from(data[3]) << 8)) as i16,
        z: (u16::from(data[4]) | (u16::from(data[5]) << 8)) as i16,
    }
}

fn get_data_addresses(selector: SensorSelector) -> (u8, u8) {
    let begin = if selector.magnet {
        Register::MAG
    } else if selector.gyro {
        Register::GYR
    } else if selector.accel {
        Register::ACC
    } else if selector.time {
        Register::SENSORTIME
    } else {
        0
    };

    let end = if selector.time {
        Register::SENSORTIME + 3
    } else if selector.accel {
        Register::ACC + 6
    } else if selector.gyro {
        Register::GYR + 6
    } else if selector.magnet {
        Register::MAG + 8
    } else {
        0
    };

    (begin, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod data_addresses {
        use super::*;
        #[test]
        fn all() {
            let result = get_data_addresses(SensorSelector::all());
            assert_eq!(result, (Register::MAG, Register::SENSORTIME + 3));
        }

        #[test]
        fn none() {
            let result = get_data_addresses(SensorSelector::new());
            assert_eq!(result, (0, 0));
        }

        #[test]
        fn only_accel() {
            let result = get_data_addresses(SensorSelector::new().accel());
            assert_eq!(result, (Register::ACC, Register::ACC + 6));
        }

        #[test]
        fn only_gyro() {
            let result = get_data_addresses(SensorSelector::new().gyro());
            assert_eq!(result, (Register::GYR, Register::GYR + 6));
        }

        #[test]
        fn only_magnet() {
            let result = get_data_addresses(SensorSelector::new().magnet());
            assert_eq!(result, (Register::MAG, Register::MAG + 8));
        }

        #[test]
        fn accel_and_time() {
            let result = get_data_addresses(SensorSelector::new().accel().time());
            assert_eq!(result, (Register::ACC, Register::SENSORTIME + 3));
        }

        #[test]
        fn gyro_and_time() {
            let result = get_data_addresses(SensorSelector::new().gyro().time());
            assert_eq!(result, (Register::GYR, Register::SENSORTIME + 3));
        }
    }

    mod sensor3d_data {
        use super::*;

        #[test]
        fn can_decode_positive_array() {
            let result = get_sensor3d_data(&[0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
            assert_eq!(
                result,
                Sensor3DData {
                    x: 0x0201,
                    y: 0x0403,
                    z: 0x0605
                }
            );
        }

        #[test]
        fn can_decode_negative_array() {
            let result = get_sensor3d_data(&[0x0B, 0x86, 0x0B, 0x86, 0x0B, 0x86]);
            assert_eq!(
                result,
                Sensor3DData {
                    x: -31221,
                    y: -31221,
                    z: -31221
                }
            );
        }
    }
}
