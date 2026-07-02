use std::io::Error;
use std::io::Write;

use chrono::Local;

pub struct UserData {
    pub time: String,
    pub high_score: i32
}

impl UserData {
    pub fn load(user_data_file: &String) -> UserData {
        let is_file_accessible = std::fs::metadata(user_data_file).map(|m| m.is_file()).unwrap_or(false);
        if is_file_accessible {
            let file_contents = std::fs::read_to_string(user_data_file).expect("No User File Found");
            if let Some(mut last_entry) = file_contents.lines().last() {
                last_entry = last_entry.trim();
                let words: Vec<&str> = last_entry.split_whitespace().collect();
                let score: i32 = words[1].parse().unwrap();
                return UserData { time: words[0].to_string(), high_score: score };
            }
        }

        UserData { time: "0.0.0".to_string(), high_score: 0 }
    }

    pub fn save(&mut self, user_data_file: &String) -> Result<(), Error> {
        self.time = Local::now().format("%Y-%m-%d:%H-%M-%S").to_string();
        let mut file = std::fs::OpenOptions::new().append(true).create(true).open(user_data_file)?;
        writeln!(file, "{} {}", self.time, self.high_score)?;
        Ok(())
    }
}
