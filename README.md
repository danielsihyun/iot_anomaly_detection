Overview

This project simulates a network of IoT sensors that continuously generate readings. Each sensor sends its reading (a simulated temperature value) to a central detection engine that checks for anomalies. A reading is considered anomalous if it is below 20.0 or above 30.0. The project uses Rust's concurrency features, including threads and channels, to mimic a real-time monitoring system.

Project Structure

• Cargo.toml - Contains project configuration and dependencies (the rand crate). 
• src/main.rs - The entry point; it sets up sensor threads, the detection engine, and a shutdown mechanism. 
• src/sensor.rs - Simulates sensor behavior by generating periodic readings with noise and occasional anomalies. 
• src/detection.rs - Receives sensor data and prints messages indicating whether readings are normal or anomalous.

Installation and Running

Install Rust (if not already installed).
Open a terminal and navigate to the project directory.
Run the command "cargo run" to compile and start the project.
The program will continuously display sensor readings. To shut it down, press Enter.

Shutdown Mechanism

A separate thread listens for user input. When the user presses Enter, a shared atomic flag is set to signal sensor threads to stop. Once all sensor threads exit and the channel is closed, the detection loop also ends, allowing the program to terminate gracefully.
