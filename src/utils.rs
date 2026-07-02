use std::collections::HashMap;
use std::fs;
use std::io::Error;


pub fn safe_u8_add(a: u8, b: i32) -> u8 {
    let x = a as i32;
    let y = b;

    let m = 255;
    let sum = (x + y).rem_euclid(m * 2);
    let b_sum = m - (sum - m).abs();
    b_sum as u8
}


pub struct Settings {
    pub rotation: bool
}

impl Settings {
    pub fn new() -> Self {
        Settings { rotation: false }
    }

    pub fn load(settings_file: &String) -> Settings {
        let mut settings = Settings::new();

        if let Ok(contents) = fs::read_to_string(settings_file) {
            for line in contents.lines() {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                if let Some((key, value)) = line.split_once(':') {
                    let key = key.trim().trim_matches('"').to_string();
                    let value = value.trim().trim_matches('"').to_string();
                    if key == "rotation" {
                        settings.rotation = value == "true";
                    }
                }
            }
        }

        settings
    }

    pub fn save(&self, settings_file: &String) -> Result<(), Error> {
        let mut contents = String::new();
        contents.push_str(&format!("\"rotation\": \"{}\"\n", self.rotation));
        fs::write(settings_file, contents)
    }

}
