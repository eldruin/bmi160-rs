//! I2C/SPI interfaces

use crate::{private, Error};
use embedded_hal::{i2c, spi::SpiDevice};

const I2C_DEV_BASE_ADDR: u8 = 0x68;

/// I2C interface
#[derive(Debug)]
pub struct I2cInterface<I2C> {
    pub(crate) i2c: I2C,
    pub(crate) address: u8,
}

/// SPI interface
#[derive(Debug)]
pub struct SpiInterface<SPI> {
    pub(crate) spi: SPI,
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
    /// The numerical address of the device
    pub fn addr(self) -> u8 {
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
    I2C: i2c::I2c<Error = E>,
{
    type Error = Error<E>;
    fn write_register(&mut self, register: u8, data: u8) -> Result<(), Self::Error> {
        let payload: [u8; 2] = [register, data];
        let addr = self.address;
        self.i2c.write(addr, &payload).map_err(Error::Comm)
    }

    fn write_data(&mut self, payload: &mut [u8]) -> Result<(), Self::Error> {
        let addr = self.address;
        self.i2c.write(addr, payload).map_err(Error::Comm)
    }
}

impl<SPI, CommE> WriteData for SpiInterface<SPI>
where
    SPI: SpiDevice<u8, Error = CommE>,
{
    type Error = Error<CommE>;
    fn write_register(&mut self, register: u8, data: u8) -> Result<(), Self::Error> {
        let payload: [u8; 2] = [register, data];
        self.spi.write(&payload).map_err(Error::Comm)
    }

    fn write_data(&mut self, payload: &mut [u8]) -> Result<(), Self::Error> {
        self.spi.write(payload).map_err(Error::Comm)
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
    I2C: i2c::I2c<Error = E>,
{
    type Error = Error<E>;
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

impl<SPI, CommE> ReadData for SpiInterface<SPI>
where
    SPI: SpiDevice<u8, Error = CommE>,
{
    type Error = Error<CommE>;
    fn read_register(&mut self, register: u8) -> Result<u8, Self::Error> {
        let mut data = [register + 0x80, 0];
        self.spi.transfer_in_place(&mut data).map_err(Error::Comm)?;
        Ok(data[1])
    }

    fn read_data(&mut self, payload: &mut [u8]) -> Result<(), Self::Error> {
        payload[0] += 0x80;
        self.spi.transfer_in_place(payload).map_err(Error::Comm)?;
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
