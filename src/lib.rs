use embedded_hal::i2c::{I2c};

pub type Pressure = f32;
pub type Temperature = f32;
pub type Response = [u8; 5];
pub type PressureBytes = [u8; 3];
pub type TemperatureBytes = [u8; 2];


#[derive(Debug)]
pub struct XGZP6897D<I2C> {
    i2c: I2C,
    address: u8,
    conversion_factor: f32,
}

pub const DEVICE_ADDRESS: u8 = 0x6D;
pub const MEASUREMENT_COMMAND: u8 = 0x30;
pub const PRESSURE_REGISTER: u8 = 0x06;
pub const TEMPERATURE_REGISTER: u8 = 0x0A;

#[cfg(feature = "sync")]
impl<I2C: I2c> XGZP6897D<I2C>
{
    pub fn new(i2c: I2C, address: u8, conversion_factor: f32) -> Self {
        Self { i2c, address, conversion_factor }
    }

    pub fn read_sensor_raw(&mut self) -> Result<Response, I2C::Error> {
        let mut payload: Response = [0, 0, 0, 0, 0];

        self.i2c.write(self.address, &[MEASUREMENT_COMMAND, TEMPERATURE_REGISTER])?;
        self.i2c.write_read(self.address, &[MEASUREMENT_COMMAND], &mut [1])?;
        self.i2c.write_read(self.address, &[PRESSURE_REGISTER], &mut payload)?;

        Ok(payload)
    }

    pub fn read_sensor(&mut self) -> Result<(Pressure, Temperature), I2C::Error> {
        match self.read_sensor_raw() {
            Ok([pressure @ .., msb, lsb]) => {
                let temp = bytes_to_u16([msb, lsb]) as f32 / (256f32);
                let pressure_raw = bytes_to_i32(pressure) as f32 / &self.conversion_factor;
                Ok((pressure_raw, temp))
            }
            Err(error) => Err(error),
        }
    }
}

fn bytes_to_i32([msb, csb, lsb]: PressureBytes) -> i32 {
    let value = ((msb as u32) << 16) + ((csb as u32) << 8) + lsb as u32;
    match value & 0x800000 {
        0 => value as i32,
        _ => (value | 0xFF000000) as i32
    }
}

fn bytes_to_u16([msb, lsb]: TemperatureBytes) -> u16 {
    ((msb as u16) << 8) + (lsb as u16)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_to_i32() {
        for (input, output) in [
            ([0, 0, 0], 0),
            ([0, 0, 1], 1),
            ([0, 1, 0], 256),
            ([0, 255, 255], 65535),
            ([0, 255, 0], 65535 - 255),
            ([255, 255, 255], -1),
            ([255, 255, 254], -2),
        ] {
            assert_eq!(bytes_to_i32(input), output);
        }
    }

    #[test]
    fn test_bytes_to_u16() {
        for (input, output) in [
            ([0, 0], 0),
            ([0, 1], 1),
            ([1, 0], 256),
            ([0, 255], 255),
            ([255, 255], 65535),
            ([255, 0], 65535 - 255),
        ] {
            assert_eq!(bytes_to_u16(input), output);
        }
    }
}
