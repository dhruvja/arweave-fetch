pub use std::{
    collections::HashMap,
    env,
    fs::File,
    io::Write,
    thread::{self, JoinHandle},
};
extern crate base64;

const DIFFERENCE: usize = 262144; // 256Kb in bytes
const URL: &str = "https://arweave.net/";

pub fn get_args(mut args: impl Iterator<Item = String>) -> Result<(String, String), &'static str> {
    args.next();

    let tx_id = match args.next() {
        Some(t) => t,
        None => return Err("Didnt get the query string"),
    };

    let file_name = match args.next() {
        Some(t) => t,
        None => return Err("Didnt get the file path string"),
    };

    Ok((tx_id, file_name))
}

pub fn get_size_and_offset(tx_id: &str) -> (usize, usize) {
    let offset_endpoint = format!("{}tx/{}/offset", URL, tx_id);
    let resp = reqwest::blocking::get(offset_endpoint)
        .unwrap()
        .json::<HashMap<String, String>>()
        .unwrap();
    let size = resp.get("size").unwrap().parse::<usize>().unwrap();
    let offset = resp.get("offset").unwrap().parse::<usize>().unwrap();
    (size, offset)
}

pub fn get_chunks(mut offset: usize, total_chunks: usize, total_threads: usize) -> Vec<u8> {
    // Will store the information returned from different threads
    let mut handles = Vec::new();
    let mut decoded_chunk_data = Vec::new();

    // This value gets decreased gradually until it reaches 0
    // This is used to split the values almost equally between all the threads
    let mut current_calls = total_chunks;

    let mut end = 0;

    for id in 0..total_threads {
        let diff = current_calls / (total_threads - id);
        let start = end;
        end = start + diff;
        current_calls = current_calls - (current_calls / (total_threads - id));
        let handle = thread::spawn(move || {
            let mut decoded_chunk_data: Vec<u8> = Vec::new();
            if id == (total_threads - 1) {
                end = total_chunks + 1;
            }
            offset = offset + (DIFFERENCE) * start;
            for _i in start..end {
                let buff = fetch_chunks(offset, (id + 1).try_into().unwrap());
                offset = offset + buff.len();
                decoded_chunk_data = [decoded_chunk_data, buff].concat();
            }
            decoded_chunk_data
        });
        handles.push(handle);
    }

    for handle in handles {
        // In this method, we wait for the threads to complete the process and data returned is stored in the variable
        let data = handle.join().unwrap();
        decoded_chunk_data = [decoded_chunk_data, data].concat();
    }
    decoded_chunk_data
}

#[tokio::main]
pub async fn fetch_chunks(offset: usize, id: u8) -> Vec<u8> {
    println!("thread{id}: {offset}");
    let chunks_endpoint = format!("{}chunk/{}", URL, offset);
    // This is a non blocking asynchronous http request
    let resp = reqwest::get(chunks_endpoint)
        .await
        .unwrap()
        .json::<HashMap<String, String>>()
        .await;
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
