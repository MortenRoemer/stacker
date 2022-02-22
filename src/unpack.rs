use std::io;

pub trait Unpack {
    fn unpack_from(reader: &mut impl io::Read) -> io::Result<Self>
    where
        Self: Sized;

    fn unpack_from_vec(source: &mut &[u8]) -> io::Result<Self>
    where
        Self: Sized,
    {
        Self::unpack_from(source)
    }
}

impl Unpack for bool {
    fn unpack_from(reader: &mut impl io::Read) -> io::Result<Self> {
        let mut bytes = [0x00];
        reader.read_exact(&mut bytes)?;
        Ok(bytes[0] != 0xFF)
    }
}

impl Unpack for u8 {
    fn unpack_from(reader: &mut impl io::Read) -> io::Result<Self> {
        let mut bytes = [0x00];
        reader.read_exact(&mut bytes)?;
        Ok(bytes[0])
    }
}

impl Unpack for u16 {
    fn unpack_from(reader: &mut impl io::Read) -> io::Result<Self> {
        let mut bytes = [0x00; 2];
        reader.read_exact(&mut bytes)?;
        Ok(u16::from_be_bytes(bytes))
    }
}

impl Unpack for u32 {
    fn unpack_from(reader: &mut impl io::Read) -> io::Result<Self> {
        let mut bytes = [0x00; 4];
        reader.read_exact(&mut bytes)?;
        Ok(u32::from_be_bytes(bytes))
    }
}

impl Unpack for u64 {
    fn unpack_from(reader: &mut impl io::Read) -> io::Result<Self> {
        let mut bytes = [0x00; 8];
        reader.read_exact(&mut bytes)?;
        Ok(u64::from_be_bytes(bytes))
    }
}

impl Unpack for u128 {
    fn unpack_from(reader: &mut impl io::Read) -> io::Result<Self> {
        let mut bytes = [0x00; 16];
        reader.read_exact(&mut bytes)?;
        Ok(u128::from_be_bytes(bytes))
    }
}

impl Unpack for i16 {
    fn unpack_from(reader: &mut impl io::Read) -> io::Result<Self> {
        let mut bytes = [0x00; 2];
        reader.read_exact(&mut bytes)?;
        Ok(i16::from_be_bytes(bytes))
    }
}

impl Unpack for i32 {
    fn unpack_from(reader: &mut impl io::Read) -> io::Result<Self> {
        let mut bytes = [0x00; 4];
        reader.read_exact(&mut bytes)?;
        Ok(i32::from_be_bytes(bytes))
    }
}

impl Unpack for i64 {
    fn unpack_from(reader: &mut impl io::Read) -> io::Result<Self> {
        let mut bytes = [0x00; 8];
        reader.read_exact(&mut bytes)?;
        Ok(i64::from_be_bytes(bytes))
    }
}

impl Unpack for i128 {
    fn unpack_from(reader: &mut impl io::Read) -> io::Result<Self> {
        let mut bytes = [0x00; 16];
        reader.read_exact(&mut bytes)?;
        Ok(i128::from_be_bytes(bytes))
    }
}

impl Unpack for f32 {
    fn unpack_from(reader: &mut impl io::Read) -> io::Result<Self> {
        let mut bytes = [0x00; 4];
        reader.read_exact(&mut bytes)?;
        Ok(f32::from_be_bytes(bytes))
    }
}

impl Unpack for f64 {
    fn unpack_from(reader: &mut impl io::Read) -> io::Result<Self> {
        let mut bytes = [0x00; 8];
        reader.read_exact(&mut bytes)?;
        Ok(f64::from_be_bytes(bytes))
    }
}

impl Unpack for String {
    fn unpack_from(reader: &mut impl io::Read) -> io::Result<Self> {
        let mut len = u32::unpack_from(reader)? as usize;
        let mut bytes = Vec::with_capacity(len);
        let mut buffer = [0x00; 128];

        while len > 0 {
            let _read = reader.read(&mut buffer)?;

            len = if len > buffer.len() {
                bytes.extend_from_slice(&buffer);
                len - buffer.len()
            } else {
                bytes.extend(buffer.iter().take(len));
                0
            }
        }

        Ok(String::from_utf8(bytes).unwrap())
    }
}

impl<T: Unpack> Unpack for Vec<T> {
    fn unpack_from(mut reader: &mut impl io::Read) -> io::Result<Self> {
        let len = u32::unpack_from(reader)? as usize;
        let mut result = Vec::with_capacity(len);

        for _i in 0..len {
            result.push(T::unpack_from(&mut reader)?);
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unpack_bool() {
        let bytes: [u8; 1] = [0xFF];
        let value = bool::unpack_from_vec(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, false);
    }

    #[test]
    fn unpack_u8() {
        let bytes = [0xFF];
        let value = u8::unpack_from_vec(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, 255);
    }

    #[test]
    fn unpack_u16() {
        let bytes = [0x00, 0x02];
        let value = u16::unpack_from_vec(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, 2);
    }

    #[test]
    fn unpack_u32() {
        let bytes = [0x00, 0x00, 0x00, 0x02];
        let value = u32::unpack_from_vec(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, 2);
    }

    #[test]
    fn unpack_u64() {
        let bytes = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02];
        let value = u64::unpack_from_vec(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, 2);
    }

    #[test]
    fn unpack_u128() {
        let bytes = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x02,
        ];
        let value = u128::unpack_from_vec(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, 2);
    }

    #[test]
    fn unpack_i16() {
        let bytes = [0xFF, 0xFF];
        let value = i16::unpack_from_vec(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, -1);
    }

    #[test]
    fn unpack_i32() {
        let bytes = [0xFF, 0xFF, 0xFF, 0xFF];
        let value = i32::unpack_from_vec(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, -1);
    }

    #[test]
    fn unpack_i64() {
        let bytes = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
        let value = i64::unpack_from_vec(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, -1);
    }

    #[test]
    fn unpack_i128() {
        let bytes = [
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF,
        ];
        let value = i128::unpack_from_vec(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, -1);
    }

    #[test]
    fn unpack_f32() {
        let bytes = [0xBF, 0x80, 0x00, 0x00];
        let value = f32::unpack_from_vec(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, -1.0);
    }

    #[test]
    fn unpack_f64() {
        let bytes = [0xBF, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let value = f64::unpack_from_vec(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, -1.0);
    }

    #[test]
    fn unpack_str() {
        let bytes = [0x00, 0x00, 0x00, 0x03, 0x61, 0x62, 0x63];
        let value = String::unpack_from_vec(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, "abc");
    }

    #[test]
    fn unpack_array() {
        type Array = Vec<u8>;
        let bytes = [0x00, 0x00, 0x00, 0x03, 0x01, 0x02, 0x03];
        let value = Array::unpack_from_vec(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, [1, 2, 3]);
    }
}
