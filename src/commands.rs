use std::io::ErrorKind;
use std::path::Path;
use std::str::FromStr;

use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::Result;

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> Result<()> {
    let chunk_type = ChunkType::from_str(&args.chunk_type)?;
    let chunk = Chunk::new(chunk_type, args.mess.into());
    let mut png = Png::from_file(Path::new(&args.in_file_path))?;
    png.append_chunk(chunk);

    if let Some(output_path) = args.out_file_path {
        png.to_file(Path::new(&output_path))?;
    }

    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> Result<()> {
    let png = Png::from_file(Path::new(&args.in_file_path))?;
    if let Some(mess_chunk) = png.chunk_by_type(&args.chunk_type) {
        let mess = mess_chunk.data_as_string()?;
        println!("The secret message: {mess}");
        Ok(())
    } else {
        Err(Box::new(std::io::Error::new(
            ErrorKind::InvalidInput,
            "Cannot find the secret message",
        )))
    }
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> Result<()> {
    let mut png = Png::from_file(Path::new(&args.in_file_path))?;
    match png.remove_chunk(&args.chunk_type) {
        Ok(_) => {
            png.to_file(Path::new(&args.in_file_path))?;
            Ok(())
        }
        Err(err) => Err(err),
    }
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> Result<()> {
    let png = Png::from_file(Path::new(&args.in_file_path))?;
    for chunk in png.chunks() {
        println!("{chunk}");
    }
    Ok(())
}
