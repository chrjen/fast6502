use core::ops::{Index, IndexMut};

use super::Memory;

pub struct Ram<'a> {
    buf: &'a mut [u8],
}

impl Ram<'_> {
    pub fn new<'a, T>(buf: &'a mut T) -> Ram<'a>
    where
        T: ?Sized,
        T: AsMut<[u8]>,
    {
        Ram { buf: buf.as_mut() }
    }
}

impl<'a> Memory for Ram<'a> {
    #[inline]
    fn get(&self, addr: u16) -> Option<u8> {
        self.buf.get(usize::from(addr)).copied()
    }

    #[inline]
    fn set(&mut self, addr: u16, value: u8) {
        if let Some(v) = self.buf.get_mut(usize::from(addr)) {
            *v = value
        }
    }
}

impl Index<u16> for Ram<'_> {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        self.buf.index(usize::from(index))
    }
}

impl IndexMut<u16> for Ram<'_> {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        self.buf.index_mut(usize::from(index))
    }
}

impl AsRef<[u8]> for Ram<'_> {
    fn as_ref(&self) -> &[u8] {
        self.buf
    }
}

impl AsMut<[u8]> for Ram<'_> {
    fn as_mut(&mut self) -> &mut [u8] {
        self.buf
    }
}
