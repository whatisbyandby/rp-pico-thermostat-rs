#[derive(Clone, Debug, PartialEq)]

pub enum TemperatureUnits {
    Celsius,
    Fahrenheit,
}

impl Default for TemperatureUnits {
    fn default() -> Self {
        TemperatureUnits::Celsius
    }
}

#[derive(Debug, PartialEq)]
pub enum ThermostatMode {
    Off,
}

impl Default for ThermostatMode {
    fn default() -> Self {
        ThermostatMode::Off
    }
}

#[derive(Debug, PartialEq)]
pub enum ThermostatCommand {
    SetTemperature(f32),
}

impl ThermostatCommand {
    pub fn parse(command: &str) -> Result<Self, ()> {
        let mut parts = command.split_whitespace();
        match parts.next() {
            Some("temperature") => {
                if let Some(temp) = parts.next() {
                    if let Ok(temp) = temp.parse::<f32>() {
                        if let Some(_) = parts.next() {
                            return Err(());
                        }
                        return Ok(ThermostatCommand::SetTemperature(temp));
                    }
                }
            }
            _ => {}
        }
        Err(())
    }
}

pub struct Thermostat {
    target_temperature: f32,
    mode: ThermostatMode,
    temperature_units: TemperatureUnits,
}

impl Thermostat {
    pub fn run(&mut self) -> Result<(), ()> {
        Ok(())
    }

    pub fn execute(&mut self, command: ThermostatCommand) {
        match command {
            ThermostatCommand::SetTemperature(temp) => {
                self.target_temperature = temp;
            }
        }
    }

    fn convert_temperature_from_std_units(&self, temperature: f32) -> f32 {
        match self.temperature_units {
            TemperatureUnits::Celsius => temperature,
            TemperatureUnits::Fahrenheit => temperature * 9.0 / 5.0 + 32.0,
        }
    }

    fn convert_temperature_to_std_units(&self, temperature: f32) -> f32 {
        match self.temperature_units {
            TemperatureUnits::Celsius => temperature,
            TemperatureUnits::Fahrenheit => (temperature - 32.0) * 5.0 / 9.0,
        }
    }
}

pub struct ThermostatBuilder {
    target_temperature: f32,
    default_mode: ThermostatMode,
    default_units: TemperatureUnits,
}

impl ThermostatBuilder {
    pub fn new() -> Self {
        Self {
            target_temperature: 20.0,
            default_mode: Default::default(),
            default_units: Default::default(),
        }
    }

    pub fn with_units(mut self, units: TemperatureUnits) -> Self {
        self.default_units = units;
        self
    }

    pub fn with_sensor(mut self) {}

    pub fn build(self) -> Thermostat {
        Thermostat {
            target_temperature: self.target_temperature,
            mode: self.default_mode,
            temperature_units: self.default_units,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_thermostat_builder_defaults() {
        let thermostat = ThermostatBuilder::new().build();
        assert_eq!(thermostat.target_temperature, 20.0);
        assert_eq!(thermostat.mode, ThermostatMode::Off);
    }

    #[test]
    fn test_thermostat_builder_with_units() {
        let thermostat = ThermostatBuilder::new()
            .with_units(TemperatureUnits::Fahrenheit)
            .build();
        assert_eq!(thermostat.temperature_units, TemperatureUnits::Fahrenheit);
    }

    #[test]
    fn test_convert_temperature_to_std_units() {
        // Standard temperature units for the thermostat are Celsius
        let mut thermostat = ThermostatBuilder::new()
            .with_units(TemperatureUnits::Fahrenheit)
            .build();

        // when units are Fahrenheit, the temperature should be converted to Celsius
        assert_eq!(thermostat.convert_temperature_to_std_units(68.0), 20.0);

        // when units are Celsius, the temperature should be returned as is
        thermostat.temperature_units = TemperatureUnits::Celsius;
        assert_eq!(thermostat.convert_temperature_to_std_units(20.0), 20.0);
    }

    #[test]
    fn test_convert_temperature_from_std_units() {
        // Standard temperature units for the thermostat are Celsius
        let mut thermostat = ThermostatBuilder::new()
            .with_units(TemperatureUnits::Fahrenheit)
            .build();

        // when units are Fahrenheit, the temperature should be to Fahrenheit
        assert_eq!(thermostat.convert_temperature_from_std_units(20.0), 68.0);

        // when units are Celsius, the temperature should be returned as is
        thermostat.temperature_units = TemperatureUnits::Celsius;
        assert_eq!(thermostat.convert_temperature_from_std_units(20.0), 20.0);
    }

    #[test]
    fn test_thermostat_command_parse() {
        let command = ThermostatCommand::parse("temperature 20.0").unwrap();
        assert_eq!(ThermostatCommand::SetTemperature(20.0), command);

        let command = ThermostatCommand::parse("temperature");
        assert_eq!(Err(()), command);

        let command = ThermostatCommand::parse("temperature one");
        assert_eq!(Err(()), command);

        let command = ThermostatCommand::parse("temperature 20.0 30.0");
        assert_eq!(Err(()), command);
    }
}
