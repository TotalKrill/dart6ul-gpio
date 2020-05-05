use dart6ul_gpio::OutPin;
use dart6ul_gpio::OutputPin;

use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut pin = OutPin::new(1, 5).unwrap();

    for _ in 1..10 {
        pin.set_low();
        sleep(Duration::from_secs(2));
        pin.set_high();
        sleep(Duration::from_secs(2));
    }
}
