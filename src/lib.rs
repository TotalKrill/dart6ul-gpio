use std::fs::File;
use std::io;
use std::io::prelude::*;


mod input;
mod output;

pub use input::*;
pub use output::*;

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
