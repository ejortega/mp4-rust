use std::io::{Read, Seek, Write};
use serde::{Serialize};

use crate::mp4box::*;

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct TfdtBox {
    pub version: u8,
    pub flags: u32,
    pub base_media_decode_time: u64
}

impl Mp4Box for TfdtBox {
    fn box_type(&self) -> BoxType {
        BoxType::TfdtBox
    }

    fn box_size(&self) -> u64 {
        HEADER_SIZE + HEADER_EXT_SIZE + 8
    }
    
    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self).unwrap())
    }

    fn summary(&self) -> Result<String> {
        Ok(String::new())
    }
}

impl<R: Read + Seek> ReadBox<&mut R> for TfdtBox {
    fn read_box(reader: &mut R, size: u64) -> Result<Self> {
        let start = box_start(reader)?;

        let (version, flags) = read_box_header_ext(reader)?;
        let base_media_decode_time = match version {
            0 => reader.read_u32::<BigEndian>()? as u64,
            _ => reader.read_u64::<BigEndian>()?,
        };

        skip_bytes_to(reader, start + size)?;

        Ok(TfdtBox {
            version,
            flags,
            base_media_decode_time,
        })
            
    }
}

impl<W: Write> WriteBox<&mut W> for TfdtBox {
    fn write_box(&self, writer: &mut W) -> Result<u64> {
        let size = self.box_size();
        BoxHeader::new(self.box_type(), size).write(writer)?;

        write_box_header_ext(writer, self.version, self.flags)?;
        writer.write_u64::<BigEndian>(self.base_media_decode_time)?;

        Ok(size)
        
    }
}
