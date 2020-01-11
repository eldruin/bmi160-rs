//! I2C/SPI interfaces

use core::convert::From;

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

/// Possible slave addresses with value for SDO
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SlaveAddr(pub bool);

impl Default for SlaveAddr {
    /// Default slave address (SDO pulled to GND)
    fn default() -> Self {
        SlaveAddr(false)
    }
}

impl From<SlaveAddr> for u8 {
    fn from(addr: SlaveAddr) -> Self {
        I2C_DEV_BASE_ADDR | addr.0 as u8
    }
}

#[cfg(test)]
mod tests {
    use super::SlaveAddr;
    use super::I2C_DEV_BASE_ADDR as ADDR;

    #[test]
    fn can_get_default_address() {
        let addr = SlaveAddr::default();
        assert_eq!(ADDR, addr.into());
    }

    #[test]
    fn can_generate_alternative_addresses() {
        assert_eq!(ADDR, SlaveAddr(false).into());
        assert_eq!(ADDR | 1, SlaveAddr(true).into());
    }
}
