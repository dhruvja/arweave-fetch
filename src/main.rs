use serde_json::Value;
use std::{collections::HashMap, env, fs::File, io::Write, thread};
extern crate base64;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let tx_id = &args[1];
    let file_name = &args[2];
    let offset_endpoint = format!("https://arweave.net/tx/{}/offset", tx_id);
    println!("{}", offset_endpoint);
    let resp = reqwest::blocking::get(offset_endpoint)?.json::<HashMap<String, String>>()?;
    let size = resp.get("size").unwrap().parse::<usize>().unwrap();
    let mut offset = resp.get("offset").unwrap().parse::<usize>().unwrap();
    println!("{}", size);

    let mut total_chunk_data = "".to_owned();
    let mut decoded_chunk_data: Vec<u8> = Vec::new();

    let difference = 262144;

    let chunks_endpoint = format!("https://arweave.net/chunk/{}", offset);
    let resp = reqwest::blocking::get(chunks_endpoint)?.json::<HashMap<String, String>>()?;

    let chunks = &resp.get("chunk").unwrap()[..];
    let buff = base64::decode_config(chunks, base64::URL_SAFE_NO_PAD)?;
    let total_calls = (size - buff.len())/difference;
    println!("{}", total_calls);
    offset = offset - buff.len() + 1;

    let thread1 = thread::spawn(move || {
        let mut decoded_chunk_data: Vec<u8> = Vec::new();
        let end = total_calls/2;
        for i in 0..end {
            println!("thread1: {}", offset);
            let chunks_endpoint = format!("https://arweave.net/chunk/{}", offset);
            let resp = reqwest::blocking::get(chunks_endpoint).unwrap().json::<HashMap<String, String>>();
            let response = match resp {
                Ok(t) => t,
                Err(err) => {
                    eprintln!("thread1: {}", err);
                    continue;
                }
            };
            let chunks = &response.get("chunk").unwrap()[..];
            let buff = base64::decode_config(chunks, base64::URL_SAFE_NO_PAD).unwrap();
            offset = offset - buff.len() + 1;
            decoded_chunk_data = [decoded_chunk_data, buff].concat();
        }
        decoded_chunk_data
    });

    let thread2 = thread::spawn(move || {
        let mut decoded_chunk_data: Vec<u8> = Vec::new();
        let start = total_calls/2;
        offset = offset - (difference-1)*start;
        for i in start..(total_calls+1) {
            println!("thread2: {}", offset);
            let chunks_endpoint = format!("https://arweave.net/chunk/{}", offset);
            let resp = reqwest::blocking::get(chunks_endpoint).unwrap().json::<HashMap<String, String>>();
            let response = match resp {
                Ok(t) => t,
                Err(err) => {
                    eprintln!("thread2: {}", err);
                    continue;
                }
            };
            let chunks = &response.get("chunk").unwrap()[..];
            let buff = base64::decode_config(chunks, base64::URL_SAFE_NO_PAD).unwrap();
            offset = offset - buff.len() + 1;
            decoded_chunk_data = [decoded_chunk_data, buff].concat();
        }
        decoded_chunk_data
    });

    let thread1_data = thread1.join().expect("thread panicked");
    let thread2_data = thread2.join().expect("thread panicked");

    decoded_chunk_data = [decoded_chunk_data,buff, thread1_data, thread2_data].concat();

    println!("{}", decoded_chunk_data.len());

    let mut file = File::create(file_name).expect("Error encountered while creating file!");
    file.write_all(&decoded_chunk_data)
        .expect("Error while writing to file");

    Ok(())
}
