use std::error::Error;

#[macro_use]
extern crate serde_json;

fn load_json(filename: &str) -> Result<serde_json::Value, Box<Error>>{
    println!("load: {}", filename);
    let file = std::fs::File::open(filename)?;
    let config = serde_json::from_reader(&file)?;
    Ok(config)
}

fn main() {
    // load_json().expect("need to be able to load json");
    println!("please run: cargo test -- --nocapture");
}

#[test]
fn test_load_json() {
    let filename = std::env::current_dir()
        .unwrap()
        .join("src/test.json");
    let config = load_json(filename.to_str().unwrap()).expect("need to load json correctly");
    println!("{}", config);
    assert_eq!(config, json!({
        "input": "in128.dat",
        "output": "output.dat",
        "num": 12
    }
    ));
}
