use crate::file_opts::file_ops::load_file;
use std::io;
use crate::models::config::Config;

pub fn read_net_config(file_path: &str) -> io::Result<Config> {
    let contents = load_file(file_path)?;
    Ok(Config::from(&contents).unwrap())
}