use rppal::pwm::{Channel, Polarity, Pwm};
use std::fs::File;
use std::io::{self, Read};
use std::thread;
use std::time::Duration;

fn get_cpu_temp() -> io::Result<f32> {
    let mut file = File::open("/sys/class/thermal/thermal_zone0/temp")?;
    let mut temp_str = String::new();
    file.read_to_string(&mut temp_str)?;
    let temp_millidegrees: i32 = temp_str.trim().parse().unwrap_or(0);
    let temp_celsius = temp_millidegrees as f32 / 1000.0;
    Ok(temp_celsius)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use Channel::Pwm0 for GPIO 12 (pin 32)
    let pwm = Pwm::with_frequency(Channel::Pwm0, 25000.0, 0.0, Polarity::Normal, true)?;

    let mut report_counter = 3;

    loop {
        report_counter -= 1;
        match get_cpu_temp() {
            Ok(temp) => {
                let duty_cycle = match temp {
                    // Any temp up to 40°C keep the duty cycle at 20%
                    t if t < 40.0 => 0.2,
                    t if t > 40.0 && t < 45.0 => 0.3,
                    t if t > 45.0 && t < 50.0 => 0.4,
                    t if t > 50.0 && t < 55.0 => 0.6,
                    t if t > 55.0 && t < 60.0 => 0.7,
                    t if t > 60.0 => {
                        println!("CPU Temperature getting toasty: {:.2}°C - High", temp);
                        1.0
                    },
                    _ => 0.2,
                };
                pwm.set_duty_cycle(duty_cycle)?;
                // Don't spam the logs, just output a message every 15 mins
                if report_counter <= 1 {
                    println!(
                        "CPU Temperature: {:.2}°C, Fan Duty Cycle: {:.0}%",
                        temp, duty_cycle * 100.0
                    );
                    report_counter = 180;
                }
            }
            Err(e) => {
                eprintln!("Failed to read temperature: {}", e);
                pwm.set_duty_cycle(0.5)?; // Leave the fan spinning at 50% just in case
                break;
            }
        }

        // Check every second
        thread::sleep(Duration::from_secs(5));
    }

    Ok(())
}
