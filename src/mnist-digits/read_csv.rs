use std::error::Error;
use std::path::Path;
use std::str::FromStr;

fn convert(path: &Path, has_header: bool) -> Result<Box<Vec<Vec<f64>>>, Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new().has_headers(has_header).from_path(path)?;
    let res = reader.records().into_iter().map(|s| {
        match s {
            Ok(string_record) => {
                string_record.iter().map(|a| (f64::from_str(a).unwrap()/255.)).collect::<Vec<f64>>()
            },
            Err(_e) => vec![]
        }
    }).collect();
    Ok(Box::new(res))
}

fn validate_file(filename: &str) -> Result<&Path, String> {
    let path = Path::new(filename);
    match path.is_file() {
        true => Ok(path),
        false => Err(From::from("file does not exist"))
    }
}

pub fn read_csv(filename: &str, has_header: bool) -> Result<Vec<Vec<f64>>, Box<dyn Error>> {
    let path = validate_file(filename)?;
    let res = convert(path, has_header)?;
    Ok(res.to_vec())
}
//
// pub fn split(data: Vec<Vec<f64>>) -> Vec<(Box<Vec<f64>>, f64)> {
//     let vec = data.to_vec().iter().map(|row: &mut Vec<f64>| {
//         let digit = row.remove(0);
//         (row.to_vec(), digit)
//     }).collect::<Vec<(Box<Vec<f64>>, f64)>>();
//     vec
// }
