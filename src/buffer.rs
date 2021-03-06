use core::convert::From;
use core::ops::{Deref, DerefMut};

use std::io::{self, Read, Write};

/// A byte buffer which is versed in both the Greater and Lesser arts
///
/// Convenient for grabbing all the data from a file, and then using `Pread`/`Pwrite`, etc., on it. Only present when `std` feature is used (the default).
///
/// # Example
/// ```rust
/// use scroll::{ctx, Pread, Pwrite, Buffer};
/// let hello_world: &str = "hello world";
/// let mut buffer = Buffer::with(0x0u8, hello_world.len());
/// buffer.pwrite(hello_world, 0).unwrap();
/// let hello = buffer.pread_with::<&str>(0, ctx::SPACE).unwrap();
/// assert_eq!(hello, "hello");
/// ```

#[derive(Default, Debug)]
pub struct Buffer {
    inner: Vec<u8>
}

impl Buffer {
    /// Creates a new buffer from `bytes`
    /// # Example
    /// ```rust
    /// use scroll::Buffer;
    /// let bytes: [u8; 2] = [0x48, 0x49];
    /// let buffer = Buffer::new(bytes);
    pub fn new<T: AsRef<[u8]>> (bytes: T) -> Self {
        Buffer { inner: Vec::from(bytes.as_ref()) }
    }
    /// Initializes a new buffer with `seed`, `size` times
    /// # Example
    /// ```rust
    /// use scroll::Buffer;
    /// let buffer = Buffer::with(0x0u8, 10);
    pub fn with (seed: u8, size: usize) -> Self {
        Buffer { inner: vec![seed; size] }
    }
    /// Tries to suck the bytes out from `R` and create a new `Buffer` from it.
    /// # Example
    /// ```rust
    /// use scroll::Buffer;
    /// use std::io::Cursor;
    /// let bytes: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    /// // this could be a `File` also
    /// let cursor = Cursor::new(bytes);
    /// let buffer = Buffer::try_from(cursor).unwrap();
    pub fn try_from<R: Read> (mut file: R) -> io::Result<Buffer> {
        let mut inner = Vec::new();
        file.read_to_end(&mut inner)?;
        Ok(Buffer { inner: inner })
    }
    pub fn as_slice (&self) -> &[u8] {
        self.inner.as_slice()
    }
    /// Consumes self and returns the inner byte vector
    pub fn into_inner(self) -> Vec<u8> {
        self.inner
    }
}

// these gets us Pread, Pwrite, Gread, Gwrite, Greadable... abstraction ftw
impl AsRef<[u8]> for Buffer {
    fn as_ref (&self) -> &[u8] {
        self.inner.as_slice()
    }
}

impl AsMut<[u8]> for Buffer {
    fn as_mut (&mut self) -> &mut [u8] {
        self.inner.as_mut_slice()
    }
}

// can't impl because without specialization (i think) because conflicts with above...
// impl<T> From<T> for Buffer where T: AsRef<[u8]> {
//     fn from(bytes: T) -> Buffer {
//         Buffer { inner: Vec::from(bytes.as_ref()) }
//     }
// }

impl Deref for Buffer {
    type Target = [u8];
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Buffer {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

// this (will) gets us Lread
impl Read for Buffer {
    fn read (&mut self, buf: &mut [u8]) -> io::Result<usize> {
        Read::read(&mut self.inner.as_slice(), buf)
    }
}

// this gets us Lwrite
impl Write for Buffer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Write::write(&mut self.inner.as_mut_slice(), buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        Write::flush(&mut self.inner.as_mut_slice())
    }
}
