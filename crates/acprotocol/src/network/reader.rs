/// Binary data reader for parsing network packets
pub struct BinaryReader<'a> {
    data: &'a [u8],
    position: usize,
}

impl<'a> BinaryReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, position: 0 }
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn set_position(&mut self, pos: usize) {
        self.position = pos;
    }

    pub fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.position)
    }

    pub fn read_u8(&mut self) -> std::io::Result<u8> {
        if self.position >= self.data.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Not enough data",
            ));
        }
        let val = self.data[self.position];
        self.position += 1;
        Ok(val)
    }

    pub fn read_u16(&mut self) -> std::io::Result<u16> {
        if self.position + 2 > self.data.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Not enough data",
            ));
        }
        let val = u16::from_le_bytes([self.data[self.position], self.data[self.position + 1]]);
        self.position += 2;
        Ok(val)
    }

    pub fn read_u32(&mut self) -> std::io::Result<u32> {
        if self.position + 4 > self.data.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Not enough data",
            ));
        }
        let val = u32::from_le_bytes([
            self.data[self.position],
            self.data[self.position + 1],
            self.data[self.position + 2],
            self.data[self.position + 3],
        ]);
        self.position += 4;
        Ok(val)
    }

    pub fn read_bytes(&mut self, len: usize) -> std::io::Result<Vec<u8>> {
        if self.position + len > self.data.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Not enough data",
            ));
        }
        let result = self.data[self.position..self.position + len].to_vec();
        self.position += len;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_reader_u8() {
        let data = vec![0x42, 0x13, 0xFF];
        let mut reader = BinaryReader::new(&data);

        assert_eq!(reader.read_u8().unwrap(), 0x42);
        assert_eq!(reader.position(), 1);
        assert_eq!(reader.remaining(), 2);

        assert_eq!(reader.read_u8().unwrap(), 0x13);
        assert_eq!(reader.read_u8().unwrap(), 0xFF);

        // Should error on EOF
        assert!(reader.read_u8().is_err());
    }

    #[test]
    fn test_binary_reader_u16() {
        let data = vec![0x34, 0x12, 0xCD, 0xAB];
        let mut reader = BinaryReader::new(&data);

        // Little-endian: 0x1234
        assert_eq!(reader.read_u16().unwrap(), 0x1234);
        assert_eq!(reader.position(), 2);

        // Little-endian: 0xABCD
        assert_eq!(reader.read_u16().unwrap(), 0xABCD);

        // Should error on EOF
        assert!(reader.read_u16().is_err());
    }

    #[test]
    fn test_binary_reader_u32() {
        let data = vec![0x78, 0x56, 0x34, 0x12, 0xFF, 0xFF, 0xFF, 0xFF];
        let mut reader = BinaryReader::new(&data);

        // Little-endian: 0x12345678
        assert_eq!(reader.read_u32().unwrap(), 0x12345678);
        assert_eq!(reader.position(), 4);

        assert_eq!(reader.read_u32().unwrap(), 0xFFFFFFFF);

        // Should error on EOF
        assert!(reader.read_u32().is_err());
    }

    #[test]
    fn test_binary_reader_bytes() {
        let data = vec![0x01, 0x02, 0x03, 0x04, 0x05];
        let mut reader = BinaryReader::new(&data);

        let bytes = reader.read_bytes(3).unwrap();
        assert_eq!(bytes, vec![0x01, 0x02, 0x03]);
        assert_eq!(reader.position(), 3);
        assert_eq!(reader.remaining(), 2);

        let bytes = reader.read_bytes(2).unwrap();
        assert_eq!(bytes, vec![0x04, 0x05]);

        // Should error on EOF
        assert!(reader.read_bytes(1).is_err());
    }

    #[test]
    fn test_binary_reader_set_position() {
        let data = vec![0x01, 0x02, 0x03, 0x04];
        let mut reader = BinaryReader::new(&data);

        reader.set_position(2);
        assert_eq!(reader.position(), 2);
        assert_eq!(reader.read_u8().unwrap(), 0x03);
    }

    #[test]
    fn test_binary_reader_remaining() {
        let data = vec![0x01, 0x02, 0x03];
        let mut reader = BinaryReader::new(&data);

        assert_eq!(reader.remaining(), 3);
        reader.read_u8().unwrap();
        assert_eq!(reader.remaining(), 2);
        reader.read_u16().unwrap();
        assert_eq!(reader.remaining(), 0);
    }
}
