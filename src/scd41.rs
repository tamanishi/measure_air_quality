use chrono::Local;
use linux_embedded_hal::{Delay, I2cdev};
use scd4x::scd4x::Scd4x;

use crate::data::AirQuality;

pub struct Scd41 {
    sensor: Scd4x<I2cdev, Delay>,
}

impl Scd41 {
    pub fn new() -> Scd41 {
        let i2c = I2cdev::new("/dev/i2c-0").expect("Couldn't start i2c.");
        Scd41 {
            sensor: Scd4x::new(i2c, Delay),
        }
    }

    pub fn read_single_shot(&mut self) -> Result<AirQuality, String> {
        self.sensor.wake_up();
        self.sensor.stop_periodic_measurement().unwrap();
        self.sensor.reinit().unwrap();
        self.sensor.measure_single_shot().unwrap();
        match self.sensor.measurement() {
            Ok(data) => Ok(AirQuality {
                timestamp: Local::now(),
                co2: From::from(data.co2),
                temperature: From::from(data.temperature),
                humidity: From::from(data.humidity),
            }),
            Err(e) => Err(format!("Couldn't read data: {:?}", e)),
        }
    }
}
