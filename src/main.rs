use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;

mod binops;
mod compressor;
mod decompressor;
mod huffman;
mod utils;

fn main() -> std::io::Result<()> {
    let args = utils::parse_args(&env::args().collect())?;

    println!(
        "Mode: {}\nInput: {}\nOutput{}\n",
        args.mode, args.file_in, args.file_out
    );

    let file_in = File::open(Path::new(&args.file_in))?;
    let mut reader = BufReader::new(file_in);

    let initial_tree = compressor::tree::init(&mut reader)?;
    let huffman_tree = compressor::tree::build(initial_tree);

    if let Some(root_node) = huffman_tree {
        let codes = compressor::calc_codes(&root_node);
        let stream_len = utils::stream_length(&mut reader)?;

        reader.seek(SeekFrom::Start(0))?;

        while utils::stream_current_position(&mut reader)? != stream_len {
            let chunk_size = std::cmp::min(stream_len, 1024);

            let mut chunk = vec![0u8; chunk_size as usize];
            reader.read_exact(&mut chunk)?;

            let res = compressor::compress(&chunk, &codes);
            println!(">> Compressed stream >>");
            for entry in &res.0 {
                print!("{:b} ", entry);
            }
            println!("\n<< Compressed stream <<");

            println!(">> Decompressed stream >>");
            println!("{}", decompressor::decompress(&res, &root_node));
            println!("<< Decompressed stream <<\n");

            println!(
                "Message size: {} Compressed size: {}",
                chunk.len(),
                res.0.len()
            );
        }
    } else {
        println!("Empty tree.");
    }

    Ok(())
}
