use std::{env, fs::File, io::Write};
use arweave_fetch::{get_chunks, get_size_and_offset, get_first_chunk, get_args};


const DIFFERENCE: usize = 262144; // 256Kb in bytes 
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let (tx_id, file_name) = get_args(&args);
    let (size, mut offset) = get_size_and_offset(tx_id);
    println!("{}", size);

    let buff = get_first_chunk(offset);
    let total_calls = (size - buff.len())/DIFFERENCE;
    offset = offset - buff.len() + 1;

    let decoded_chunk_data = get_chunks(offset, total_calls, buff);

    let mut file = File::create(file_name).expect("Error encountered while creating file!");
    file.write_all(&decoded_chunk_data)
        .expect("Error while writing to file");

    Ok(())
}


