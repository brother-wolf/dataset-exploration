use std::fs::File;
use std::io::{Read, Write};
use nn::NN;

pub fn does_file_exist(path: &str) -> bool {
    match File::open(path) {
        Ok(_file) => true,
        Err(_e) => false,
    }
}

pub fn save_file(file_path: &str, contents: &str) -> std::io::Result<()> {
    Ok(File::create(file_path)?.write_all(contents.as_bytes())?)
}

pub fn load_file(file_path: &str) -> std::io::Result<String> {
    let mut contents = String::new();
    File::open(file_path)?.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn load_model(model_path: &str) -> std::io::Result<NN> {
    Ok(NN::from_json(&load_file(model_path)?))
}

pub fn save_model(save_model_path: &str, nn: &NN) {
    if does_file_exist(save_model_path) {
        println!("WARN: Not saving model: Model path already exists");
    } else {
        save_file(save_model_path, &nn.to_json()).expect("failed to save the model");
    }
}

#[test]
fn does_the_file_really_exist_test() {
    let exist = does_file_exist("this does not exist");
    assert!(!exist, "File somehow reported as existing");
    assert!(!does_file_exist("data/mnist-digits/model.json"));
}
