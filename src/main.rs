use serde_json::Value;
use std::{collections::HashMap, env};

struct Offset {
    size: u64,
    offset: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args:Vec<String> = env::args().collect();
    let tx_id = &args[1];
    // let tx_id = match &args[0] {
    //     Some(t) => t,
    //     None => panic!(),
    // };
    let offset_endpoint = format!("https://arweave.net/tx/{}/offset", tx_id);
    println!("{}", offset_endpoint);
    let resp = reqwest::blocking::get(offset_endpoint)?.json::<HashMap<String, String>>()?;
    let size = resp.get("size").unwrap().parse::<u64>().unwrap();
    let offset = resp.get("size").unwrap().parse::<u64>().unwrap();

    println!("{:?}", resp);
    Ok(())
}
