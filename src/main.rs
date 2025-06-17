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
    println!("Hello, world!");
}
