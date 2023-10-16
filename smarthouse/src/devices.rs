use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};
use socket_lib::{Command, Response};
use std::fmt::Display;

use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub enum Device {
    Socket(SmartSocket),
    Thermometer(SmartThermometer),
}

impl Device {
    pub fn get_id(&self) -> Option<Uuid> {
        match self {
            Device::Socket(socket) => Some(socket.id),
            Device::Thermometer(thermometer) => Some(thermometer.id),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SmartSocket {
    id: Uuid,
    name: String,
    enabled: bool,
}

impl SmartSocket {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            enabled: false,
        }
    }

    pub fn process_command(&mut self, cmd: Command) -> Response {
        match cmd {
            Command::TurnOn => {
                self.enabled = true;
                Response::Ok
            }
            Command::TurnOff => {
                self.enabled = false;
                Response::Ok
            }
            Command::IsEnabled => {
                if self.enabled {
                    Response::Enabled
                } else {
                    Response::Disabled
                }
            }
            Command::GetPower => {
                if self.enabled {
                    Response::Power(220.5)
                } else {
                    Response::Power(0.0)
                }
            }
            Command::Unknown => {
                println!("Unknown command received");
                Response::Unknown
            }
        }
    }
    fn get_power(&self) -> Response {
        if self.enabled {
            Response::Power(220.5)
        } else {
            Response::Power(0.0)
        }
    }

    fn is_enabled(&self) -> Response {
        if self.enabled {
            Response::Enabled
        } else {
            Response::Disabled
        }
    }
}

impl From<SmartSocket> for Device {
    fn from(value: SmartSocket) -> Self {
        Device::Socket(value)
    }
}

impl Display for SmartSocket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {}\nSmartSocket: {}, isEnabled: {}, Power: {}",
            &self.id,
            &self.name,
            &self.is_enabled(),
            &self.get_power(),
        )
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SmartThermometer {
    id: Uuid,
    name: String,
    started: DateTime<Utc>,
}
impl SmartThermometer {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            started: Utc::now(),
        }
    }

    pub fn get_temperature(&self) -> f32 {
        let delay = Utc::now() - self.started;
        20.0 + (delay.num_seconds() as f32 / 2.0).sin()
    }
}

impl From<SmartThermometer> for Device {
    fn from(value: SmartThermometer) -> Self {
        Device::Thermometer(value)
    }
}

impl Display for SmartThermometer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {}\nSmartThermometer: {}, temperature: {}°C",
            &self.id,
            &self.name,
            &self.get_temperature()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_device() {
        let device = SmartSocket::new(String::from("My Socket"));
        assert_eq!(device.name, "My Socket");
    }

    #[test]
    fn get_temperature_in_thermo() {
        let thermo = SmartThermometer::new(String::from("My Thermometer"));
        assert_eq!(thermo.name, "My Thermometer");
    }

    #[test]
    fn socket_state_on() {
        let mut socket = SmartSocket::new(String::from("My Socket"));
        socket.process_command(Command::TurnOn);

        assert!(socket.enabled);
    }
}
