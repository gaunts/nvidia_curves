use std::process::Command;

use nvml_wrapper::{Device, Nvml};


pub struct FanControl {
    current_speed: Option<u32>,
}

impl FanControl {
    pub fn new() -> FanControl {
        FanControl {
            current_speed: None,
        }
    }

    #[allow(unused)]
    fn send_state_command(state: bool) {
        Command::new("sudo")
            .arg("nvidia-settings")
            .arg("-a")
            .arg(format!("GPUFanControlState={}", if state { "1" } else { "0" }))
            .status()
            .expect("Error setting GPUFanControlState");
    }

    #[allow(unused)]
    fn send_speed_command(speed: u32) {
        Command::new("sudo")
            .arg("nvidia-settings")
            .arg("-a")
            .arg(format!("GPUTargetFanSpeed={speed}"))
            .status()
            .expect("Error setting GPUTargetFanSpeed");
    }

    pub fn set_control_speed(&mut self, device: &Device, speed: Option<u32>) {
        if self.current_speed == speed {
            return;
        }

        if self.current_speed.is_some() && speed.is_none() {
            device.set_default_fan_speed(0).expect("Could not reset fan speed");
            device.set_default_fan_speed(1).expect("Could not reset fan speed");
        }

        self.current_speed = speed;
        let Some(speed) = speed else {
            return;
        };
        device.set_fan_speed(0, speed).expect("Could not set fan speed");
        device.set_fan_speed(1, speed).expect("Could not set fan speed");
    }
}
