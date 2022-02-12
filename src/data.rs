use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct AirQuality {
    pub timestamp: DateTime<Local>,
    pub co2: u16,
    pub temperature: f32,
    pub humidity: f32,
}

impl fmt::Display for AirQuality {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "\"datetime\": {:?}, \"co2\": {:.0}, \"temperature\": {:.1}, \"humidity\": {:.1}",
            self.timestamp.format("%Y/%m/%d %H:%M:%S").to_string(),
            self.co2,
            self.temperature,
            self.humidity
        )
    }
}
