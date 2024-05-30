use std::process::Command;
use nvml_wrapper::{enum_wrappers::device::TemperatureSensor, Nvml};

pub struct FanControl {
    current_speed: Option<u32>,
    nvml: Nvml,
}

impl FanControl {
    pub fn new() -> FanControl {
        FanControl {
            current_speed: None,
            nvml: Nvml::init().unwrap()
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

    pub fn get_temperature(&self) -> u32 {
        let device = self.nvml.device_by_index(0).expect("Could not get device");
        device.temperature(TemperatureSensor::Gpu).expect("Could not get temperature")
    }

    pub fn set_control_speed(&mut self, speed: Option<u32>) {
        let device = self.nvml.device_by_index(0).expect("Could not get device");
        let num_fans = device.num_fans().expect("Could not get number of fans");

        if self.current_speed == speed {
            return;
        }

        if self.current_speed.is_some() && speed.is_none() {
            for i in 0..num_fans {
               device.set_default_fan_speed(i).expect("Could not reset fan speed");
            }
        }

        self.current_speed = speed;
        let Some(speed) = speed else {
            return;
        };
        for i in 0..num_fans {
            device.set_fan_speed(i, speed).expect("Could not set fan speed");
        }
    }
}
