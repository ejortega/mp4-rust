use crate::mp4box::*;

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize)]
pub struct MdatBox {
    pub data: Vec<u8>,
}

impl Mp4Box for MdatBox {
    fn box_type(&self) -> BoxType {
        BoxType::MdatBox
    }

    fn box_size(&self) -> u64 {
        HEADER_SIZE + self.data.len() as u64
    }

    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self).unwrap())
    }

    fn summary(&self) -> Result<String> {
        Ok(String::new())
    }
}

impl<R: Read + Seek> ReadBox<&mut R> for MdatBox {
    fn read_box(reader: &mut R, size: u64) -> Result<Self> {
        let start = box_start(reader)?;

        let mut data = Vec::with_capacity(size as usize);
        reader.read_exact(&mut data)?;

        skip_bytes_to(reader, start + size)?;

        Ok(Self {
            data
        })
    }
}

impl<W: Write> WriteBox<&mut W> for MdatBox {
    fn write_box(&self, writer: &mut W) -> Result<u64> {
        let size = self.box_size();
        BoxHeader::new(self.box_type(), size).write(writer)?;

        writer.write_all(&self.data)?;

        Ok(size)
    }
}
