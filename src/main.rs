use std::{collections::HashMap, env, fs::File, io::Write, thread::{self, JoinHandle}};
extern crate base64;
use arweave_fetch::{get_chunks, get_size_and_offset, get_first_chunk};


const DIFFERENCE: usize = 262144; // 256Kb in bytes 
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let tx_id = &args[1];
    let file_name = &args[2];
    let (size, mut offset) = get_size_and_offset(tx_id);
    println!("{}", size);

    let mut decoded_chunk_data: Vec<u8> = Vec::new();

    let buff = get_first_chunk(offset);
    let total_calls = (size - buff.len())/DIFFERENCE;
    offset = offset - buff.len() + 1;

    decoded_chunk_data = get_chunks(offset, total_calls, buff);

    let mut file = File::create(file_name).expect("Error encountered while creating file!");
    file.write_all(&decoded_chunk_data)
        .expect("Error while writing to file");

    Ok(())
}


