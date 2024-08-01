use crate::error::Error;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use curl::easy::Easy;

use crate::cmd::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::png::Chunk;
use crate::png::ChunkType;
use crate::png::Png;
use crate::Result;

use std::fs::File;
use std::io::Write;

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> Result<()> {
    let chunk_type = ChunkType::from_str(&args.chunk_type)?;
    let chunk = Chunk::new(chunk_type, args.mess.into());

    let png_file_path = if let Some(file_path) = &args.in_file_path {
        Ok(file_path.clone())
    } else {
        if let Some(url) = &args.url {
            // if !is_png_url(url) {
            //     return Err(Error::InvalidPNGURL(url.clone()));
            // }

            let mut easy_curl = Easy::new();
            easy_curl
                .url(&url)
                .map_err(|_| Error::CurlErr(url.clone()))?;

            let file_name = url.split('/').last().ok_or(Error::CurlErr(url.clone()))?;
            let mut download_png =
                File::create(file_name).map_err(|err| Error::FileWriteErr(err))?;
            easy_curl
                .write_function(move |data| {
                    download_png.write_all(data).unwrap();
                    Ok(data.len())
                })
                .map_err(|_| Error::CurlErr(url.clone()))?;
            easy_curl
                .perform()
                .map_err(|_| Error::CurlErr(url.clone()))?;

            Ok(PathBuf::from_str(file_name).unwrap())
        } else {
            Err(Error::MissingArg("Input PNG file or URL".to_string()))
        }
    }?;

    let mut png = Png::try_from_file(Path::new(&png_file_path))?;
    png.append_chunk(chunk);

    // if let Some(output_path) = args.out_file_path {
    //     png.to_file(Path::new(&output_path))?;
    // }
    png.to_file(Path::new(&args.out_file_path))?;

    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> Result<()> {
    let png = Png::try_from_file(Path::new(&args.in_file_path))?;
    if let Some(mess_chunk) = png.chunk_by_type(&args.chunk_type)? {
        let mess = mess_chunk.data_as_string()?;
        println!("The secret message: {mess}");
        Ok(())
    } else {
        Err(Error::NotFoundSecMess)
    }
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> Result<()> {
    let mut png = Png::try_from_file(Path::new(&args.in_file_path))?;
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
    let png = Png::try_from_file(Path::new(&args.in_file_path))?;
    for chunk in png.chunks() {
        println!("{chunk}");
    }
    Ok(())
}

/// Checks if the given URL links to a PNG file
/// by checking the extension part, the after-dot part.
fn is_png_url(url: &str) -> bool {
    return url.split('.').last().unwrap() == "png";
}

#[cfg(test)]
mod tests {
    use super::is_png_url;

    #[test]
    fn test_is_png_url() {
        assert!(is_png_url("https://abc.com/xyz/123.png"));
        assert!(!is_png_url("http://abc.com/xyz/qwe/890.jpg"));
        assert!(!is_png_url("https://abc.com/xyz/qwe/890.jpg.pNg"))
    }
}
