mod read_csv;

use nn::{NN, HaltCondition};
use read_csv::read_csv;
use std::fs::File;
use std::io::{Write, Read};
use std::time::SystemTime;

fn digit_as_vec_data(digit: f64) -> Result<Vec<f64>, String> {
    if digit < 0.0 || digit > 9.9 {return Err(String::from("digit out of range 0..9"))}
    let mut prototype = vec![1.,0.,0.,0.,0.,0.,0.,0.,0.,0.];
    prototype.swap(0, digit as usize);
    Ok(prototype)
}

fn replace(row: &mut Vec<f64>) -> (Vec<f64>, Vec<f64>) {
    let digit = row.remove(0);
    let digits = digit_as_vec_data(digit);
    (row.to_vec().to_owned(), digits.unwrap())
}

fn save_model(model_path: &str, model: &NN) -> std::io::Result<()> {
    let mut file = File::create(model_path)?;
    file.write_all(model.to_json().as_bytes())?;
    Ok(())
}

fn does_file_exist(path: &str) -> bool {
    match File::open(path) {
        Ok(_file) => true,
        Err(_e) => false,
    }
}

fn load_model(model_path: &str) -> std::io::Result<NN> {
    let mut file = File::open(model_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let net = NN::from_json(&contents);
    Ok(net)
}

fn main() {
    let start_time = SystemTime::now();
    println!("Start: {:?}", start_time);
    let model_path= "data/mnist-digits/model.json";
    let net = if does_file_exist(model_path) {
        load_model(model_path).expect("failed to load model")
    } else {
        let mut train_csv = read_csv("data/mnist-digits/train.csv", true).unwrap();


        // TODO This could do with putting back into a function now that it seems to run (also see replace_several_test below) ...
        let train_data = train_csv.iter_mut().map(|row| replace(row)).collect::<Vec<(Vec<f64>, Vec<f64>)>>();

        println!("Training model");
        let nn = train(&train_data);
        println!("Saving model");
        save_model(model_path, &nn).expect("failed to save the model");
        nn
    };
    let mut test_csv = read_csv("data/mnist-digits/test.csv", true).unwrap();
    let test_data = test_csv.iter_mut().map(|row| replace(row)).collect::<Vec<(Vec<f64>, Vec<f64>)>>();
    evaluate(net, &test_data);

    let end_time = SystemTime::now();
    println!("End: {:?}", end_time);
    let difference = end_time.duration_since(start_time)
        .expect("Clock may have gone backwards");
    println!("{:?}", difference);
}

fn train(data: &[(Vec<f64>, Vec<f64>)]) -> NN {
    let mut net = NN::new(&[784, 392, 98, 10]);

    net.train(&data)
        .halt_condition(HaltCondition::Epochs(10))
        .log_interval(Some(1))
        .momentum(0.1)
        .rate(0.3)
        .go();
    net
}

fn evaluate(net: NN, data: &Vec<(Vec<f64>, Vec<f64>)>) {
    data.iter().for_each(|(inputs, outputs)| {
        let results = net.run(&inputs.to_vec());

        let (result, key) = (results[0].round(), outputs[0]);
        println!("Result: {}     key: {}", result, key);
        assert_eq!(result, key);
    });
}

#[test]
fn digit_as_vec_data_test() {
    assert_eq!(digit_as_vec_data(9.).unwrap(), vec![0., 0. , 0. , 0. , 0. , 0. , 0. , 0. , 0. , 1. ]);
}

#[test]
fn replace_test() {
    let mut data = vec![9., 0.2, 0.3, 0.1];
    let replaced = replace(&mut data);
    assert_eq!(replaced, (vec![0.2, 0.3, 0.1], vec![0., 0., 0., 0., 0., 0., 0., 0., 0., 1. ]));
    println!("{:?}", data);
}

#[test]
fn replace_several_test() {
    let mut data = vec![vec![9., 0.2, 0.3, 0.1],vec![1., 0.4, 0.8, 0.3],];
    let replaced = data.iter_mut().map(|row| replace(row)).collect::<Vec<(Vec<f64>, Vec<f64>)>>();
    let expected: Vec<(Vec<f64>, Vec<f64>)> = vec![
        (vec![0.2, 0.3, 0.1], vec![0., 0., 0., 0., 0., 0., 0., 0., 0., 1.]),
        (vec![0.4, 0.8, 0.3], vec![0., 1., 0., 0., 0., 0., 0., 0., 0., 0.]),
    ];
    assert_eq!(replaced, expected);
}

#[test]
fn does_the_file_really_exist_test() {
    let exist = does_file_exist("this does not exist");
    assert!(!exist, "File somehow reported as existing");
    assert!(!does_file_exist("data/mnist-digits/model.json"));
}
