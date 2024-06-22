use nvml_wrapper::{enum_wrappers::device::TemperatureSensor, Nvml};

use crate::curves::CurvePoint;

pub struct FanControl {
    current_speed: Option<u32>,
    nvml: Nvml,
}

impl FanControl {
    pub fn new() -> Self {
        Self {
            current_speed: None,
            nvml: Nvml::init().unwrap()
        }
    }

    fn get_temperature(&self) -> u32 {
        let device = self.nvml.device_by_index(0).expect("Could not get device");
        device.temperature(TemperatureSensor::Gpu).expect("Could not get temperature")
    }

    fn set_control_speed(&mut self, speed: Option<u32>) {
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

    pub fn update_fan_speed(&mut self, curve: &[CurvePoint]) {
        let temp = self.get_temperature();

        let speed = curve
            .iter()
            .filter_map(|p| if p.temperature <= temp { Some(p.speed) } else { None })
            .last();

        self.set_control_speed(speed);
    }
}
