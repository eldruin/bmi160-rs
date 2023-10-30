use crate::{
    interface::{ReadData, WriteData},
    Bmi160, Data, Error, MagnetometerData, Register, Sensor3DData, SensorSelector,
};

#[cfg(feature = "scaled_data")]
use crate::scaling::{DataScaled, Sensor3DDataScaled};

impl<DI, CommE, PinE> Bmi160<DI>
where
    DI: ReadData<Error = Error<CommE, PinE>> + WriteData<Error = Error<CommE, PinE>>,
{
    /// Read latest sensor data
    pub fn data(&mut self, selector: SensorSelector) -> Result<Data, Error<CommE, PinE>> {
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
}

#[cfg(feature = "scaled_data")]
impl<DI, CommE, PinE> Bmi160<DI>
where
    DI: ReadData<Error = Error<CommE, PinE>> + WriteData<Error = Error<CommE, PinE>>,
{
    /// Read latest sensor data
    pub fn scaled_data(
        &mut self,
        selector: SensorSelector,
    ) -> Result<DataScaled, Error<CommE, PinE>> {
        let accel_div: f32 = match self.accel_range {
            crate::AccelerometerRange::Range2g => 16384.0f32,
            crate::AccelerometerRange::Range4g => 8192.0f32,
            crate::AccelerometerRange::Range8g => 4096.0f32,
        };

        let gyro_div: f32 = match self.gyro_range {
            crate::GyroscopeRange::Range2000s => 16.4,
            crate::GyroscopeRange::Range1000s => 32.8,
            crate::GyroscopeRange::Range500s => 65.6,
            crate::GyroscopeRange::Range250s => 131.2,
            crate::GyroscopeRange::Range125s => 262.4,
        };

        let result = if selector != SensorSelector::new() {
            let (begin, end) = get_data_addresses(selector);
            let mut data = [0_u8; 24];
            data[0] = begin;
            let len = (1 + end - begin) as usize;
            self.iface.read_data(&mut data[0..len])?;
            let unscaled_data = get_data(selector, &data[1..], (begin - Register::MAG) as usize);
            DataScaled {
                accel: unscaled_data.accel.and_then(|unscaled_accel| {
                    Some(Sensor3DDataScaled {
                        x: unscaled_accel.x as f32 / accel_div,
                        y: unscaled_accel.y as f32 / accel_div,
                        z: unscaled_accel.z as f32 / accel_div,
                    })
                }),
                gyro: unscaled_data.gyro.and_then(|unscaled_gyro| {
                    Some(Sensor3DDataScaled {
                        x: unscaled_gyro.x as f32 / gyro_div,
                        y: unscaled_gyro.y as f32 / gyro_div,
                        z: unscaled_gyro.z as f32 / gyro_div,
                    })
                }),
                time: unscaled_data.time,
            }
        } else {
            DataScaled {
                accel: None,
                gyro: None,
                time: None,
            }
        };
        Ok(result)
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
