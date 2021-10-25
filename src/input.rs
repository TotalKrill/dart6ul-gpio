pub use embedded_hal::digital::v2::InputPin;

use std::{
    fs::File,
    io::{ErrorKind, Read},
};

use crate::{Error, Pin};

pub struct InPin {
    num: u32,
}

impl InPin {
    /// Resets the pins by unexporting the pins from userspace through its file interface, to reset its state, then configures a new
    /// pin. This should make sure that the pin is usable.
    ///
    /// Note: It does not take into account if other
    /// applications are using the pins or anything like that.
    pub fn force_new(port: u8, index: u8) -> Result<Self, Error> {
        Pin::force_reset(port, index);
        Self::new(port, index)
    }

    /// Tries to export and configure a new output pin, this can error out due to the pin already
    /// configured, usually with a device or resource busy
    pub fn new(port: u8, index: u8) -> Result<Self, Error> {
        let num = Pin::init(port, index, "in")?;
        Ok(InPin { num })
    }

    fn read_is_high(&self) -> Result<bool, Error> {
        // Read input from the sysfs interface
        let mut value = File::create(format!("/sys/class/gpio/gpio{}/value", self.num))?;

        // Read a byte from the file
        let buf = &mut [0u8; 1];
        value.read(buf)?;

        // If there was a character in the input and it was '0' or '1'
        if buf.len() == 1 && (buf[0] == b'1' || buf[0] == b'0') {
            Ok(buf[0] == b'1')
        } else {
            // Somehow, the sysfs interface returned an invalid value
            Err(Error::new(
                ErrorKind::Other,
                "Invalid value was read from sysfs interface",
            ))
        }
    }
}

impl InputPin for InPin {
    type Error = Error;

    fn is_high(&self) -> Result<bool, Self::Error> {
        self.read_is_high()
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        self.read_is_high().map(|val| !val)
    }
}

// Implement drop so that we can remove the pin from memory and setup once it is not to be used
// anymore
impl Drop for InPin {
    fn drop(&mut self) {
        Pin::force_reset_abs(self.num);
    }
}
