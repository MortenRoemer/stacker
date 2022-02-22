use std::io;

pub trait Pack {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize>;

    fn pack_to_vec(&self) -> io::Result<Vec<u8>> {
        let mut buffer = Vec::new();
        self.pack_into(&mut buffer)?;
        Ok(buffer)
    }
}

impl Pack for bool {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let value = match self {
            true => 0x00,
            false => 0xFF,
        };
        let buffer = [value];
        writer.write(&buffer)
    }
}

impl Pack for u8 {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let buffer = [*self];
        writer.write(&buffer)
    }
}

impl Pack for u16 {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let buffer = self.to_be_bytes();
        writer.write(&buffer)
    }
}

impl Pack for u32 {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let buffer = self.to_be_bytes();
        writer.write(&buffer)
    }
}

impl Pack for u64 {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let buffer = self.to_be_bytes();
        writer.write(&buffer)
    }
}

impl Pack for u128 {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let buffer = self.to_be_bytes();
        writer.write(&buffer)
    }
}

impl Pack for i16 {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let buffer = self.to_be_bytes();
        writer.write(&buffer)
    }
}

impl Pack for i32 {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let buffer = self.to_be_bytes();
        writer.write(&buffer)
    }
}

impl Pack for i64 {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let buffer = self.to_be_bytes();
        writer.write(&buffer)
    }
}

impl Pack for i128 {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let buffer = self.to_be_bytes();
        writer.write(&buffer)
    }
}

impl Pack for f32 {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let buffer = self.to_be_bytes();
        writer.write(&buffer)
    }
}

impl Pack for f64 {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let buffer = self.to_be_bytes();
        writer.write(&buffer)
    }
}

impl Pack for str {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let buffer = self.as_bytes();
        let len = buffer.len() as u32;
        let written = len.pack_into(writer)?;
        writer.write(buffer).map(|x| written + x)
    }
}

impl<T: Pack> Pack for [T] {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let len = self.len() as u32;
        let mut written = len.pack_into(writer)?;

        for item in self.iter() {
            written += item.pack_into(writer)?;
        }

        Ok(written)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_bool() {
        let value = false;
        let bytes = value.pack_to_vec().unwrap();
        assert_eq!(bytes, [0xFF]);
    }

    #[test]
    fn pack_u8() {
        let value: u8 = 2;
        let bytes = value.pack_to_vec().unwrap();
        assert_eq!(bytes, [0x02]);
    }

    #[test]
    fn pack_u16() {
        let value: u16 = 2;
        let bytes = value.pack_to_vec().unwrap();
        assert_eq!(bytes, [0x00, 0x02]);
    }

    #[test]
    fn pack_u32() {
        let value: u32 = 2;
        let bytes = value.pack_to_vec().unwrap();
        assert_eq!(bytes, [0x00, 0x00, 0x00, 0x02]);
    }

    #[test]
    fn pack_u64() {
        let value: u64 = 2;
        let bytes = value.pack_to_vec().unwrap();
        assert_eq!(bytes, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02]);
    }

    #[test]
    fn pack_u128() {
        let value: u128 = 2;
        let bytes = value.pack_to_vec().unwrap();
        assert_eq!(
            bytes,
            [
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x02
            ]
        );
    }

    #[test]
    fn pack_i16() {
        let value: i16 = -1;
        let bytes = value.pack_to_vec().unwrap();
        assert_eq!(bytes, [0xFF, 0xFF]);
    }

    #[test]
    fn pack_i32() {
        let value: i32 = -1;
        let bytes = value.pack_to_vec().unwrap();
        assert_eq!(bytes, [0xFF, 0xFF, 0xFF, 0xFF]);
    }

    #[test]
    fn pack_i64() {
        let value: i64 = -1;
        let bytes = value.pack_to_vec().unwrap();
        assert_eq!(bytes, [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    }

    #[test]
    fn pack_i128() {
        let value: i128 = -1;
        let bytes = value.pack_to_vec().unwrap();
        assert_eq!(
            bytes,
            [
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF
            ]
        );
    }

    #[test]
    fn pack_f32() {
        let value: f32 = -1.0;
        let bytes = value.pack_to_vec().unwrap();
        assert_eq!(bytes, [0xBF, 0x80, 0x00, 0x00]);
    }

    #[test]
    fn pack_f64() {
        let value: f64 = -1.0;
        let bytes = value.pack_to_vec().unwrap();
        assert_eq!(bytes, [0xBF, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn pack_str() {
        let value = "abc";
        let bytes = value.pack_to_vec().unwrap();
        assert_eq!(bytes, [0x00, 0x00, 0x00, 0x03, 0x61, 0x62, 0x63]);
    }

    #[test]
    fn pack_array() {
        let value: [u8; 3] = [1, 2, 3];
        let bytes = value.pack_to_vec().unwrap();
        assert_eq!(bytes, [0x00, 0x00, 0x00, 0x03, 0x01, 0x02, 0x03]);
    }
}
