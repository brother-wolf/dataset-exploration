use std::error::Error;
use std::str::FromStr;

pub fn read_csv(filename: &str, has_header: bool) -> Result<Box<Vec<Vec<f64>>>, Box<dyn Error>> {
    // if !does_file_exist(filename) { panic!("file {} does not exist", filename)}
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(has_header)
        .from_path(filename)?;

    Ok(Box::new(reader.records().into_iter().map(|s| {
        match s {
            Ok(string_record) => {
                string_record.iter()
                    .map(|a| (f64::from_str(a).unwrap()))
                    .collect::<Vec<f64>>()
            },
            Err(_e) => vec![]
        }
    }).collect()))
}

fn digit_as_vec_data(digit: f64) -> Result<Vec<f64>, String> {
    if digit < 0.0 || digit > 9.9 {return Err(String::from("digit out of range 0..9"))}
    let mut prototype = vec![1.,0.,0.,0.,0.,0.,0.,0.,0.,0.];
    prototype.swap(0, digit as usize);
    Ok(prototype)
}

fn replace(row: &mut Vec<f64>) -> (Vec<f64>, Vec<f64>) {
    let digit = row.remove(0);
    let digits = digit_as_vec_data(digit);
    (normalise(row), digits.unwrap())
}

fn normalise(row: &mut Vec<f64>) -> Vec<f64> {
    row.to_vec().iter_mut().map(|x| *x / 255.).collect()//.to_owned()
}

pub fn split_and_normalise(data: &mut Vec<Vec<f64>>) -> Vec<(Vec<f64>, Vec<f64>)> {
    data.iter_mut()
        .map(|row| replace(row))
        .collect::<Vec<(Vec<f64>, Vec<f64>)>>()
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
fn digit_as_vec_data_test() {
    assert_eq!(digit_as_vec_data(9.).unwrap(), vec![0., 0. , 0. , 0. , 0. , 0. , 0. , 0. , 0. , 1. ]);
}
