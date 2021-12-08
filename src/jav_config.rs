use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub enum BinaryType {
    None = 0,
    Windows32 = 1,
    Windows64 = 2,
    Linux = 3,
    OSX = 4,
    WindowsCompat32 = 5,
    WindowsCompat64 = 6,
}

#[derive(Debug)]
pub struct ClientFile {
    pub name: String,
    pub crc: String,
    pub hash: String,
}

#[derive(Debug)]
pub struct JavConfig {
    pub binary_type: BinaryType,
    pub properties: HashMap<String, String>,
    pub messages: HashMap<String, String>,
    pub params: HashMap<String, String>,
    pub files: Vec<ClientFile>,
}

impl JavConfig {
    pub fn get_base_url(&self) -> Option<String> {
        let codebase = self.properties.get("codebase")?;
        Option::Some(format!("{}client?binaryType={}", codebase, self.binary_type as u8))
    }
}

fn parse(raw: &str) -> JavConfig {
    let mut jav_config = JavConfig {
        binary_type: BinaryType::None,
        properties: HashMap::new(),
        messages: HashMap::new(),
        params: HashMap::new(),
        files: Vec::new(),
    };

    for line in raw.lines() {
        if line.starts_with("msg=") {
            let (key, value) = split_key_value(line.trim_start_matches("msg="));
            jav_config.messages.insert(key, value);
        } else if line.starts_with("param=") {
            let (key, value) = split_key_value(line.trim_start_matches("param="));
            jav_config.params.insert(key, value);
        } else {
            let (key, value) = split_key_value(line);
            jav_config.properties.insert(key, value);
        }
    }

    let binary_count = jav_config.properties.get("binary_count")
        .map_or("0", |s| s.as_str())
        .parse::<i32>()
        .unwrap_or(0);
    for i in 0..binary_count {
        let name = jav_config.properties.remove(format!("download_name_{}", i).as_str()).unwrap();
        let crc = jav_config.properties.remove(format!("download_crc_{}", i).as_str()).unwrap();
        let hash = jav_config.properties.remove(format!("download_hash_{}", i).as_str()).unwrap();
        jav_config.files.push(ClientFile { name, crc, hash });
    }

    jav_config
}

pub fn load(binary_type: BinaryType) -> Result<JavConfig, Box<dyn Error>> {
    let url = format!("https://runescape.com/jav_config.ws?binaryType={}", binary_type as u8);
    let response = reqwest::blocking::get(url)?.error_for_status()?;

    let jav_config_text = response.text()?;
    let mut jav_config = parse(jav_config_text.as_str());
    jav_config.binary_type = binary_type;
    Result::Ok(jav_config)
}

fn split_key_value(line: &str) -> (String, String) {
    let split: Vec<&str> = line.splitn(2, "=").collect();
    (split[0].to_string(), split[1].to_string())
}
