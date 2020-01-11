use crate::{
    interface::{I2cInterface, ReadData, SpiInterface, WriteData},
    AccelerometerPowerMode, Bmi160, Error, GyroscopePowerMode, MagnetometerPowerMode, Register,
    SensorPowerMode, SlaveAddr,
};

impl<I2C> Bmi160<I2cInterface<I2C>> {
    /// Create new instance of the BMI160 device communicating through I2C.
    pub fn new_with_i2c(i2c: I2C, address: SlaveAddr) -> Self {
        Bmi160 {
            iface: I2cInterface {
                i2c,
                address: address.into(),
            },
        }
    }

    /// Destroy driver instance, return I2C bus.
    pub fn destroy(self) -> I2C {
        self.iface.i2c
    }
}

impl<SPI, CS> Bmi160<SpiInterface<SPI, CS>> {
    /// Create new instance of the BMI160 device communicating through SPI.
    pub fn new_with_spi(spi: SPI, chip_select: CS) -> Self {
        Bmi160 {
            iface: SpiInterface {
                spi,
                cs: chip_select,
            },
        }
    }

    /// Destroy driver instance, return SPI bus instance and chip select pin.
    pub fn destroy(self) -> (SPI, CS) {
        (self.iface.spi, self.iface.cs)
    }
}

impl<DI, CommE, PinE> Bmi160<DI>
where
    DI: ReadData<Error = Error<CommE, PinE>> + WriteData<Error = Error<CommE, PinE>>,
{
    /// Get chip ID
    pub fn chip_id(&mut self) -> Result<u8, Error<CommE, PinE>> {
        self.iface.read_register(Register::CHIPID)
    }

    /// Get sensor power mode
    pub fn power_mode(&mut self) -> Result<SensorPowerMode, Error<CommE, PinE>> {
        let status = self.iface.read_register(Register::PMU_STATUS)?;
        let accel = match status & (0b11 << 4) {
            0 => AccelerometerPowerMode::Suspend,
            0b10_0000 => AccelerometerPowerMode::LowPower,
            _ => AccelerometerPowerMode::Normal,
        };
        let magnet = match status & 0b11 {
            0 => MagnetometerPowerMode::Suspend,
            2 => MagnetometerPowerMode::LowPower,
            _ => MagnetometerPowerMode::Normal,
        };
        let gyro = match status & (0b11 << 2) {
            0 => GyroscopePowerMode::Suspend,
            0b1100 => GyroscopePowerMode::FastStartUp,
            _ => GyroscopePowerMode::Normal,
        };
        Ok(SensorPowerMode {
            accel,
            gyro,
            magnet,
        })
    }
}
