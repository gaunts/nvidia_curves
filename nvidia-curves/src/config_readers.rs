use std::error::Error;

pub trait ConfReader {
    fn get_conf_string(&self) -> Result<String, Box<dyn Error>>;
}

pub struct LinuxConfigReader {
}

impl ConfReader for LinuxConfigReader {
    fn get_conf_string(&self) -> Result<String, Box<dyn Error>> {
        // let path = "/etc/nvidia-curves.d/";
        // let dir = std::fs::read_dir(path)?;

        // std::fs::read_to_string(path);
        todo!();
    }
}
