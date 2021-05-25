use crate::environment::{fetch_client_file, get_client_file_path};
use crate::jav_config::BinaryType;

mod jav_config;
mod environment;

fn main() {
    let binary_type = BinaryType::Windows64;

    let jav_config = match jav_config::load(binary_type) {
        Ok(jav_config) => jav_config,
        Err(err) => {
            eprintln!("Error while loading JavConfig: {}", err);
            return;
        }
    };

    let base_url = jav_config.get_base_url().unwrap();
    for file in jav_config.files.iter() {
        if let Err(err) = fetch_client_file(file, &base_url) {
            eprintln!("Error while fetching file {}: {}", &file.name, err);
            return;
        }
    }

    let binary_name = jav_config.properties.get("binary_name").unwrap();
    let executable = get_client_file_path(binary_name).unwrap();
}
