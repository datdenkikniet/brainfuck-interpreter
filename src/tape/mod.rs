#[cfg(feature = "impls")]
pub mod impls;

/// Data that can be stored on the tape
pub trait TapeData: PartialEq + Clone {
    /// `Self` that is considered to be zero
    fn zero() -> Self;
    /// Increase this data
    fn increase(&mut self);
    /// Decrease this data
    fn decrease(&mut self);
}

impl TapeData for u8 {
    fn zero() -> Self {
        const ZERO: u8 = 0;
        ZERO
    }

    fn increase(&mut self) {
        *self = self.wrapping_add(1);
    }

    fn decrease(&mut self) {
        *self = self.wrapping_sub(1);
    }
}

/// An implementation of Tape
pub trait Tape {
    /// The type of data that is contained by this tape
    type Data: TapeData;

    /// Get the data at a specific index
    ///
    /// This function should return `None` if the index is out of bounds
    fn get_data_at(&mut self, index: usize) -> Option<&Self::Data>;
    /// Get the data at a specific index, mutably
    ///
    /// This function should return `None` if the index is out of bounds
    fn get_data_at_mut(&mut self, index: usize) -> Option<&mut Self::Data>;
    /// Reset this tape
    fn reset(&mut self);
}
