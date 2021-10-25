use std::fs::File;
use std::io;
use std::io::prelude::*;

pub use embedded_hal::digital::v2::OutputPin;

mod input;

pub use input::*;

type Error = io::Error;

struct Pin;

impl Pin {
    /// Calculates the pins number for usage with the export and unexport files found on the linux
    /// system
    pub(crate) fn convert_to_absolute(port: u8, index: u8) -> u32 {
        let port = port as u32;
        let index = index as u32;
        let outnum = (port - 1) * 32 + index;
        outnum
    }

    pub(crate) fn force_reset_abs(num: u32) {
        if let Ok(mut export) = File::create("/sys/class/gpio/unexport") {
            let _e = export.write(num.to_string().as_bytes());
        }
    }

    pub(crate) fn force_reset(port: u8, index: u8) {
        let num = Pin::convert_to_absolute(port, index);
        Self::force_reset_abs(num);
    }

    pub(crate) fn init(port: u8, index: u8, direction: &str) -> Result<u32, Error> {
        let num = Self::convert_to_absolute(port, index);
        let mut export = File::create("/sys/class/gpio/export")?;
        export.write(num.to_string().as_bytes())?;

        let mut f_direction = File::create(format!("/sys/class/gpio/gpio{}/direction", num))?;
        f_direction.write(direction.as_bytes())?;
        Ok(num)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutPin {
    num: u32,
}

impl OutPin {
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
        let num = Pin::init(port, index, "out")?;
        Ok(OutPin { num })
    }

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
