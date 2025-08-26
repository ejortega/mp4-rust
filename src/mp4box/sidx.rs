use crate::mp4box::*;
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize)]
pub struct SidxBox {
    pub version: u8,
    pub flags: u32,
    pub reference_id: u32,
    pub timescale: u32,
    pub earliest_presentation_time: u64,
    pub first_offset: u64,

    pub subseg_durations: Vec<u32>,
}

impl SidxBox {
    pub fn get_type(&self) -> BoxType {
        BoxType::SidxBox
    }

    pub fn total_duration(&self) -> u32 {
        self.subseg_durations.iter().sum()
    }

    pub fn timescale(&self) -> u32 {
        self.timescale
    }

    pub fn get_size(&self) -> u64 {
        let sub_hdr_sz = match self.version {
            0 => 8,
            _ => 16,
        };

        HEADER_SIZE + HEADER_EXT_SIZE + 4 + 8 + sub_hdr_sz + (self.subseg_durations.len() as u64 * 12)
    }
}

impl Mp4Box for SidxBox {
    fn box_type(&self) -> BoxType {
        self.get_type()
    }

    fn box_size(&self) -> u64 {
        self.get_size()
    }

    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self).unwrap())
    }

    fn summary(&self) -> Result<String> {
        Ok(String::new())
    }
}

impl<R: Read + Seek> ReadBox<&mut R> for SidxBox {
    fn read_box(reader: &mut R, size: u64) -> Result<Self> {
        let start = box_start(reader)?;

        let version = reader.read_u8()?;
        let flags = reader.read_u24::<BigEndian>()?;

        let reference_id = match version {
            0 => reader.read_u16::<BigEndian>()? as u32,
            _ => reader.read_u32::<BigEndian>()?,
        };

        let timescale = match version {
            0 => reader.read_u16::<BigEndian>()? as u32,
            _ => reader.read_u32::<BigEndian>()?,
        };

        let earliest_presentation_time = reader.read_u64::<BigEndian>()?;
        let first_offset = reader.read_u64::<BigEndian>()?;
        let _reserved = reader.read_u16::<BigEndian>()?;
        let ref_count = reader.read_u16::<BigEndian>()?;

        let mut subseg_durations = Vec::new();
        for _ in 1..=ref_count {
            let _ = reader.read_u32::<BigEndian>()?;
            let duration = reader.read_u32::<BigEndian>()?;

            let _ = reader.read_u32::<BigEndian>()?;

            subseg_durations.push(duration);
        }

        skip_bytes_to(reader, start + size)?;

        Ok(Self {
            version,
            flags,
            reference_id,
            timescale,
            earliest_presentation_time,
            first_offset,
            subseg_durations
        })
    }
}

impl<W: Write> WriteBox<&mut W> for SidxBox {
    fn write_box(&self, writer: &mut W) -> Result<u64> {
        let size = self.box_size();
        BoxHeader::new(self.box_type(), size).write(writer)?;

        write_box_header_ext(writer, self.version, self.flags)?;
        match self.version {
            0 => writer.write_u16::<BigEndian>(self.reference_id as u16)?,
            _ => writer.write_u32::<BigEndian>(self.reference_id)?,
        }

        match self.version {
            0 => writer.write_u16::<BigEndian>(self.timescale as u16)?,
            _ => writer.write_u32::<BigEndian>(self.timescale)?,
        }

        writer.write_u64::<BigEndian>(self.earliest_presentation_time)?;
        writer.write_u64::<BigEndian>(self.first_offset)?;
        writer.write_u16::<BigEndian>(0)?;
        // writer.write_u16::<BigEndian>(self.rest.len() as u16)?;
        for _ in &self.subseg_durations {
            // NOTE: Todo
        }

        Ok(size)
    }
}
