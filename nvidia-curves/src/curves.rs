use std::{collections::HashMap, error::Error};

use crate::config_readers::ConfReader;

#[derive(Debug)]
struct SpeedTooHighError {
    value: u32 
}

impl std::fmt::Display for SpeedTooHighError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid fan speed {} rpm. Maximum accepted value is 100", self.value)
    }
}

pub struct CurvePoint {
    pub temperature: u32,
    pub speed: u32
}

impl CurvePoint {
    fn new(temperature: u32, speed: u32) -> Result<Self, SpeedTooHighError> {
        if speed > 100 {
            Err(SpeedTooHighError{ value: speed })
        } else {
            Ok(Self {
                temperature,
                speed
            })
        }
    }
}

pub struct CurveManager {
    curves: HashMap<Option<String>, Vec<CurvePoint>>
}

impl TryFrom<&dyn ConfReader> for CurveManager {
    type Error = Box<dyn Error>;

    fn try_from(value: &dyn ConfReader) -> Result<Self, Self::Error> {
        let mut loader = CurveManager {
            curves: HashMap::new()
        };
        let conf_string = value.get_conf_string()?;
        loader.load_profiles(&conf_string)?;
        Ok(loader)
    }
}

impl CurveManager {
    fn load_profiles(&mut self, conf_string: &str) -> Result<(), Box<dyn Error>> {
        let profiles_lines = conf_string
            .split("\n\n")
            .map(|s| s.lines()
        );

        for profile in profiles_lines {
            let profile = Self::parse_profile(profile)?;
            self.curves.insert(profile.0, profile.1);
        }
        Ok(())
    }

    fn parse_profile(lines: std::str::Lines) -> Result<(Option<String>, Vec<CurvePoint>), Box<dyn Error>> {
        let mut profile: Option<String> = None;
        let mut curve_points: Vec<CurvePoint> = Vec::new();

        for line in lines {
            if line.starts_with('[') && line.ends_with(']') {
                profile = Some(line[1..line.len()-1].to_string());
            } else {
                let values: Vec<u32> = line
                    .split_whitespace()
                    .map(str::parse)
                    .collect::<Result<Vec<u32>, _>>()?;
                assert_eq!(values.len(), 2);
                curve_points.push(CurvePoint::new(values[0], values[1]).expect("Error building CurvePoint"));
            }
        }
        Ok((profile, curve_points))
    }
    
    pub fn get_curve(&self, profile_name: &Option<String>) -> &Vec<CurvePoint> {
        self.curves.get(profile_name).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let curves_loader = CurvesManager::new(&["../resources/tests/curves1.conf"]);

    }
}
