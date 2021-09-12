//! Default implementations of tape and tape data

use crate::{Tape, TapeData};

impl<D> Tape for Vec<D>
where
    D: TapeData,
{
    type Data = D;

    fn get_data_at_mut(&mut self, index: usize) -> Option<&mut D> {
        if self.len() <= index {
            self.resize(index + 1, D::zero());
        }
        unsafe { Some(self.get_unchecked_mut(index)) }
    }

    fn reset(&mut self) {
        self.iter_mut().for_each(|data| *data = D::zero());
    }

    fn get_data_at(&mut self, index: usize) -> Option<&D> {
        if self.len() <= index {
            self.resize(index + 1, D::zero());
        }
        unsafe { Some(self.get_unchecked(index)) }
    }
}

impl<D, const N: usize> Tape for [D; N]
where
    D: TapeData + Copy,
{
    type Data = D;

    fn get_data_at(&mut self, index: usize) -> Option<&D> {
        if index < N {
            Some(&self[index])
        } else {
            None
        }
    }

    fn get_data_at_mut(&mut self, index: usize) -> Option<&mut D> {
        if index < N {
            Some(&mut self[index])
        } else {
            None
        }
    }

    fn reset(&mut self) {
        self.iter_mut().for_each(|val| *val = D::zero());
    }
}

impl<D> Tape for &mut [D]
where
    D: TapeData,
{
    type Data = D;

    fn get_data_at(&mut self, index: usize) -> Option<&Self::Data> {
        if index < self.len() {
            Some(&self[index])
        } else {
            None
        }
    }

    fn get_data_at_mut(&mut self, index: usize) -> Option<&mut Self::Data> {
        if index < self.len() {
            Some(&mut self[index])
        } else {
            None
        }
    }

    fn reset(&mut self) {
        self.iter_mut().for_each(|d| *d = D::zero());
    }
}
