use rand;
use std::fmt::Display;
use uuid::Uuid;

#[derive(Clone)]
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

#[derive(Clone)]
pub struct SmartSocket {
    id: Uuid,
    name: String,
    state: bool,
    power: f64,
}

impl SmartSocket {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            state: false,
            power: 0.0,
        }
    }

    pub fn switch(&mut self, state: bool) {
        self.state = state;

        if self.is_on() {
            self.power = (rand::random::<f64>() * 100.0).round() / 100.0
        } else {
            self.power = 0.0
        }
    }

    pub fn is_on(&self) -> bool {
        self.state
    }

    pub fn get_status(&self) {}
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
            "id: {}\nSmartSocket: {}, State: {}, Power: {}",
            &self.id,
            &self.name,
            if self.state { "On" } else { "Off" },
            &self.power
        )
    }
}

#[derive(Clone)]
pub struct SmartThermometer {
    id: Uuid,
    name: String,
    temperature: f64,
}
impl SmartThermometer {
    pub fn new(name: String, temperature: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            temperature,
        }
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
            &self.id, &self.name, &self.temperature
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
        let thermo = SmartThermometer::new(String::from("My Thermometer"), 23.4);

        assert_eq!(thermo.name, "My Thermometer");
    }

    #[test]
    fn socket_state_on() {
        let mut socket = SmartSocket::new(String::from("My Socket"));
        socket.switch(true);

        assert!(socket.state);
    }
}
