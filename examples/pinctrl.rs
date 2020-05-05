use dart6ul_gpio::OutPin;

// the embedded hal trait re-exported, if you want...
use dart6ul_gpio::OutputPin;

use std::thread::sleep;
use std::time::Duration;

fn main() {
    // resets pinstate before attempting to create a new one.
    let mut pin = OutPin::force_new(1, 5).unwrap();

    for _ in 1..10 {
        pin.set_low().unwrap();
        sleep(Duration::from_secs(2));
        pin.set_high().unwrap();
        sleep(Duration::from_secs(2));
    }
}
