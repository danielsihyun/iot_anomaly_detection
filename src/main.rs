mod sensor;
mod detection;

use sensor::start_sensors;
use detection::start_detection;
use std::sync::mpsc;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::{thread, io::{self, BufRead}};

fn main() {
    println!("Real-Time IoT Anomaly Detection Platform Starting...");

    let (tx, rx) = mpsc::channel();

    let running = Arc::new(AtomicBool::new(true));

    let sensor_handles = start_sensors(tx, 5, running.clone());

    // spawn thread for user program termination
    {
        let running_clone = running.clone();
        thread::spawn(move || {
            println!("Press Enter to exit...");
            let stdin = io::stdin();
            let mut handle = stdin.lock();
            let mut buffer = String::new();
            handle.read_line(&mut buffer).unwrap();
            running_clone.store(false, Ordering::Relaxed);
            println!("Shutting down...");
        });
    }

    start_detection(rx);

    for handle in sensor_handles {
        handle.join().unwrap();
    }

    println!("Program terminated.");
}
