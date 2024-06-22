mod fan_control;
mod curves;

use curves::CurveManager;
use std::{error::Error, time::Duration};
use fan_control::FanControl;

fn main() -> Result<(), Box<dyn Error>> {
    let conf_reader = curves::LinuxConfReader{};
    let curves_manager = CurveManager::try_from(&conf_reader as &dyn curves::ConfReader)?;
    
    let mut fan_control = FanControl::new();
    loop {
        fan_control.update_fan_speed(curves_manager.get_curve(&Some("Profile1".to_owned())).unwrap());
        std::thread::sleep(Duration::from_secs(2));
    }
}
