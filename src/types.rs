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
