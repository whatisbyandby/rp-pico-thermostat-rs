use embedded_hal::i2c::{Error as I2cError, I2c};

const AHT20_ADDRESS: u8 = 0x38;

#[derive(Debug)]
pub enum EnvironmentSensorError<E> {
    I2c(E),
    SensorError,
}

pub struct EnvironmentSensor<I2C> {
    i2c: I2C,
}

impl<I2C, E> EnvironmentSensor<I2C>
where
    I2C: I2c<Error = E>,
{
    pub fn new(i2c: I2C) -> Self {
        EnvironmentSensor { i2c }
    }

    pub fn init(&mut self) -> Result<(), EnvironmentSensorError<E>> {
        let mut buf = [0; 1];
        self.i2c
            .write_read(AHT20_ADDRESS, &[0x71], &mut buf)
            .map_err(EnvironmentSensorError::I2c)?;
        Ok(())
    }

    pub fn read_temperature_humidity(&mut self) -> Result<(f32, f32), EnvironmentSensorError<E>> {
        let mut buf = [0; 6];
        self.i2c
            .write_read(AHT20_ADDRESS, &[0xAC, 0x33, 0x00], &mut buf)
            .map_err(EnvironmentSensorError::I2c)?;

        // Check the status bit
        if (buf[0] & 0x80) != 0 {
            return Err(EnvironmentSensorError::SensorError);
        }

        // Calculate humidity from buffer
        let raw_hum = ((buf[1] as u32) << 12) | ((buf[2] as u32) << 4) | ((buf[3] as u32) >> 4);
        let humidity = raw_hum as f32 * 100.0 / 1048576.0;

        // Calculate temperature from buffer
        let raw_temp = ((buf[3] as u32 & 0x0F) << 16) | ((buf[4] as u32) << 8) | (buf[5] as u32);
        let temperature = raw_temp as f32 * 200.0 / 1048576.0 - 50.0;

        Ok((temperature, humidity))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

    #[test]
    fn test_init() {
        let expectations = [I2cTransaction::write(AHT20_ADDRESS, vec![0xBE, 0x08, 0x00])];
        let mut mock = I2cMock::new(&expectations);

        let mut sensor = EnvironmentSensor::new(mock.clone());
        assert!(sensor.init().is_ok());

        mock.done();
    }

    #[test]
    fn test_read_temperature_humidity() {
        let expectations = [I2cTransaction::write_read(
            AHT20_ADDRESS,
            vec![0xAC, 0x33, 0x00],
            vec![0, 0x80, 0x00, 0x05, 0x99, 0x9A],
        )];
        let mut mock = I2cMock::new(&expectations);

        let mut sensor = EnvironmentSensor::new(mock.clone());
        let result = sensor.read_temperature_humidity();
        assert!(result.is_ok());
        let (temperature, humidity) = result.unwrap();
        assert_approx_eq!(temperature, 20.0, 0.01);
        assert_approx_eq!(humidity, 50.0, 0.01);

        mock.done();
    }
}
