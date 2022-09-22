pub use std::{collections::HashMap, env, fs::File, io::Write, thread::{self, JoinHandle}};
extern crate base64;


const DIFFERENCE: usize = 262144; // 256Kb in bytes 

pub fn get_size_and_offset(tx_id: &str) -> (usize, usize) {
    let offset_endpoint = format!("https://arweave.net/tx/{}/offset", tx_id);
    println!("{}", offset_endpoint);
    let resp = reqwest::blocking::get(offset_endpoint).unwrap().json::<HashMap<String, String>>().unwrap();
    let size = resp.get("size").unwrap().parse::<usize>().unwrap();
    let offset = resp.get("offset").unwrap().parse::<usize>().unwrap();
    (size, offset)
}

pub fn get_first_chunk(offset: usize) -> Vec<u8> {
    let chunks_endpoint = format!("https://arweave.net/chunk/{}", offset);
    let resp = reqwest::blocking::get(chunks_endpoint).unwrap().json::<HashMap<String, String>>().unwrap();

    let chunks = &resp.get("chunk").unwrap()[..];
    let buff = base64::decode_config(chunks, base64::URL_SAFE_NO_PAD).unwrap();
    buff
}

pub fn get_chunks(mut offset: usize, total_calls: usize, mut decoded_chunk_data: Vec<u8>) -> Vec<u8> {
    let total_threads = 27; 

    let mut handles = Vec::new();

    let mut current_calls = total_calls;

    let mut start = 0;
    let mut end = 0;

    for id in 0..total_threads {
        let diff = current_calls/(total_threads - id);
        start = end;
        end = start + diff;
        current_calls = current_calls - (current_calls/(total_threads - id));
        let handle = thread::spawn(move || {
            let mut decoded_chunk_data: Vec<u8> = Vec::new();
            if id == (total_threads - 1) {
                end = total_calls + 1;
            }
            // println!("thread{id}: {start} {end} {current_calls}");
            offset = offset - (DIFFERENCE-1)*start;
            for _i in start..end {
                let buff = fetch_chunks(offset, (id+1).try_into().unwrap());
                offset = offset - buff.len() + 1;
                decoded_chunk_data = [decoded_chunk_data, buff].concat();
            }
            decoded_chunk_data 
        });
        handles.push(handle);
    }

    for handle in handles{
        let data = handle.join().unwrap();
        decoded_chunk_data = [decoded_chunk_data, data].concat();
    }
    decoded_chunk_data
}

#[tokio::main]
pub async fn fetch_chunks(offset: usize, id: u8) -> Vec<u8> {
    println!("thread{id}: {offset}");
    let chunks_endpoint = format!("https://arweave.net/chunk/{}", offset);
    let resp = reqwest::get(chunks_endpoint).await.unwrap().json::<HashMap<String, String>>().await;
    let response = match resp {
        Ok(t) => t,
        Err(err) => {
            eprintln!("thread{id}: {}", err);
            panic!();
        }
    };
    let chunks = &response.get("chunk").unwrap()[..];
    let buff = base64::decode_config(chunks, base64::URL_SAFE_NO_PAD).unwrap(); 
    buff
}