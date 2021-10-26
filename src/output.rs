pub use embedded_hal::digital::v2::OutputPin;

use std::{fs::File, io::Write};

use crate::{Error, Pin};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutPin {
    num: u32,
}

impl OutPin {
    pub fn force_new(port: u8, index: u8) -> Result<Self, Error> {
        Pin::force_reset(port, index);
        Self::new(port, index)
    }

    /// Tries to export and configure a new output pin, this can error out due to the pin already
    /// configured, usually with a device or resource busy
    pub fn new(port: u8, index: u8) -> Result<Self, Error> {
        let num = Pin::init(port, index, "out")?;
        Ok(OutPin { num })
    }

    /// Write to the output pin
    fn write_output(&mut self, value: &str) -> Result<(), Error> {
        let mut direction = File::create(format!("/sys/class/gpio/gpio{}/value", self.num))?;
        direction.write(value.as_bytes())?;
        Ok(())
    }
}

impl OutputPin for OutPin {
    type Error = Error;

    fn set_low(&mut self) -> Result<(), Error> {
        self.write_output("0")
    }

    fn set_high(&mut self) -> Result<(), Error> {
        self.write_output("1")
    }
}

// Implement drop so that we can remove the pin from memory and setup once it is not to be used
// anymore
impl Drop for OutPin {
    fn drop(&mut self) {
        Pin::force_reset_abs(self.num);
    }
}
