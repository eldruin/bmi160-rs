pub struct Register;
impl Register {
    pub const CHIPID: u8 = 0x00;
    pub const PMU_STATUS: u8 = 0x03;
    pub const STATUS: u8 = 0x1B;
    pub const MAG: u8 = 0x04;
    pub const GYR: u8 = 0x0C;
    pub const ACC: u8 = 0x12;
    pub const SENSORTIME: u8 = 0x18;
    pub const CMD: u8 = 0x7E;
    pub const ACC_RANGE: u8 = 0x41;
    pub const GYR_RANGE: u8 = 0x43;
}

pub struct BitFlags;
impl BitFlags {
    pub const DRDY_ACC: u8 = 1 << 7;
    pub const DRDY_GYR: u8 = 1 << 6;
    pub const DRDY_MAG: u8 = 1 << 5;
    pub const NVM_RDY: u8 = 1 << 4;
    pub const FOC_RDY: u8 = 1 << 3;
    pub const MAG_MAN_OP: u8 = 1 << 2;
    pub const GYR_SELF_TEST_OK: u8 = 1 << 1;
}
