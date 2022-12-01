#![allow(clippy::needless_return)]

use crate::common::IReporter;
use crate::devices::IDevice;

pub struct SmartSocket {
    name: String,
}

impl SmartSocket {
    pub fn new(name: String) -> Self {
        return Self { name };
    }
}

impl IReporter for SmartSocket {
    fn create_report(&self) -> Result<String, String> {
        let message = format!("Это умная розетка '{}'. Роботает штатно.", &self.name);
        return Ok(message);
    }
}

impl IDevice for SmartSocket {
    fn get_name(&self) -> &String {
        return &self.name;
    }
}

pub struct SmartThermometer {
    name: String,
}

impl SmartThermometer {
    pub fn new(name: String) -> Self {
        return Self { name };
    }
}

impl IReporter for SmartThermometer {
    fn create_report(&self) -> Result<String, String> {
        let message = format!("Это умный термометр '{}'. Роботает штатно.", self.name);
        return Ok(message);
    }
}

impl IDevice for SmartThermometer {
    fn get_name(&self) -> &String {
        return &self.name;
    }
}
