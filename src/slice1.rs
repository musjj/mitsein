use std::borrow::ToOwned;
use std::fmt::{self, Debug, Formatter};
use std::mem;
use std::ops::{Deref, DerefMut};
use std::slice;

use crate::iter1::Iterator1;
use crate::vec1::Vec1;
use crate::NonEmpty;

pub type Slice1<T> = NonEmpty<[T]>;

// TODO: As with `Vec1`, provide functions with stronger guarantees for slices of one or more
//       items. For example, functions that isolate terminal items like `split_first` need not be
//       fallible.
impl<T> Slice1<T> {
    pub(crate) fn from_slice_unchecked(items: &[T]) -> &Self {
        // SAFETY:
        unsafe { mem::transmute::<&'_ [T], &'_ Slice1<T>>(items) }
    }

    pub(crate) fn from_mut_slice_unchecked(items: &mut [T]) -> &mut Self {
        // SAFETY:
        unsafe { mem::transmute::<&'_ mut [T], &'_ mut Slice1<T>>(items) }
    }

    pub fn try_from_slice(items: &[T]) -> Result<&Self, &[T]> {
        match items.len() {
            0 => Err(items),
            _ => Ok(Slice1::from_slice_unchecked(items)),
        }
    }

    pub fn try_from_mut_slice(items: &mut [T]) -> Result<&mut Self, &mut [T]> {
        match items.len() {
            0 => Err(items),
            _ => Ok(Slice1::from_mut_slice_unchecked(items)),
        }
    }

    pub fn to_vec1(&self) -> Vec1<T>
    where
        T: Clone,
    {
        Vec1::from(self)
    }

    pub fn iter1(&self) -> Iterator1<slice::Iter<'_, T>> {
        Iterator1::from_iter_unchecked(self.as_slice().iter())
    }

    pub fn iter1_mut(&mut self) -> Iterator1<slice::IterMut<'_, T>> {
        Iterator1::from_iter_unchecked(self.as_mut_slice().iter_mut())
    }

    pub fn as_slice(&self) -> &'_ [T] {
        // SAFETY:
        unsafe { mem::transmute::<&'_ Slice1<T>, &'_ [T]>(self) }
    }

    pub fn as_mut_slice(&mut self) -> &'_ mut [T] {
        // SAFETY:
        unsafe { mem::transmute::<&'_ mut Slice1<T>, &'_ mut [T]>(self) }
    }
}

impl<T> AsMut<[T]> for &'_ mut Slice1<T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.items
    }
}

impl<T> AsRef<[T]> for &'_ Slice1<T> {
    fn as_ref(&self) -> &[T] {
        &self.items
    }
}

impl<T> Debug for Slice1<T>
where
    T: Debug,
{
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_list()
            .entries(self.as_slice().iter())
            .finish()
    }
}

impl<T> Deref for Slice1<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T> DerefMut for Slice1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl<T> ToOwned for Slice1<T>
where
    T: Clone,
{
    type Owned = Vec1<T>;

    fn to_owned(&self) -> Self::Owned {
        Vec1::from(self)
    }
}

#[cfg(test)]
mod tests {}
