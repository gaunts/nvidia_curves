use core::fmt;
use std::{collections::HashMap, error::Error};

pub trait ConfReader {
    fn get_conf_string(&self) -> Result<String, Box<dyn Error>>;
}

pub struct LinuxConfReader {}

#[derive(Debug)]
pub enum ConfReaderError {
    IoError(String, std::io::Error),
}

impl Error for ConfReaderError {}

impl fmt::Display for ConfReaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(path, err) => write!(f, "IO Error at path '{path}': {err}"),
        }
    }
}

impl ConfReader for LinuxConfReader {
    fn get_conf_string(&self) -> Result<String, Box<dyn Error>> {
        const PATH: &str = "/etc/nvidia-curves.d/curves.conf";
        Ok(std::fs::read_to_string(PATH)
            .map_err(|err| ConfReaderError::IoError(PATH.to_string(), err))?
        )
    }
}

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
    const fn new(temperature: u32, speed: u32) -> Result<Self, SpeedTooHighError> {
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

#[derive(Debug)]
pub enum InvalidProfileNameError {
    Empty,
    TooBig,
    WrongCharacters
}

#[derive(Debug)]
pub enum CurveConfLoadingError {
    UnexpectedEmptyLine(u32),
    UnexpectedCharacter(u32),
    EmptyCurve,
    InvalidCurvePoint,
    InvalidProfileName(InvalidProfileNameError)
}
impl std::error::Error for CurveConfLoadingError {}
impl fmt::Display for CurveConfLoadingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}


pub struct CurveManager {
    curves: HashMap<Option<String>, Vec<CurvePoint>>
}

impl TryFrom<&dyn ConfReader> for CurveManager {
    type Error = Box<dyn Error>;

    fn try_from(value: &dyn ConfReader) -> Result<Self, Self::Error> {
        let mut loader = Self {
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
            if line.is_empty() {
                Err(CurveConfLoadingError::UnexpectedEmptyLine(1))?
            }
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
    
    pub fn get_curve(&self, profile_name: &Option<String>) -> Option<&Vec<CurvePoint>>  {
        self.curves.get(profile_name)
    }
}

#[cfg(test)]
mod tests {
    use super::{ConfReader, CurveManager};

    struct TestConfReader<'a> {
        conf_string: &'a str
    }

    impl<'a>  ConfReader for TestConfReader<'a>  {
        fn get_conf_string(&self) -> Result<String, Box<dyn std::error::Error>> {
            Ok(self.conf_string.to_owned())
        }
    }

    #[test]
    fn unexpected_character() {
        let conf_reader = TestConfReader { conf_string: "asdsqds" };
        let manager = CurveManager::try_from(&conf_reader as &dyn ConfReader);
        assert!(manager.is_err());
    }
    
    #[test]
    fn unexpected_empty_line() {
        let conf_reader = TestConfReader { conf_string: 
"
[profile1]
12 54
54 88
"};
        let manager = CurveManager::try_from(&conf_reader as &dyn ConfReader);
        assert!(manager.is_err());
        if let Err(e) = manager {
            let e = e.downcast::<crate::curves::CurveConfLoadingError>();
            assert!(e.is_ok_and(|e| {
                matches!(*e, crate::curves::CurveConfLoadingError::UnexpectedEmptyLine(line) if line == 1)
            }));
        }
    }
}
