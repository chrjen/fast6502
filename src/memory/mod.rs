#[cfg(any(feature = "alloc", doc))]
mod alloc;

use core::mem;

#[allow(dead_code)]
const _: () = if mem::size_of::<u16>() > mem::size_of::<usize>() {
    panic!("usize needs to be at least as big as u16 in size");
};

/// A data structure where bytes can be read/written at specific addresses.
///
/// There are two methods you need to implement [`get`] and [`set`]. You are
/// heavily encouraged to also implement [`read`] and [`write`].
///
/// [`get`]: Memory::get
/// [`set`]: Memory::set
/// [`read`]: Memory::read
/// [`write`]: Memory::write
pub trait Memory {
    /// Gets the value at the address. This exist to allow rust code to get
    /// values from memory without triggering any read related side effects.
    /// For actual reads from the processor see [`read`].
    ///
    /// [`read`]: Memory::read
    fn get(&self, addr: u16) -> Option<u8>;

    /// Sets the value at the address. This exist to allow rust code to set
    /// values in memory without triggering any write related side effects.
    /// For actual writes from the processor see [`write`].
    ///
    /// [`write`]: Memory::write
    fn set(&mut self, addr: u16, value: u8);

    /// Perform an actual memory read from the processor. This may trigger any
    /// read related side effects like memory-mapped I/O. To avoid triggering
    /// any memory-mapped side effects when reading look at [`get`].
    ///
    /// [`get`]: Memory::get
    #[inline]
    fn read(&mut self, addr: u16) -> u8 {
        self.get(addr).unwrap_or(0)
    }

    /// Perform an actual memory write from the processor. This may trigger any
    /// write related side effects like memory-mapped I/O. To avoid triggering
    /// any memory-mapped side effects when writing look at [`set`].
    ///
    /// [`set`]: Memory::set
    #[inline]
    fn write(&mut self, addr: u16, value: u8) {
        self.set(addr, value);
    }

    /// Same as [`get`], but for slices.
    ///
    /// [`get`]: Memory::get
    #[inline]
    fn get_slice(&mut self, addr: u16, buf: &mut [Option<u8>]) {
        for (i, value) in buf.iter_mut().enumerate() {
            let offset: u16 = i.try_into().expect("buffer length should fit inside a u16");
            *value = self.get(addr + offset);
        }
    }

    /// Same as [`set`], but for slices.
    ///
    /// [`set`]: Memory::set
    #[inline]
    fn set_slice(&mut self, addr: u16, buf: &[u8]) {
        for (i, &value) in buf.iter().enumerate() {
            let offset: u16 = i.try_into().expect("buffer length should fit inside a u16");
            self.set(addr + offset, value);
        }
    }

    /// Same as [`read`], but for slices.
    ///
    /// [`read`]: Memory::read
    #[inline]
    fn read_slice(&mut self, addr: u16, buf: &mut [u8]) {
        for (i, value) in buf.iter_mut().enumerate() {
            let offset: u16 = i.try_into().expect("buffer length should fit inside a u16");
            *value = self.read(addr + offset);
        }
    }

    /// Same as [`write`], but for slices.
    ///
    /// [`write`]: Memory::write
    #[inline]
    fn write_slice(&mut self, addr: u16, buf: &[u8]) {
        for (i, &value) in buf.iter().enumerate() {
            let offset: u16 = i.try_into().expect("buffer length fits inside a u16");
            self.write(addr + offset, value);
        }
    }
}

impl Memory for &[u8] {
    #[inline]
    fn get(&self, addr: u16) -> Option<u8> {
        <[u8]>::get(self, usize::from(addr)).copied()
    }

    #[inline]
    fn set(&mut self, _addr: u16, _value: u8) {
        // Do nothing since this data structure is read-only.
    }
}

impl Memory for &mut [u8] {
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
