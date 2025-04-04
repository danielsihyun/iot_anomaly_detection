use crate::sensor::SensorData;
use std::sync::mpsc::Receiver;

pub fn start_detection(rx: Receiver<SensorData>) {
    let lower_threshold = 20.0;
    let upper_threshold = 30.0;

    loop {
        match rx.recv() {
            Ok(data) => {
                if data.value < lower_threshold || data.value > upper_threshold {
                    println!(
                        "Anomaly detected from sensor {}: value = {:.2} at time: {:?}",
                        data.sensor_id,
                        data.value,
                        data.timestamp
                    );
                } else {
                    println!(
                        "Normal reading from sensor {}: value = {:.2}",
                        data.sensor_id,
                        data.value
                    );
                }
            },
            Err(_) => break,
        }
    }
}
