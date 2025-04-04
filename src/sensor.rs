use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use rand::Rng;
use std::time::SystemTime;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct SensorData {
    pub sensor_id: usize,
    pub value: f64,
    pub timestamp: SystemTime,
}

pub fn start_sensors(tx: Sender<SensorData>, num_sensors: usize, running: Arc<AtomicBool>) -> Vec<thread::JoinHandle<()>> {
    let mut handles = Vec::new();

    for id in 0..num_sensors {
        let sensor_tx = tx.clone();
        let running_clone = running.clone();
        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            while running_clone.load(Ordering::Relaxed) {
                let baseline = 25.0;
                let noise: f64 = rng.gen_range(-1.0..1.0);
                let mut value = baseline + noise;

                if rng.gen_bool(0.05) {
                    if rng.gen_bool(0.5) {
                        value = baseline - rng.gen_range(5.0..10.0);
                    } else {
                        value = baseline + rng.gen_range(5.0..10.0);
                    }
                }

                let data = SensorData {
                    sensor_id: id,
                    value,
                    timestamp: SystemTime::now(),
                };

                if sensor_tx.send(data).is_err() {
                    break;
                }

                thread::sleep(Duration::from_millis(500));
            }
        });
        handles.push(handle);
    }
    handles
}
