use std::fs::File;
use std::io;
use std::io::prelude::*;

pub use embedded_hal::digital::v2::OutputPin;

pub struct OutPin {
    num: u32,
}

type Error = io::Error;

impl OutPin {
    fn convert_to_absolute(port: u8, index: u8) -> u32 {
        let port = port as u32;
        let index = index as u32;
        let outnum = (port - 1) * 32 + index;
        outnum
    }
    pub fn new(port: u8, index: u8) -> Result<Self, Error> {
        let num = Self::convert_to_absolute(port, index);
        let mut export = File::create("/sys/class/gpio/export")?;
        export.write(num.to_string().as_bytes())?;

        std::thread::sleep_ms(1000);

        let mut direction = File::create(format!("/sys/class/gpio/gpio{}/direction", num))?;
        direction.write("out".as_bytes())?;

        Ok(OutPin { num })
    }
}

impl OutputPin for OutPin {
    type Error = io::Error;

    fn set_low(&mut self) -> Result<(), Error> {
        let mut direction = File::create(format!("/sys/class/gpio/gpio{}/value", self.num))?;
        direction.write("0".as_bytes())?;
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Error> {
        let mut direction = File::create(format!("/sys/class/gpio/gpio{}/value", self.num))?;
        direction.write("1".as_bytes())?;
        Ok(())
    }
}
// Implement drop so that we can remove the pin from memory and setup once it is not to be used
// anymore
impl Drop for OutPin {
    fn drop(&mut self) {
        if let Ok(mut unexport) = File::create("/sys/class/gpio/unexport") {
            let _e = unexport.write(self.num.to_string().as_bytes());
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}