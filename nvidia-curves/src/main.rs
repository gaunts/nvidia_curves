mod fan_control;
mod curves;
mod config_readers;

use config_readers::{ConfReader, LinuxConfigReader};
use curves::CurveManager;
use std::{error::Error, time::Duration};
use fan_control::FanControl;

fn main() -> Result<(), Box<dyn Error>> {
    let conf_reader = LinuxConfigReader{};
    let curves_manager = CurveManager::try_from(&conf_reader as &dyn ConfReader)?;
    
    let mut fan_control = FanControl::new();
    loop {
        fan_control.update_fan_speed(curves_manager.get_curve(&Some("Profile1".to_string())));
        std::thread::sleep(Duration::from_secs(2));
    }
}
