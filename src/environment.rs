use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

use crate::jav_config::ClientFile;

pub fn fetch_client_file(client_file: &ClientFile, base_url: &str) -> Result<(), Box<dyn Error>> {
    let path = get_client_file_path(&client_file.name);
    if path.exists() {
        let file_crc = get_crc(&path)?;
        if file_crc.to_string() == client_file.crc {
            println!("Skipping up-to-date file: {}", &client_file.name);
            return Result::Ok(());
        }
        println!("Updating file: {}", &client_file.name);
    } else {
        println!("Downloading missing file: {}", &client_file.name);
    }

    std::fs::create_dir_all(&path.parent().unwrap());
    let url = format!("{}&fileName={}&crc={}", base_url, &client_file.name, client_file.crc);
    download_and_decompress(&url, &path)
}

pub fn get_client_file_path(name: &str) -> PathBuf {
    let mut home = dirs::home_dir().unwrap();
    home.push("NXTLauncher");
    home.push(name);
    home
}

fn download_and_decompress(url: &str, path: &Path) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(path)?;
    let mut response = reqwest::blocking::get(url)?.error_for_status()?;
    let mut reader = BufReader::new(&mut response);

    lzma_rs::lzma_decompress(&mut reader, &mut file)?;
    Result::Ok(())
}

fn get_crc(path: &Path) -> Result<u32, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = std::io::BufReader::new(file);
    let crc = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
    let mut digest = crc.digest();
    let mut buffer = [0; 8192];
    loop {
        let read = reader.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        digest.update(&buffer[0..read]);
    }
    Result::Ok(digest.finalize())
}
