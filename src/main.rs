mod range;
mod model;
mod encoder;
mod decoder;

use std::{io::{Cursor, Result, Write}, fs::File};

use bitbit::{BitWriter, BitReader, MSB};
use decoder::Decoder;
use encoder::Encoder;
use model::Model;

pub fn main() -> Result<()>  {
    let input = std::fs::read("./quijote-es.txt").unwrap();
    let mut output = File::create("output.bin")?;
    output.write_all(&encode(&input)?)?;

    let mut decoded = File::create("decoded.txt")?;
    let input_decoder = std::fs::read("./output.bin").unwrap();
    decoded.write_all(&decode(&input_decoder)?)?;

    Ok(())
}

fn encode(data: &[u8]) -> Result<Vec<u8>> {
    let mut model = Model::new(256, model::EOF::EndAddOne);

    let compressed = Cursor::new(vec![]);
    let mut compressed_writer = BitWriter::new(compressed);

    let mut encoder = Encoder::new(63);

    for &sym in data {
        encoder.encode(sym as u32, &model, &mut compressed_writer)?;
        model.update_symbol(sym as u32);
    }

    encoder.encode(model.eof(), &model, &mut compressed_writer)?;
    encoder.flush(&mut compressed_writer)?;
    compressed_writer.pad_to_byte()?;

    Ok(compressed_writer.get_ref().get_ref().clone())
}

fn decode(data: &[u8]) -> Result<Vec<u8>> {
    let mut model = Model::new(256, model::EOF::EndAddOne);

    let mut input_reader = BitReader::<_, MSB>::new(data);
    let mut decoder = Decoder::new(63);
    let mut decompressed_data = vec![];

    while !decoder.finished() {
        let sym = decoder.decode(&model, &mut input_reader)?;
        model.update_symbol(sym);
        decompressed_data.push(sym as u8);
    }

    decompressed_data.pop(); // remove the EOF

    Ok(decompressed_data)
}
