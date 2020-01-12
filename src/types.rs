/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<CommE, PinE> {
    /// I²C / SPI communication error
    Comm(CommE),
    /// Chip-select pin error (SPI)
    Pin(PinE),
    /// Invalid input data provided
    InvalidInputData,
}

/// Sensor power mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SensorPowerMode {
    /// Accelerometer power mode
    pub accel: AccelerometerPowerMode,
    /// Gyroscope power mode
    pub gyro: GyroscopePowerMode,
    /// Magnetometer power mode
    pub magnet: MagnetometerPowerMode,
}

/// Accelerometer power mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AccelerometerPowerMode {
    /// Normal mode
    Normal,
    /// Suspend mode
    Suspend,
    /// Low power mode
    LowPower,
}

/// Gyroscope power mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GyroscopePowerMode {
    /// Normal mode
    Normal,
    /// Suspend mode
    Suspend,
    /// Fast start-up mode
    FastStartUp,
}

/// Magnetometer power mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MagnetometerPowerMode {
    /// Normal mode
    Normal,
    /// Suspend mode
    Suspend,
    /// Low power mode
    LowPower,
}

/// Sensor status flags
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Status {
    /// Accelerometer has data ready
    pub accel_data_ready: bool,
    /// Gyroscope has data ready
    pub gyro_data_ready: bool,
    /// Magnetometer has data ready
    pub magnet_data_ready: bool,
    /// NVM controller ready
    pub nvm_ready: bool,
    /// Fast offset compensation (FOC) completed
    pub foc_ready: bool,
    /// Manual magnetometer operation
    pub magnet_manual_op: bool,
    /// Gyroscope self-test completed successfully
    pub gyro_self_test_ok: bool,
}

/// Sensor data read selector
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SensorSelector {
    pub(crate) accel: bool,
    pub(crate) gyro: bool,
    pub(crate) magnet: bool,
    pub(crate) time: bool,
}

impl SensorSelector {
    /// Create new instance of the selector.
    ///
    /// This does not include any data.
    pub fn new() -> Self {
        SensorSelector {
            accel: false,
            gyro: false,
            magnet: false,
            time: false,
        }
    }

    /// Include acceleration sensor data
    pub fn accel(mut self) -> Self {
        self.accel = true;
        self
    }

    /// Include gyroscope sensor data
    pub fn gyro(mut self) -> Self {
        self.gyro = true;
        self
    }

    /// Include magnetometer sensor data
    pub fn magnet(mut self) -> Self {
        self.magnet = true;
        self
    }

    /// Include sensor time
    pub fn time(mut self) -> Self {
        self.time = true;
        self
    }

    /// Include accelerometer, gyroscope, magnetometer and time data
    pub fn all() -> Self {
        SensorSelector {
            accel: true,
            gyro: true,
            magnet: true,
            time: true,
        }
    }
}

impl Default for SensorSelector {
    fn default() -> Self {
        SensorSelector::all()
    }
}

/// Sensor data read selector
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sensor3DData {
    /// X axis data
    pub x: i16,
    /// Y axis data
    pub y: i16,
    /// Z axis data
    pub z: i16,
}

/// Magnetometer data
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MagnetometerData {
    /// Axes data
    pub axes: Sensor3DData,
    /// Hall resistence data
    pub hall_resistence: u16,
}

/// Sensor data read
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Data {
    /// Accelerometer data (if selected)
    pub accel: Option<Sensor3DData>,
    /// Gyroscope data (if selected)
    pub gyro: Option<Sensor3DData>,
    /// Magnetometer data (if selected)
    pub magnet: Option<MagnetometerData>,
    /// Time data (if selected)
    pub time: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selector_default_all() {
        assert_eq!(SensorSelector::all(), SensorSelector::default());
    }

    #[test]
    fn selector_all() {
        assert_eq!(
            SensorSelector::all(),
            SensorSelector {
                accel: true,
                gyro: true,
                magnet: true,
                time: true
            }
        );
    }

    macro_rules! selector_test {
        ($name:ident, $method:ident) => {
            #[test]
            fn $name() {
                let mut expected = SensorSelector {
                    accel: false,
                    gyro: false,
                    magnet: false,
                    time: false,
                };
                expected.$method = true;
                assert_eq!(SensorSelector::new().$method(), expected);
            }
        };
    }
    selector_test!(selector_accel, accel);
    selector_test!(selector_gyro, gyro);
    selector_test!(selector_magnet, magnet);
    selector_test!(selector_time, time);

    #[test]
    fn selector_combine_all() {
        assert_eq!(
            SensorSelector::all(),
            SensorSelector::new().accel().gyro().magnet().time()
        );
    }
}
