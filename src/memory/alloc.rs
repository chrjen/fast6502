use alloc::{boxed::Box, vec::Vec};

use super::Memory;

extern crate alloc;

/// Requires feature `alloc`.
impl<M: Memory + ?Sized> Memory for Box<M> {
    #[inline]
    fn get(&self, addr: u16) -> Option<u8> {
        self.as_ref().get(addr)
    }

    #[inline]
    fn set(&mut self, addr: u16, value: u8) {
        self.as_mut().set(addr, value)
    }
}

/// Requires feature `alloc`.
impl Memory for Box<[u8]> {
    #[inline]
    fn get(&self, addr: u16) -> Option<u8> {
        <[u8]>::get(self, usize::from(addr)).copied()
    }

    #[inline]
    fn set(&mut self, addr: u16, value: u8) {
        if let Some(v) = <[u8]>::get_mut(self, usize::from(addr)) {
            *v = value
        }
    }
}

/// Requires feature `alloc`.
impl Memory for Vec<u8> {
    #[inline]
    fn get(&self, addr: u16) -> Option<u8> {
        <[u8]>::get(self, usize::from(addr)).copied()
    }

    #[inline]
    fn set(&mut self, addr: u16, value: u8) {
        if let Some(v) = <[u8]>::get_mut(self, usize::from(addr)) {
            *v = value
        }
    }
}
