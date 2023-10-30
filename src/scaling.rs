//! Scaled data structures

/// Floating point 3D data
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sensor3DDataScaled {
    /// X axis data
    pub x: f32,
    /// Y axis data
    pub y: f32,
    /// Z axis data
    pub z: f32,
}

/// Sensor data read
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DataScaled {
    /// Accelerometer data (if selected)
    pub accel: Option<Sensor3DDataScaled>,
    /// Gyroscope data (if selected)
    pub gyro: Option<Sensor3DDataScaled>,
    /// Time data (if selected)
    pub time: Option<u32>,
}
