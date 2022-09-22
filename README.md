# arweave-fetch

This is a simple program where the data is fetched from arweave in chunks if the size of data is more than 256Kb.

## How does it work

First the offset and size of the tx is fetched at `https://arweave.net/tx/{tx_id}/offset`.
Then the chunks are fetched at `https://arweave.net/chunk/{offset}`
The offset is subtracted by the size of the chunk and is continued until the last chunk is obtained.

## Run the program

- run `cargo build`
- run `cargo run -- tx_id file_name` .
  For Example:
  `cargo run -- OFJ0mBSQXy1ErQWUmQ1E5R-EBJhFCA6B_f9PhT7-Gpk document.png`
  
## Performance

Multithreading is used to speed up the transaction by the number of threads specified.
More the threads, the faster the data would be fetched. 
With multithreading, parallel processing can be achieved.
