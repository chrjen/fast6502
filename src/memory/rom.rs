use core::ops::Index;

use super::Memory;

pub struct Rom<'a> {
    buf: &'a [u8],
}

impl Rom<'_> {
    pub fn new<'a, T>(buf: &'a T) -> Rom<'a>
    where
        T: ?Sized,
        T: AsRef<[u8]>,
    {
        Rom { buf: buf.as_ref() }
    }
}

impl<'a> Memory for Rom<'a> {
    #[inline]
    fn get(&self, addr: u16) -> Option<u8> {
        self.buf.get(usize::from(addr)).copied()
    }

    #[inline]
    fn set(&mut self, _addr: u16, _value: u8) {
        // Do nothing since this data structure is read-only.
    }

    #[inline]
    fn write(&mut self, _addr: u16, _value: u8) {
        // Do nothing since this data structure is read-only.
    }

    #[inline]
    fn set_slice(&mut self, _addr: u16, _buf: &[u8]) {
        // Do nothing since this data structure is read-only.
    }

    #[inline]
    fn write_slice(&mut self, _addr: u16, _buf: &[u8]) {
        // Do nothing since this data structure is read-only.
    }
}

impl Index<u16> for Rom<'_> {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        self.buf.index(usize::from(index))
    }
}

impl AsRef<[u8]> for Rom<'_> {
    fn as_ref(&self) -> &[u8] {
        self.buf
    }
}
