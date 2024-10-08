use fundsp::hacker32::*;
use io::*;
use std::{thread, time::Duration};

/// hello 440
fn main() {
    let graph = sine_hz(440.);
    play(Box::new(graph));
    thread::sleep(Duration::from_secs(1));
}
