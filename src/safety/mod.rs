// LINT: APIs in this module are pervasive and the set of features that require an item can be
//       volatile and difficult to determine. Dead code is allowed instead, though it requires more
//       careful auditing. Items with a more obvious or simple set of dependent features are
//       annotated with `cfg` or `cfg_attr` when possible. Contrast `OptionExt` with `ArrayVecExt`,
//       for example.
#![allow(dead_code)]

// Checked implementation of extension traits that fails or panics in error conditions.
//
// This implementation is used in test builds, but not Miri builds.
#[cfg(all(not(miri), test))]
#[path = "checked.rs"]
mod maybe;
// Unchecked implementation of extension traits that ignores error conditions and so has undefined
// behavior if such a condition occurs.
//
// This implementation is used in non-test builds and Miri builds.
#[cfg(not(all(not(miri), test)))]
#[path = "unchecked.rs"]
mod maybe;

use core::slice::SliceIndex;

// TODO: At time of writing, traits cannot expose `const` functions. Remove this in favor of
//       extension traits when this is possible.
// LINT: Some of these functions are unused depending on which features are enabled. The set of
//       features may be complicated, so this module prefers `allow` over `cfg_attr` and `expect`.
#[allow(unused_imports)]
pub use maybe::{
    non_zero_from_usize_maybe_unchecked, unreachable_maybe_unchecked, unwrap_option_maybe_unchecked,
};

#[cfg(feature = "arrayvec")]
pub trait ArrayVecExt<T> {
    /// # Safety
    ///
    /// `self` must have non-zero vacancy (length must be less than capacity).
    unsafe fn push_maybe_unchecked(&mut self, item: T);
}

pub trait NonZeroExt<T> {
    /// # Safety
    ///
    /// `n` must be non-zero.
    unsafe fn new_maybe_unchecked(n: T) -> Self;
}

pub trait OptionExt<T> {
    /// # Safety
    ///
    /// The `Option` must be [`Some`]
    ///
    /// [`Some`]: core::option::Option::Some
    unsafe fn unwrap_maybe_unchecked(self) -> T;
}

pub trait ResultExt<T, E> {
    /// # Safety
    ///
    /// The `Result` must be [`Ok`]
    ///
    /// [`Ok`]: core::result::Result::Ok
    unsafe fn unwrap_maybe_unchecked(self) -> T;
}

pub trait SliceExt<T> {
    /// # Safety
    ///
    /// `index` must be within the bounds of the slice.
    unsafe fn get_maybe_unchecked<I>(&self, index: I) -> &<I as SliceIndex<[T]>>::Output
    where
        I: SliceIndex<[T]>;

    /// # Safety
    ///
    /// `index` must be within the bounds of the slice.
    unsafe fn get_maybe_unchecked_mut<I>(
        &mut self,
        index: I,
    ) -> &mut <I as SliceIndex<[T]>>::Output
    where
        I: SliceIndex<[T]>;
}
