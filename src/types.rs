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
