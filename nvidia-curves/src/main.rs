use std::time::Duration;
use nvidia_curves::FanControl;

struct CurvePoint {
    temperature: u32,
    speed: u32
}

static CURVE: [CurvePoint; 8] = [
    CurvePoint{ temperature: 40, speed: 35 },
    CurvePoint{ temperature: 45, speed: 45 },
    CurvePoint{ temperature: 65, speed: 45 },
    CurvePoint{ temperature: 70, speed: 50 },
    CurvePoint{ temperature: 75, speed: 55 },
    CurvePoint{ temperature: 80, speed: 65 },
    CurvePoint{ temperature: 85, speed: 75 },
    CurvePoint{ temperature: 90, speed: 90 },
];

fn main() {
    let mut fan_control = FanControl::new();

    loop {
        let temp = fan_control.get_temperature();

        let speed = CURVE
            .iter()
            .filter_map(|p| if p.temperature <= temp { Some(p.speed) } else { None })
            .last();

        println!("{temp}");

        if let Some(speed) = speed {
            fan_control.set_control_speed(Some(speed));
        } else {
            fan_control.set_control_speed(None);
        };

        std::thread::sleep(Duration::from_secs(2));
    }
}
