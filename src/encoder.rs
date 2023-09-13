use crate::model::Model;
use crate::range::Range;
use bitbit::BitWriter;
use std::io::{Error, Write};

pub struct Encoder {
    _precision_bits: u8,
    pending_bits: u32,
    range: Range
}

impl Encoder {
    pub fn new(precision: u8) -> Self {
        Self {
            _precision_bits: precision,
            pending_bits: 0,
            range: Range::new(precision)
        }
    }

    pub fn encode<T: Write>(&mut self, symbol: u32, model: &Model, output: &mut BitWriter<T>,) -> Result<(), Error> {
        let low_high: (u64, u64) = self.range.calculate_range(symbol, model);
        self.range.update_range(low_high);

        while  self.range.in_upper_half() || self.range.in_bottom_half() {
            if self.range.in_upper_half() {
                self.range.scale_upper_half();
                self.write(true, output)?;
            } else {
                self.range.scale_bottom_half();
                self.write(false, output)?;
            }
        }

        //Now MSB could be 01 or 10 -> Middle half
        while self.range.in_middle_half() {
            self.pending_bits += 1;
            self.range.scale_middle_half();
        }

        Ok(())
    }

    pub fn write<T: Write>(&mut self, bit: bool, output: &mut BitWriter<T>) -> Result<(), Error> {
        output.write_bit(bit)?;

        while self.pending_bits > 0 {
            output.write_bit(!bit)?;
            self.pending_bits -= 1;
        }

        Ok(())
    }

    pub fn flush<T: Write>(&mut self, output: &mut BitWriter<T>) -> Result<(), Error> {
        self.pending_bits += 1;

        if self.range.in_bottom_quarter() {
            self.write(false, output)?;
        } else {
            self.write(true, output)?;
        }

        Ok(())
    }
} 

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use bitbit::BitWriter;

    use super::Encoder;
    use crate::{Model, model::EOF};

    #[test]
    fn e2e() {
        let mut encoder = Encoder::new(30);
        let mut source_model = Model::new(9, EOF::EndAddOne);
        let mut output = Cursor::new(vec![]);
        let mut out_writer = BitWriter::new(&mut output);
        let to_encode: [u32; 5] = [7, 2, 2, 2, 7];
        for x in &to_encode {
            encoder.encode(*x, &source_model, &mut out_writer).unwrap();
            source_model.update_symbol(*x);
        }
        encoder
            .encode(source_model.eof(), &source_model, &mut out_writer)
            .unwrap();
        encoder.flush(&mut out_writer).unwrap();
        out_writer.pad_to_byte().unwrap();
        assert_eq!(output.get_ref(), &[184, 96, 208]);
    }
}
