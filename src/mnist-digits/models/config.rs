use serde_json::Value;

pub struct Config {
    pub layers: Vec<u32>,
    pub epochs: u32,
    pub log_interval: u32,
    pub momentum: f64,
    pub rate: f64,
}

impl Config {
    pub fn from(json: &str) -> Option<Config> {
        let v: Value = match serde_json::from_str(json) {
            Ok(parsed) => parsed,
            Err(_e) => panic!(format!("{:?}", _e))
        };

        let epochs = v["epochs"].as_u64()? as u32;
        let log_interval = v["log_interval"].as_u64()? as u32;
        let rate = v["rate"].as_f64()?;
        let momentum = v["momentum"].as_f64()?;
        // let layers = v["layers"]?;
        let layers: Vec<u32> = v["layers"].as_array().unwrap().iter().map(|v| v.as_u64().unwrap() as u32).collect();
        Some(Config { layers, epochs, log_interval, momentum, rate})
    }
}

#[test]
fn test() {
    let json = r#"
{
  "layers": [ 784, 56, 10 ],
  "epochs": 5,
  "log_interval": 1,
  "momentum": 0.1,
  "rate": 0.3
}
"#;
    let result = Config::from(json).unwrap();
    let v: Vec<u32> = vec![784, 56, 10];
    assert_eq!(result.layers, v);
    assert_eq!(result.epochs, 5);
    assert_eq!(result.log_interval, 1);
    assert_eq!(result.momentum, 0.1);
    assert_eq!(result.rate, 0.3);
}