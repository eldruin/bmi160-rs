//! I2C/SPI interfaces

use crate::{private, Error};
use embedded_hal::{
    blocking::{i2c, spi},
    digital,
};

const I2C_DEV_BASE_ADDR: u8 = 0x68;

/// I2C interface
#[derive(Debug)]
pub struct I2cInterface<I2C> {
    pub(crate) i2c: I2C,
    pub(crate) address: u8,
}

/// SPI interface
#[derive(Debug)]
pub struct SpiInterface<SPI, CS> {
    pub(crate) spi: SPI,
    pub(crate) cs: CS,
}

/// Possible slave addresses
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SlaveAddr {
    /// Default slave address
    Default,
    /// Alternative slave address providing bit value for the SDO pin
    Alternative(bool),
}

impl Default for SlaveAddr {
    /// Default slave address
    fn default() -> Self {
        SlaveAddr::Default
    }
}

impl SlaveAddr {
    pub(crate) fn addr(self) -> u8 {
        match self {
            SlaveAddr::Default => I2C_DEV_BASE_ADDR,
            SlaveAddr::Alternative(false) => I2C_DEV_BASE_ADDR,
            SlaveAddr::Alternative(true) => I2C_DEV_BASE_ADDR | 1,
        }
    }
}

/// Write data
pub trait WriteData: private::Sealed {
    /// Error type
    type Error;
    /// Write to an u8 register
    fn write_register(&mut self, register: u8, data: u8) -> Result<(), Self::Error>;
    /// Write data. The first element corresponds to the starting address.
    fn write_data(&mut self, payload: &mut [u8]) -> Result<(), Self::Error>;
}

impl<I2C, E> WriteData for I2cInterface<I2C>
where
    I2C: i2c::Write<Error = E>,
{
    type Error = Error<E, ()>;
    fn write_register(&mut self, register: u8, data: u8) -> Result<(), Self::Error> {
        let payload: [u8; 2] = [register, data];
        let addr = self.address;
        self.i2c.write(addr, &payload).map_err(Error::Comm)
    }

    fn write_data(&mut self, payload: &mut [u8]) -> Result<(), Self::Error> {
        let addr = self.address;
        self.i2c.write(addr, &payload).map_err(Error::Comm)
    }
}

impl<SPI, CS, CommE, PinE> WriteData for SpiInterface<SPI, CS>
where
    SPI: spi::Write<u8, Error = CommE>,
    CS: digital::v2::OutputPin<Error = PinE>,
{
    type Error = Error<CommE, PinE>;
    fn write_register(&mut self, register: u8, data: u8) -> Result<(), Self::Error> {
        self.cs.set_low().map_err(Error::Pin)?;

        let payload: [u8; 2] = [register + 0x80, data];
        let result = self.spi.write(&payload).map_err(Error::Comm);

        self.cs.set_high().map_err(Error::Pin)?;
        result
    }

    fn write_data(&mut self, payload: &mut [u8]) -> Result<(), Self::Error> {
        self.cs.set_low().map_err(Error::Pin)?;
        payload[0] += 0x80;
        let result = self.spi.write(&payload).map_err(Error::Comm);

        self.cs.set_high().map_err(Error::Pin)?;
        result
    }
}

/// Read data
pub trait ReadData: private::Sealed {
    /// Error type
    type Error;
    /// Read an u8 register
    fn read_register(&mut self, register: u8) -> Result<u8, Self::Error>;
    /// Read some data. The first element corresponds to the starting address.
    fn read_data(&mut self, payload: &mut [u8]) -> Result<(), Self::Error>;
}

impl<I2C, E> ReadData for I2cInterface<I2C>
where
    I2C: i2c::WriteRead<Error = E>,
{
    type Error = Error<E, ()>;
    fn read_register(&mut self, register: u8) -> Result<u8, Self::Error> {
        let mut data = [0];
        let addr = self.address;
        self.i2c
            .write_read(addr, &[register], &mut data)
            .map_err(Error::Comm)
            .and(Ok(data[0]))
    }

    fn read_data(&mut self, payload: &mut [u8]) -> Result<(), Self::Error> {
        let len = payload.len();
        let addr = self.address;
        self.i2c
            .write_read(addr, &[payload[0]], &mut payload[1..len])
            .map_err(Error::Comm)
    }
}

impl<SPI, CS, CommE, PinE> ReadData for SpiInterface<SPI, CS>
where
    SPI: spi::Transfer<u8, Error = CommE>,
    CS: digital::v2::OutputPin<Error = PinE>,
{
    type Error = Error<CommE, PinE>;
    fn read_register(&mut self, register: u8) -> Result<u8, Self::Error> {
        self.cs.set_low().map_err(Error::Pin)?;
        let mut data = [register, 0];
        let result = self.spi.transfer(&mut data).map_err(Error::Comm);
        self.cs.set_high().map_err(Error::Pin)?;
        Ok(result?[1])
    }

    fn read_data(&mut self, mut payload: &mut [u8]) -> Result<(), Self::Error> {
        self.cs.set_low().map_err(Error::Pin)?;
        let result = self.spi.transfer(&mut payload).map_err(Error::Comm);
        self.cs.set_high().map_err(Error::Pin)?;
        result?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::SlaveAddr;
    use super::I2C_DEV_BASE_ADDR as ADDR;

    #[test]
    fn can_get_default_address() {
        let addr = SlaveAddr::default();
        assert_eq!(ADDR, addr.addr());
    }

    #[test]
    fn can_generate_alternative_addresses() {
        assert_eq!(ADDR, SlaveAddr::Alternative(false).addr());
        assert_eq!(ADDR | 1, SlaveAddr::Alternative(true).addr());
    }
}
