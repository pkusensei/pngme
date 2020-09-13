use std::{fs::OpenOptions, io::Write, str::FromStr};

use crate::{
    args::{Decode, Encode, Print, Remove},
    chunk::Chunk,
    chunk_type::ChunkType,
    png::Png,
    Result,
};

pub fn encode(e: Encode) -> Result<()> {
    let mut png = Png::from_file(&e.path)?;
    let chunk_type = ChunkType::from_str(&e.chunk_type)?;
    let data: Vec<u8> = e.message.into();
    let chunk = Chunk::new(chunk_type, data);
    png.append_chunk(chunk);
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&e.path)?;
    file.write_all(&png.as_bytes())?;
    Ok(())
}

pub fn decode(d: Decode) -> Result<()> {
    let png = Png::from_file(&d.path)?;
    match png.chunk_by_type(&d.chunk_type) {
        Some(c) => println!("Message is: {}", c.data_as_string()?),
        None => println!("Chunk with type \"{}\" does not exist.", d.chunk_type),
    };
    Ok(())
}

pub fn remove(r: Remove) -> Result<()> {
    let mut png = Png::from_file(&r.path)?;
    match png.remove_chunk(&r.chunk_type) {
        Ok(c) => {
            let mut file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&r.path)?;
            file.write_all(&png.as_bytes())?;
            println!("Removed chunk: {}", c);
        }
        Err(e) => println!("Cannot remove chunk \"{}\": {}", r.chunk_type, e),
    };
    Ok(())
}

pub fn print(p: Print) -> Result<()> {
    println!("{}", Png::from_file(&p.path)?);
    Ok(())
}
