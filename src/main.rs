use arweave_fetch::{get_args, get_chunks, get_size_and_offset};
use std::{env, fs::File, io::Write};

const DIFFERENCE: usize = 262144; // 256Kb in bytes
const THREADS: usize = 20; // Number of threads for parallel processing
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let (tx_id, file_name) = get_args(&args);
    let (size, mut offset) = get_size_and_offset(tx_id);
    println!("{}", size);

    let last_byte_offset = offset.clone();
    offset = offset - size + 1;
    let total_chunks = (last_byte_offset - offset) / DIFFERENCE;
    println!("total chunks: {}", total_chunks);

    // In this function, the total number of chunks are calculated and divided equally between the threads. Each thread would fetch the chunks in the specified range
    // Once the fetching is done, the data is concatenated and returned
    let decoded_chunk_data = get_chunks(offset, total_chunks, THREADS);
    // The length of decoded chunk data should be same as the size of the file fetch from offset endpoint
    println!("{}", decoded_chunk_data.len());

    // A new file is created if it doesnt exist
    let mut file = File::create(file_name).expect("Error encountered while creating file!");

    // The data chunk is written in to a file
    file.write_all(&decoded_chunk_data)
        .expect("Error while writing to file");

    Ok(())
}
