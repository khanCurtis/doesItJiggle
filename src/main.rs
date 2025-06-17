use core_foundation::base::TCFType;
use core_foundation::date::CFAbsoluteTimeGetCurrent;
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
use enigo::{Enigo, MouseControllable};
use std::{thread::sleep, time::Duration};

fn get_idle() -> u64 {
    let last_input = CGEventSource::last_event(
        CGEventSourceStateID::HIDSystemState,
        None,
    );
    (last_input * 1000) as u64
}

fn main() {
    let threshold = 60_000;
    let mut nigo = Enigo::new();

    loop {
        let idle = get_idle();
        if idle > threshold {
            enigo.mouse_move_relative(1, 0);
            enigo.mouse_move_relative(-1, 0);
        }
        sleep(Duration::from_secs(5));
    }
}
