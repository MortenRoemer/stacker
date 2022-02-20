use std::io;

pub trait Pack {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize>;

    fn pack_to_vec(&self) -> io::Result<Vec<u8>> {
        let mut buffer = Vec::new();
        self.pack_into(&mut buffer)?;
        Ok(buffer)
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
        let buffer = self.to_le_bytes();
        writer.write(&buffer)
    }
}

impl Pack for u32 {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let buffer = self.to_le_bytes();
        writer.write(&buffer)
    }
}

impl Pack for u64 {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let buffer = self.to_le_bytes();
        writer.write(&buffer)
    }
}

impl Pack for u128 {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let buffer = self.to_le_bytes();
        writer.write(&buffer)
    }
}

impl Pack for i16 {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let buffer = self.to_le_bytes();
        writer.write(&buffer)
    }
}

impl Pack for i32 {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let buffer = self.to_le_bytes();
        writer.write(&buffer)
    }
}

impl Pack for i64 {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let buffer = self.to_le_bytes();
        writer.write(&buffer)
    }
}

impl Pack for i128 {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let buffer = self.to_le_bytes();
        writer.write(&buffer)
    }
}

impl Pack for str {
    fn pack_into(&self, writer: &mut impl io::Write) -> io::Result<usize> {
        let buffer = self.as_bytes();
        let len = buffer.len() as u32;
        let written = len.pack_into(writer)?;
        writer.write(&buffer).map(|x| written + x)
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