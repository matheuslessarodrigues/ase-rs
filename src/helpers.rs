use std::io::{self, Read, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
pub use fixed::{types::extra::U16, FixedU32};

pub fn read_bytes<R>(read: &mut R, length: usize) -> io::Result<Vec<u8>>
where
    R: Read,
{
    let mut bytes = Vec::with_capacity(length);
    bytes.resize(length, 0);
    read.read_exact(&mut bytes[..])?;
    Ok(bytes)
}

pub fn read_fixed_point_number_16_16<R>(
    read: &mut R,
) -> io::Result<FixedU32<U16>>
where
    R: Read,
{
    let bits = read.read_u32::<LittleEndian>()?;
    Ok(FixedU32::<U16>::from_bits(bits))
}

pub fn write_fixed_point_number_16_16<W>(
    wtr: &mut W,
    number: &FixedU32<U16>,
) -> io::Result<()>
where
    W: Write,
{
    let bits = number.to_bits();
    wtr.write_u32::<LittleEndian>(bits)?;
    Ok(())
}

pub fn read_string<R>(read: &mut R) -> io::Result<String>
where
    R: Read,
{
    let length = read.read_u16::<LittleEndian>()? as usize;
    let bytes = read_bytes(read, length)?;
    String::from_utf8(bytes)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

pub fn write_string<W>(wtr: &mut W, string: &str) -> io::Result<()>
where
    W: Write,
{
    wtr.write_u16::<LittleEndian>(string.len() as u16)?;
    wtr.write(&string.as_bytes())?;
    Ok(())
}
