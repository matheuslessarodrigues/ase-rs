pub use fixed::{types::extra::U16, FixedU32};

use std::io::{self, Read, Seek, SeekFrom, Write};

use bitflags::bitflags;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::helpers::{
    read_fixed_point_number_16_16, write_fixed_point_number_16_16,
};

bitflags! {
    pub struct Flags: u32 {
        const PreciseBounds = 1;
    }
}

#[derive(Debug)]
pub struct CelExtraChunk {
    pub flags: Flags,
    pub precise_x_position: FixedU32<U16>,
    pub precise_y_position: FixedU32<U16>,
    pub width: FixedU32<U16>,
    pub height: FixedU32<U16>,
}

impl CelExtraChunk {
    pub fn precise_x_position(&self) -> f32 {
        self.precise_x_position.to_num::<f32>()
    }

    pub fn precise_y_position(&self) -> f32 {
        self.precise_y_position.to_num::<f32>()
    }

    pub fn width(&self) -> f32 {
        self.width.to_num::<f32>()
    }

    pub fn height(&self) -> f32 {
        self.height.to_num::<f32>()
    }

    pub fn from_read<R>(read: &mut R) -> io::Result<Self>
    where
        R: Read + Seek,
    {
        let flags = Flags::from_bits_truncate(read.read_u32::<LittleEndian>()?);
        let precise_x_position = read_fixed_point_number_16_16(read)?;
        let precise_y_position = read_fixed_point_number_16_16(read)?;
        let width = read_fixed_point_number_16_16(read)?;
        let height = read_fixed_point_number_16_16(read)?;
        read.seek(SeekFrom::Current(16))?;

        Ok(Self {
            flags,
            precise_x_position,
            precise_y_position,
            width,
            height,
        })
    }

    pub fn write<W>(&self, wtr: &mut W) -> io::Result<()>
    where
        W: Write + Seek,
    {
        wtr.write_u32::<LittleEndian>(self.flags.bits)?;
        write_fixed_point_number_16_16(wtr, &self.precise_x_position)?;
        write_fixed_point_number_16_16(wtr, &self.precise_y_position)?;
        write_fixed_point_number_16_16(wtr, &self.width)?;
        write_fixed_point_number_16_16(wtr, &self.height)?;
        wtr.seek(SeekFrom::Current(16))?;
        Ok(())
    }
}
