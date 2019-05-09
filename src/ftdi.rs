use super::*;
use byteorder::{BigEndian, ByteOrder};
use safe_ftdi as ftdi;

pub struct R64DriveFtdi<'a> {
    context: ftdi::Context,
    _dummy: std::marker::PhantomData<&'a ()>,
}

impl<'a> R64Driver<'a> for R64DriveFtdi<'a> {
    type Error = ftdi::error::Error<'a>;
    fn send_u32(&'a self, val: u32) -> Result<usize, Self::Error> {
        let mut buf = [0; 4];
        BigEndian::write_u32(&mut buf, val);
        self.context.write_data(&buf).map(|x| x as usize)
    }

    fn recv_u32(&'a self) -> Result<u32, Self::Error> {
        let mut resp = [0u8; 4];
        self.context.read_data(&mut resp)?;
        Ok(BigEndian::read_u32(&resp))
    }
}

impl<'a> R64DriveFtdi<'a> {
    pub fn new() -> R64DriveFtdi<'a> {
        // TODO take a hardware version or VID/PID
        let mut result = R64DriveFtdi {
            context: ftdi::Context::new().unwrap(),
            _dummy: std::marker::PhantomData,
        };
        result.context.open(0x0403, 0x6014).unwrap();
        result
    }
}

impl<'a> Default for R64DriveFtdi<'a> {
    fn default() -> Self {
        Self::new()
    }
}
