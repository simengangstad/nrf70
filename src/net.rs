#[cfg(feature = "defmt")]
use defmt::Formatter;

use crate::Error;

pub(crate) mod eth;

/// The network buffer is structured like this:
///
/// +--------------+
/// |   Head room  |
/// |--------------|
/// |              |
/// |     Data     |
/// |              |
/// +--------------+
///
/// The whole space is given by a raw buffer, but the user of the network
/// buffer only sees the data area.
///
/// The behind this is to have easy manipulation of the head room for varying
/// headers without copying data.
pub struct NetworkBuffer<'a> {
    /// The underlying raw buffer
    raw_buffer: &'a [u8],

    /// The size of the (free) area in the start of the raw buffer.
    head_room_size: usize,

    /// The data which the user sees, points to the area after the headroom.
    data: &'a [u8],

    /// The size of the data .
    data_size: usize,
}

#[allow(dead_code)]
impl<'a> NetworkBuffer<'a> {
    pub fn new(raw_buffer: &'a [u8], size: usize) -> Self {
        NetworkBuffer {
            raw_buffer,
            data: &raw_buffer[..size],
            head_room_size: 0,
            data_size: size,
        }
    }

    pub fn get_data(&self) -> &'a [u8] {
        return self.data;
    }

    pub fn get(&self, index: usize) -> u8 {
        return self.data[index];
    }

    /// Increase the data area of a network buffer at the start of the area.
    pub fn increase_data_area(&mut self, size: usize) -> Result<(), Error> {
        if self.data_size + size > self.raw_buffer.len() {
            return Err(Error::BufferOverflow);
        }

        self.head_room_size -= size;
        self.data_size += size;

        self.data = &self.raw_buffer[self.head_room_size..self.head_room_size + self.data_size];

        Ok(())
    }

    /// Increase the head room (and thus decrease the data area).
    pub fn increase_head_room(&mut self, size: usize) -> Result<(), Error> {
        if self.head_room_size + size > self.raw_buffer.len() {
            return Err(Error::BufferOverflow);
        }

        self.head_room_size += size;
        self.data_size -= size;

        self.data = &self.raw_buffer[self.head_room_size..self.head_room_size + self.data_size];

        Ok(())
    }
}

#[cfg(feature = "defmt")]
impl<'a> defmt::Format for NetworkBuffer<'a> {
    fn format(&self, fmt: Formatter) {
        defmt::write!(fmt, "Head room {}. Data size: {}", self.head_room_size, self.data_size)
    }
}
