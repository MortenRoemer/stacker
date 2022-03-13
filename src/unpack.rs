use std::error;
use std::fmt::{self, Display, Formatter};
use std::io;
use std::num::*;
use std::rc::Rc;
use std::string::FromUtf8Error;
use std::sync::Arc;

/// Describes the ability to deserialize a struct from a sequential bytesource
///
/// Any type implementing this trait has to be Sized and Owned but this contraints
/// may change in the future
///
/// It is not possible to derive this trait, because deserialization may be
/// sensitive to order and endianness. (Big endianness is assumed for all primitives)
pub trait Unpack {
    /// Tries to deserialize this struct from a given sequence of bytes
    ///
    /// Deserialization may fail for one of these reasons:
    /// - any IO-Error ocurred (ErrorKind::Interrupted is ignored)
    /// - a string contained invalid UTF8 code
    /// - a custom error previously defined occurred
    fn unpack_from(reader: &mut impl io::Read) -> Result<Self>
    where
        Self: Sized;
}

/// Error that may occur during deserialization
///
/// There are three possible reasons deserialization may fail:
/// - any IO-Error ocurred (ErrorKind::Interrupted is ignored)
/// - a string contained invalid UTF8 contained
/// - a custom error previously defined ocurred
#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    UTF8(FromUtf8Error),
    Custom(Box<dyn error::Error>),
}

impl Display for Error {
    fn fmt(&self, destination: &mut Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        use Error::*;
        match self {
            IO(error) => error.fmt(destination),
            UTF8(error) => error.fmt(destination),
            Custom(error) => error.fmt(destination),
        }
    }
}

impl error::Error for Error {}

/// Wrapper for a deserialization result
pub type Result<T> = std::result::Result<T, Error>;

impl Unpack for bool {
    fn unpack_from(reader: &mut impl io::Read) -> Result<Self> {
        let mut bytes = [0x00];
        reader.read_exact(&mut bytes).map_err(Error::IO)?;
        Ok(bytes[0] != 0xFF)
    }
}

impl Unpack for u8 {
    fn unpack_from(reader: &mut impl io::Read) -> Result<Self> {
        let mut bytes = [0x00];
        reader.read_exact(&mut bytes).map_err(Error::IO)?;
        Ok(bytes[0])
    }
}

impl Unpack for NonZeroU8 {
    fn unpack_from(reader: &mut impl io::Read) -> Result<Self> {
        let mut bytes = [0x00];
        reader.read_exact(&mut bytes).map_err(Error::IO)?;
        Ok(NonZeroU8::new(bytes[0]).unwrap())
    }
}

impl Unpack for u16 {
    fn unpack_from(reader: &mut impl io::Read) -> Result<Self> {
        let mut bytes = [0x00; 2];
        reader.read_exact(&mut bytes).map_err(Error::IO)?;
        Ok(u16::from_be_bytes(bytes))
    }
}

impl Unpack for NonZeroU16 {
    fn unpack_from(reader: &mut impl io::Read) -> Result<Self> {
        let mut bytes = [0x00; 2];
        reader.read_exact(&mut bytes).map_err(Error::IO)?;
        Ok(NonZeroU16::new(u16::from_be_bytes(bytes)).unwrap())
    }
}

impl Unpack for u32 {
    fn unpack_from(reader: &mut impl io::Read) -> Result<Self> {
        let mut bytes = [0x00; 4];
        reader.read_exact(&mut bytes).map_err(Error::IO)?;
        Ok(u32::from_be_bytes(bytes))
    }
}

impl Unpack for NonZeroU32 {
    fn unpack_from(reader: &mut impl io::Read) -> Result<Self> {
        let mut bytes = [0x00; 4];
        reader.read_exact(&mut bytes).map_err(Error::IO)?;
        Ok(NonZeroU32::new(u32::from_be_bytes(bytes)).unwrap())
    }
}

impl Unpack for u64 {
    fn unpack_from(reader: &mut impl io::Read) -> Result<Self> {
        let mut bytes = [0x00; 8];
        reader.read_exact(&mut bytes).map_err(Error::IO)?;
        Ok(u64::from_be_bytes(bytes))
    }
}

impl Unpack for NonZeroU64 {
    fn unpack_from(reader: &mut impl io::Read) -> Result<Self> {
        let mut bytes = [0x00; 8];
        reader.read_exact(&mut bytes).map_err(Error::IO)?;
        Ok(NonZeroU64::new(u64::from_be_bytes(bytes)).unwrap())
    }
}

impl Unpack for u128 {
    fn unpack_from(reader: &mut impl io::Read) -> Result<Self> {
        let mut bytes = [0x00; 16];
        reader.read_exact(&mut bytes).map_err(Error::IO)?;
        Ok(u128::from_be_bytes(bytes))
    }
}

impl Unpack for NonZeroU128 {
    fn unpack_from(reader: &mut impl io::Read) -> Result<Self> {
        let mut bytes = [0x00; 16];
        reader.read_exact(&mut bytes).map_err(Error::IO)?;
        Ok(NonZeroU128::new(u128::from_be_bytes(bytes)).unwrap())
    }
}

impl Unpack for i16 {
    fn unpack_from(reader: &mut impl io::Read) -> Result<Self> {
        let mut bytes = [0x00; 2];
        reader.read_exact(&mut bytes).map_err(Error::IO)?;
        Ok(i16::from_be_bytes(bytes))
    }
}

impl Unpack for NonZeroI16 {
    fn unpack_from(reader: &mut impl io::Read) -> Result<Self> {
        let mut bytes = [0x00; 2];
        reader.read_exact(&mut bytes).map_err(Error::IO)?;
        Ok(NonZeroI16::new(i16::from_be_bytes(bytes)).unwrap())
    }
}

impl Unpack for i32 {
    fn unpack_from(reader: &mut impl io::Read) -> Result<Self> {
        let mut bytes = [0x00; 4];
        reader.read_exact(&mut bytes).map_err(Error::IO)?;
        Ok(i32::from_be_bytes(bytes))
    }
}

impl Unpack for NonZeroI32 {
    fn unpack_from(reader: &mut impl io::Read) -> Result<Self> {
        let mut bytes = [0x00; 4];
        reader.read_exact(&mut bytes).map_err(Error::IO)?;
        Ok(NonZeroI32::new(i32::from_be_bytes(bytes)).unwrap())
    }
}

impl Unpack for i64 {
    fn unpack_from(reader: &mut impl io::Read) -> Result<Self> {
        let mut bytes = [0x00; 8];
        reader.read_exact(&mut bytes).map_err(Error::IO)?;
        Ok(i64::from_be_bytes(bytes))
    }
}

impl Unpack for NonZeroI64 {
    fn unpack_from(reader: &mut impl io::Read) -> Result<Self> {
        let mut bytes = [0x00; 8];
        reader.read_exact(&mut bytes).map_err(Error::IO)?;
        Ok(NonZeroI64::new(i64::from_be_bytes(bytes)).unwrap())
    }
}

impl Unpack for i128 {
    fn unpack_from(reader: &mut impl io::Read) -> Result<Self> {
        let mut bytes = [0x00; 16];
        reader.read_exact(&mut bytes).map_err(Error::IO)?;
        Ok(i128::from_be_bytes(bytes))
    }
}

impl Unpack for NonZeroI128 {
    fn unpack_from(reader: &mut impl io::Read) -> Result<Self> {
        let mut bytes = [0x00; 16];
        reader.read_exact(&mut bytes).map_err(Error::IO)?;
        Ok(NonZeroI128::new(i128::from_be_bytes(bytes)).unwrap())
    }
}

impl Unpack for f32 {
    fn unpack_from(reader: &mut impl io::Read) -> Result<Self> {
        let mut bytes = [0x00; 4];
        reader.read_exact(&mut bytes).map_err(Error::IO)?;
        Ok(f32::from_be_bytes(bytes))
    }
}

impl Unpack for f64 {
    fn unpack_from(reader: &mut impl io::Read) -> Result<Self> {
        let mut bytes = [0x00; 8];
        reader.read_exact(&mut bytes).map_err(Error::IO)?;
        Ok(f64::from_be_bytes(bytes))
    }
}

impl Unpack for String {
    fn unpack_from(reader: &mut impl io::Read) -> Result<Self> {
        let mut len = u32::unpack_from(reader)? as usize;
        let mut bytes = Vec::with_capacity(len);
        let mut buffer = [0x00; 512];

        while len > 0 {
            let _read = reader.read(&mut buffer).map_err(Error::IO)?;

            len = if len > buffer.len() {
                bytes.extend_from_slice(&buffer);
                len - buffer.len()
            } else {
                bytes.extend(buffer.iter().take(len));
                0
            }
        }

        String::from_utf8(bytes).map_err(Error::UTF8)
    }
}

impl<T: Unpack> Unpack for Vec<T> {
    fn unpack_from(mut reader: &mut impl io::Read) -> Result<Self> {
        let len = u32::unpack_from(reader)? as usize;
        let mut result = Vec::with_capacity(len);

        for _i in 0..len {
            result.push(T::unpack_from(&mut reader)?);
        }

        Ok(result)
    }
}

impl<T: Unpack> Unpack for Box<T> {
    fn unpack_from(reader: &mut impl io::Read) -> Result<Self> {
        T::unpack_from(reader).map(|x| Box::new(x))
    }
}

impl<T: Unpack> Unpack for Rc<T> {
    fn unpack_from(reader: &mut impl io::Read) -> Result<Self> {
        T::unpack_from(reader).map(|x| Rc::new(x))
    }
}

impl<T: Unpack> Unpack for Arc<T> {
    fn unpack_from(reader: &mut impl io::Read) -> Result<Self> {
        T::unpack_from(reader).map(|x| Arc::new(x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unpack_bool() {
        let bytes: [u8; 1] = [0xFF];
        let value = bool::unpack_from(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, false);
    }

    #[test]
    fn unpack_u8() {
        let bytes = [0xFF];
        let value = u8::unpack_from(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, 255);
    }

    #[test]
    fn unpack_non_zero_u8() {
        let bytes = [0xFF];
        let value = NonZeroU8::unpack_from(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, NonZeroU8::new(255).unwrap());
    }

    #[test]
    fn unpack_u16() {
        let bytes = [0x00, 0x02];
        let value = u16::unpack_from(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, 2);
    }

    #[test]
    fn unpack_non_zero_u16() {
        let bytes = [0x00, 0x02];
        let value = NonZeroU16::unpack_from(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, NonZeroU16::new(2).unwrap());
    }

    #[test]
    fn unpack_u32() {
        let bytes = [0x00, 0x00, 0x00, 0x02];
        let value = u32::unpack_from(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, 2);
    }

    #[test]
    fn unpack_non_zero_u32() {
        let bytes = [0x00, 0x00, 0x00, 0x02];
        let value = NonZeroU32::unpack_from(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, NonZeroU32::new(2).unwrap());
    }

    #[test]
    fn unpack_u64() {
        let bytes = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02];
        let value = u64::unpack_from(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, 2);
    }

    #[test]
    fn unpack_non_zero_u64() {
        let bytes = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02];
        let value = NonZeroU64::unpack_from(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, NonZeroU64::new(2).unwrap());
    }

    #[test]
    fn unpack_u128() {
        let bytes = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x02,
        ];
        let value = u128::unpack_from(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, 2);
    }

    #[test]
    fn unpack_non_zero_u128() {
        let bytes = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x02,
        ];
        let value = NonZeroU128::unpack_from(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, NonZeroU128::new(2).unwrap());
    }

    #[test]
    fn unpack_i16() {
        let bytes = [0xFF, 0xFF];
        let value = i16::unpack_from(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, -1);
    }

    #[test]
    fn unpack_non_zero_i16() {
        let bytes = [0xFF, 0xFF];
        let value = NonZeroI16::unpack_from(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, NonZeroI16::new(-1).unwrap());
    }

    #[test]
    fn unpack_i32() {
        let bytes = [0xFF, 0xFF, 0xFF, 0xFF];
        let value = i32::unpack_from(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, -1);
    }

    #[test]
    fn unpack_non_zero_i32() {
        let bytes = [0xFF, 0xFF, 0xFF, 0xFF];
        let value = NonZeroI32::unpack_from(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, NonZeroI32::new(-1).unwrap());
    }

    #[test]
    fn unpack_i64() {
        let bytes = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
        let value = i64::unpack_from(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, -1);
    }

    #[test]
    fn unpack_non_zero_i64() {
        let bytes = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
        let value = NonZeroI64::unpack_from(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, NonZeroI64::new(-1).unwrap());
    }

    #[test]
    fn unpack_i128() {
        let bytes = [
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF,
        ];
        let value = i128::unpack_from(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, -1);
    }

    #[test]
    fn unpack_non_zero_i128() {
        let bytes = [
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF,
        ];
        let value = NonZeroI128::unpack_from(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, NonZeroI128::new(-1).unwrap());
    }

    #[test]
    fn unpack_f32() {
        let bytes = [0xBF, 0x80, 0x00, 0x00];
        let value = f32::unpack_from(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, -1.0);
    }

    #[test]
    fn unpack_f64() {
        let bytes = [0xBF, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let value = f64::unpack_from(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, -1.0);
    }

    #[test]
    fn unpack_string() {
        let bytes = [0x00, 0x00, 0x00, 0x03, 0x61, 0x62, 0x63];
        let value = String::unpack_from(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, "abc");
    }

    #[test]
    fn unpack_array() {
        type Array = Vec<u8>;
        let bytes = [0x00, 0x00, 0x00, 0x03, 0x01, 0x02, 0x03];
        let value = Array::unpack_from(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, [1, 2, 3]);
    }

    #[test]
    fn unpack_box() {
        type Value = Box<u16>;
        let bytes = [0x00, 0x02];
        let value = Value::unpack_from(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, Box::new(2));
    }

    #[test]
    fn unpack_rc() {
        type Value = Rc<u16>;
        let bytes = [0x00, 0x02];
        let value = Value::unpack_from(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, Rc::new(2));
    }

    #[test]
    fn unpack_arc() {
        type Value = Arc<u16>;
        let bytes = [0x00, 0x02];
        let value = Value::unpack_from(&mut bytes.as_ref()).unwrap();
        assert_eq!(value, Arc::new(2));
    }
}
