pub mod ftdi;
pub mod test;

pub enum Commands {
    VersionRequest = 0x80,
}

pub trait R64Driver<'a> {
    type Error;
    fn send_cmd(&'a self, cmd_id: Commands, args: &[u32]) -> Result<&[u32], Self::Error>;
}

pub struct R64Drive<'a, T: R64Driver<'a>> {
    driver: &'a T,
}

impl<'a, T: R64Driver<'a>> R64Drive<'a, T> {
    pub fn new(driver: &'a T) -> R64Drive<T> {
        R64Drive { driver }
    }

    pub fn get_version(&'a self) -> Result<&[u32], T::Error> {
        Ok(&[])
    }
}
