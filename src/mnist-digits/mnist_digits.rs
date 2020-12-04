use std::time::SystemTime;

use nn::{HaltCondition, NN};
use structopt::StructOpt;

use crate::file_opts::file_ops::{does_file_exist, load_file, save_file, load_model, save_model};
use crate::file_opts::read_csv::{read_csv, split_and_normalise};
use crate::file_opts::read_json::read_net_config;
use crate::models::config::Config;

mod file_opts;
mod models;

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Opt {
    #[structopt(short = "l", long = "load-model-path")]
    load_model_path: Option<String>,
    #[structopt(short = "n", long = "neural-net-config", required = true)]
    neural_net_config: String,
    #[structopt(short = "s", long = "save-model-path")]
    save_model_path: Option<String>,
}

fn main() {
    let opt = Opt::from_args();
    let model_input_path: Option<String> = opt.model_input_path;
    let model_output_path: Option<String> = opt.model_output_path;

    let start_time = SystemTime::now();
    println!("Start: {:?}", start_time);
    // let model_path= "data/mnist-digits/model.json";
    let config = match read_net_config(&opt.neural_net_config) {
        Ok(c) => c,
        Err(e) => panic!(format!("{:?}", e)),
    };

    let train_validate_data = split_and_normalise(&mut read_csv("data/mnist-digits/train.csv", true).unwrap());
    let train_validation_split = (0.6 * train_validate_data.len() as f64) as usize;
    let (train_data, validation_data) = train_validate_data.split_at(train_validation_split);

    println!("train size: {}", &train_data.len());
    println!("valid size: {}", &validation_data.len());

    let net =
        if model_input_path.is_some() && does_file_exist(model_input_path.as_ref().unwrap()) {
            let path = model_input_path.as_ref().unwrap();
            load_model(path).expect("failed to load model")
        } else {
            println!("Training model");
            let nn = train(&train_data, &config);

            if model_output_path.is_some() {
                println!("Saving model");
                save_model(model_output_path.unwrap().as_str(), &nn);
            } else {
                println!("WARN: Not saving model: Model path not provided");
            }

            nn
        };

    // let test_data = split_and_normalise(&mut read_csv("data/mnist-digits/test.csv", true).unwrap());
    evaluate(net, &validation_data);

    let end_time = SystemTime::now();
    println!("End: {:?}", end_time);
    let difference = end_time.duration_since(start_time)
        .expect("Clock may have gone backwards");
    println!("{:?}", difference);
}


fn train(data: &[(Vec<f64>, Vec<f64>)], config: &Config) -> NN {
    // let mut net = NN::new(&[784, 392, 98, 10]);
    let mut net = NN::new(config.layers.as_slice());

    net.train(&data)
        .halt_condition(HaltCondition::Epochs(config.epochs))
        .log_interval(Some(config.log_interval))
        .momentum(config.momentum)
        .rate(config.rate)
        .go();
    net
}

fn evaluate(net: NN, data: &[(Vec<f64>, Vec<f64>)]) {
    data.iter().for_each(|(inputs, outputs)| {
        let results = net.run(&inputs.to_vec());
        let rounded_results = results.iter().map(|node| node.round()).collect::<Vec<f64>>();
        let sum_of_squares: f64 = results.iter()
            .zip(&rounded_results)
            .map(|(r, rr)| (*r - *rr) * (*r - *rr))
            .sum();
        let standard_deviation = (sum_of_squares/results.len() as f64).sqrt();

        let variance = sum_of_squares / 9.;
        println!("Result             : {:?}", &results);
        println!("Rounded Result     : {:?}", &rounded_results);
        println!("outputs            : {:?}", &outputs);
        println!("sum of sqrs e      : {:?}", &sum_of_squares);
        println!("variance           : {:?}", &variance);
        println!("standard deviation : {:?}", &standard_deviation);
        println!();

        let x: i64 = rounded_results.iter().zip(outputs).into_iter().map(|(r, o)| (*r - *o) as i64).sum();
        assert_eq!(x, 0);
    });
}
