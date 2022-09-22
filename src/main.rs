use serde_json::Value;
use std::{collections::HashMap, env, fs::File, io::Write};
extern crate base64;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let tx_id = &args[1];
    let file_name = &args[2];
    let offset_endpoint = format!("https://arweave.net/tx/{}/offset", tx_id);
    println!("{}", offset_endpoint);
    let resp = reqwest::blocking::get(offset_endpoint)?.json::<HashMap<String, String>>()?;
    let size = resp.get("size").unwrap().parse::<u64>().unwrap();
    let offset = resp.get("offset").unwrap().parse::<u64>().unwrap();

    let chunks_endpoint = format!("https://arweave.net/chunk/{}", offset);
    let resp = reqwest::blocking::get(chunks_endpoint)?.json::<HashMap<String, String>>()?;
    
    println!("{}", offset);

    let chunks = &resp.get("chunk").unwrap()[..];
    let buff = base64::decode_config(chunks, base64::URL_SAFE_NO_PAD)?;

    let mut file = File::create(file_name).expect("Error encountered while creating file!");
    file.write_all(&buff)
        .expect("Error while writing to file");

    Ok(())
}
