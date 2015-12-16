use std::io::{Result, Write};

/// A type which duplicates its writes to the provided writers
pub struct BroadcastWriter<A: Write, B: Write> {
    primary: A,
    secondary: B,
}

impl<A: Write, B: Write> BroadcastWriter<A, B> {
    /// Creates a new BroadcastWriter instance which can be used as a Write.
    /// All data will be written to the primary writer as well as the secondary
    /// writer. Errors that occur during the either write operation will be yielded.
    pub fn new(primary: A, secondary: B) -> BroadcastWriter<A, B> {
        BroadcastWriter {
            primary: primary,
            secondary: secondary,
        }
    }
}

impl<A: Write, B: Write> Write for BroadcastWriter<A, B> {
    fn write(&mut self, data: &[u8]) -> Result<usize> {
        let n = try!(self.primary.write(data));
        try!(self.secondary.write_all(&data[..n]));
        Ok(n)
    }

    fn flush(&mut self) -> Result<()> {
        self.primary.flush().and(self.secondary.flush())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    #[test]
    fn broadcast() {
        let mut first = Vec::new();
        let mut second = Vec::new();
        {
            let mut broadcaster = BroadcastWriter::new(&mut first, &mut second);
            let _ = broadcaster.write(b"it's over 9000!");
        }
        assert_eq!(first, second);
    }
}
