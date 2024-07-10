#[derive(Clone, Debug, PartialEq)]
pub enum ThermostatMode {
    Off,
    Heat,
    Cool,
    FanOnly,
}

#[derive(Debug, PartialEq)]
pub enum ThermostatCommand {
    SetTemperature(f32),
    SetMode(ThermostatMode),
}

pub struct Thermostat {
    target_temperature: f32,
    current_temperature: f32,
    mode: ThermostatMode,
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
            ThermostatCommand::SetMode(mode) => {
                self.mode = mode;
            }
        }
    }

    pub fn get_target_temperature(&self) -> f32 {
        self.target_temperature
    }

    pub fn get_current_temperature(&self) -> f32 {
        self.current_temperature
    }

    pub fn get_mode(&self) -> ThermostatMode {
        self.mode.clone()
    }
}

pub struct ThermostatBuilder {
    target_temperature: f32,
    default_mode: ThermostatMode,
}

impl ThermostatBuilder {
    pub fn new() -> Self {
        Self {
            target_temperature: 20.0,
            default_mode: ThermostatMode::Off,
        }
    }

    pub fn build(self) -> Thermostat {
        Thermostat {
            target_temperature: self.target_temperature,
            current_temperature: 0.0,
            mode: self.default_mode,
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
}
