use std::time::Duration;

use nvidia_curves::FanControl;
use nvml_wrapper::enum_wrappers::device::TemperatureSensor;
use nvml_wrapper::Nvml;

// use nvml_wrapper::{enum_wrappers::device::TemperatureSensor, Nvml};

struct CurvePoint {
    temperature: u32,
    speed: u32
}

static CURVE: [CurvePoint; 8] = [
    CurvePoint{ temperature: 50, speed: 35 },
    CurvePoint{ temperature: 60, speed: 40 },
    CurvePoint{ temperature: 65, speed: 45 },
    CurvePoint{ temperature: 70, speed: 50 },
    CurvePoint{ temperature: 75, speed: 55 },
    CurvePoint{ temperature: 80, speed: 65 },
    CurvePoint{ temperature: 85, speed: 75 },
    CurvePoint{ temperature: 90, speed: 90 },
];

fn main() {
    let nvml = Nvml::init().unwrap();
    let device = nvml.device_by_index(0).unwrap();
    let mut fan_control = FanControl::new();

    loop {
        let temp = device.temperature(TemperatureSensor::Gpu).unwrap();
        
        let speed = CURVE
            .iter()
            .filter_map(|p| if p.temperature <= temp { Some(p.speed) } else { None })
            .last();
    
        if let Some(speed) = speed {
            fan_control.set_control_speed(&device, Some(speed));
        } else {
            fan_control.set_control_speed(&device, None);
        };

        std::thread::sleep(Duration::from_secs(2));
    }
}
