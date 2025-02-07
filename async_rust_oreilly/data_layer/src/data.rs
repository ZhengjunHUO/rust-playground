use std::io::{self, Cursor, Read, Write};

#[derive(Debug)]
pub struct Payload {
    pub slot1: u32,
    pub slot2: u16,
    pub slot3: String,
}

impl Payload {
    pub fn serialize(&self) -> io::Result<Vec<u8>> {
        let mut buf = Vec::new();

        buf.write(&self.slot1.to_ne_bytes())?;
        buf.write(&self.slot2.to_ne_bytes())?;

        let slot3_len = self.slot3.len() as u32;
        buf.write(&slot3_len.to_ne_bytes())?;
        buf.extend_from_slice(self.slot3.as_bytes());

        Ok(buf)
    }

    pub fn deserialize(cursor: &mut Cursor<&[u8]>) -> io::Result<Payload> {
        let mut slot1_buf = [0u8; 4];
        let mut slot2_buf = [0u8; 2];
        cursor.read_exact(&mut slot1_buf)?;
        cursor.read_exact(&mut slot2_buf)?;
        let slot1 = u32::from_ne_bytes(slot1_buf);
        let slot2 = u16::from_ne_bytes(slot2_buf);

        let mut len_buf = [0u8; 4];
        cursor.read_exact(&mut len_buf)?;
        let string_len = u32::from_ne_bytes(len_buf) as usize;

        let mut slot3_buf = vec![0u8; string_len];
        cursor.read_exact(&mut slot3_buf)?;
        let slot3 = String::from_utf8(slot3_buf)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Contains invalid UTF-8"))?;
        Ok(Payload {
            slot1,
            slot2,
            slot3,
        })
    }
}
