use std::io;
use std::num::*;

/// Describes the ability to serialize this struct into a sequential
/// bytestream
///
/// It is not possible to derive this trait, because implementors
/// should pay close attention to the order in which they serialize
/// the attributes of their structs
///
/// A derived trait would make assumptions
/// about their order and this would break compatibility with
/// deserializing those structs later
pub trait Pack {
    /// Tries to serialize this struct into a bytestream
    ///
    /// Serialization may fail because of any IO-Error
    /// (except of the ErrorKind::Interrupted which are ignored)
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize>;

    /// Tries to serialize this struct into a byte-vector
    ///
    /// Serialization may fail because of any IO-Error
    /// (except of the ErrorKind::Interrupted which are ignored)
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

impl Pack for NonZeroU8 {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let buffer = [self.get()];
        writer.write(&buffer)
    }
}

impl Pack for u16 {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let buffer = self.to_be_bytes();
        writer.write(&buffer)
    }
}

impl Pack for NonZeroU16 {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let buffer = self.get().to_be_bytes();
        writer.write(&buffer)
    }
}

impl Pack for u32 {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let buffer = self.to_be_bytes();
        writer.write(&buffer)
    }
}

impl Pack for NonZeroU32 {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let buffer = self.get().to_be_bytes();
        writer.write(&buffer)
    }
}

impl Pack for u64 {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let buffer = self.to_be_bytes();
        writer.write(&buffer)
    }
}

impl Pack for NonZeroU64 {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let buffer = self.get().to_be_bytes();
        writer.write(&buffer)
    }
}

impl Pack for u128 {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let buffer = self.to_be_bytes();
        writer.write(&buffer)
    }
}

impl Pack for NonZeroU128 {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let buffer = self.get().to_be_bytes();
        writer.write(&buffer)
    }
}

impl Pack for i16 {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let buffer = self.to_be_bytes();
        writer.write(&buffer)
    }
}

impl Pack for NonZeroI16 {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let buffer = self.get().to_be_bytes();
        writer.write(&buffer)
    }
}

impl Pack for i32 {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let buffer = self.to_be_bytes();
        writer.write(&buffer)
    }
}

impl Pack for NonZeroI32 {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let buffer = self.get().to_be_bytes();
        writer.write(&buffer)
    }
}

impl Pack for i64 {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let buffer = self.to_be_bytes();
        writer.write(&buffer)
    }
}

impl Pack for NonZeroI64 {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let buffer = self.get().to_be_bytes();
        writer.write(&buffer)
    }
}

impl Pack for i128 {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let buffer = self.to_be_bytes();
        writer.write(&buffer)
    }
}

impl Pack for NonZeroI128 {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let buffer = self.get().to_be_bytes();
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

impl<T: Pack> Pack for dyn AsRef<T> {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let value = self.as_ref();
        value.pack_into(writer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

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
    fn pack_non_zero_u8() {
        let value = NonZeroU8::new(2).unwrap();
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
    fn pack_non_zero_u16() {
        let value = NonZeroU16::new(2).unwrap();
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
    fn pack_non_zero_u32() {
        let value = NonZeroU32::new(2).unwrap();
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
    fn pack_non_zero_u64() {
        let value = NonZeroU64::new(2).unwrap();
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
    fn pack_non_zero_u128() {
        let value = NonZeroU128::new(2).unwrap();
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
    fn pack_non_zero_i16() {
        let value = NonZeroI16::new(-1).unwrap();
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
    fn pack_non_zero_i32() {
        let value = NonZeroI32::new(-1).unwrap();
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
    fn pack_non_zero_i64() {
        let value = NonZeroI64::new(-1).unwrap();
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
    fn pack_non_zero_i128() {
        let value = NonZeroI128::new(-1).unwrap();
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

    #[test]
    fn pack_array_pointer() {
        let value: Rc<[u8; 3]> = Rc::new([1, 2, 3]);
        let bytes = value.pack_to_vec().unwrap();
        assert_eq!(bytes, [0x00, 0x00, 0x00, 0x03, 0x01, 0x02, 0x03]);
    }
}
