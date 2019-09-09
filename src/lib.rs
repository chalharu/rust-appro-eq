//! Trait for approximately equality comparisons.
//!
//! # Overview
//! Implementing the `ApproEq` traits, Can asserts that the two expressions are approximately equal to each other.
//!
//! # Licensing
//! This Source Code is subject to the terms of the Mozilla Public License
//! version 2.0 (the "License"). You can obtain a copy of the License at
//! http://mozilla.org/MPL/2.0/.
//!
//! # Examples
//!
//! ```rust
//! # #[macro_use] extern crate appro_eq;
//! # fn main() {
//! assert_appro_eq!(1f64, 1.5f64, 0.6f64); // does not panic
//! assert_appro_eq!(0f64, 1e-12f64); // does not panic
//! assert_appro_eq!(vec![0f64, 1.0, 0.0], vec![1e-12f64, 1.0, -1e-13f64]); // does not panic
//! # }
//! ```
//! ```should_panic
//! # #[macro_use] extern crate appro_eq;
//! # fn main() {
//! assert_appro_eq!(1f64, 2f64); // panics
//! # }
//! ```

#![cfg_attr(feature = "docs", feature(staged_api))]
#![cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]

#[macro_use]
mod assert;

#[cfg(feature = "num-complex")]
mod complex_impl;

#[cfg(feature = "num-traits")]
#[cfg(feature = "num-rational")]
#[cfg(feature = "num-integer")]
mod rational_impl;

#[cfg(feature = "ndarray")]
mod ndarray_impl;

use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};

use std::error;
use std::fmt;

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
type ApproEqResult<D> = Result<Option<D>, ApproEqError>;

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
#[derive(Debug)]
pub enum ApproEqError {
    #[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
    LengthMismatch,
    #[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
    NonNumDifference,
    #[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
    DividedByZero,
    #[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
    Overflow,
    #[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
    ComponentError(
        #[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
        Box<dyn error::Error>,
    ),
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl fmt::Display for ApproEqError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", error::Error::description(self))
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl error::Error for ApproEqError {
    fn description(&self) -> &str {
        match self {
            ApproEqError::LengthMismatch => "length mismatch",
            ApproEqError::NonNumDifference => "non num difference",
            ApproEqError::DividedByZero => "divided by zero",
            ApproEqError::Overflow => "overflow",
            ApproEqError::ComponentError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
pub trait RelError<Rhs: ?Sized = Self, Diff = Self> {
    /// This method tests for self(actual value) and expected values to be relative error.
    #[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
    fn rel_error(&self, expected: &Rhs) -> ApproEqResult<Diff>;
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
pub trait AbsError<Rhs: ?Sized = Self, Diff = Self> {
    /// This method tests for self(actual value) and expected values to be absolute error.
    #[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
    fn abs_error(&self, expected: &Rhs) -> ApproEqResult<Diff>;
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
pub trait AbsTolerance<Diff = Self> {
    #[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
    fn abs_tolerance() -> Diff;
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
pub trait RelTolerance<Diff = Self> {
    #[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
    fn rel_tolerance() -> Diff;
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
pub trait Tolerance<Diff = Self> {
    #[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
    fn tolerance() -> Diff;
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl<Diff: Tolerance<Diff>> AbsTolerance<Diff> for Diff {
    fn abs_tolerance() -> Diff {
        Diff::tolerance()
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl<Diff: Tolerance<Diff>> RelTolerance<Diff> for Diff {
    fn rel_tolerance() -> Diff {
        Diff::tolerance()
    }
}

/// Trait for approximately equality comparisons.
#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
pub trait AbsApproEqWithTol<Rhs: ?Sized = Self, Diff = Self> {
    /// This method tests for approximately equal.
    #[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
    fn abs_appro_eq_with_tol(&self, other: &Rhs, tol: &Diff) -> bool;

    /// This method tests for not approximately equal.
    #[inline]
    #[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
    fn abs_appro_ne_with_tol(&self, other: &Rhs, tol: &Diff) -> bool {
        !self.abs_appro_eq_with_tol(other, tol)
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl<Rhs, Diff: PartialOrd, T: AbsError<Rhs, Diff>> AbsApproEqWithTol<Rhs, Diff> for T {
    #[inline]
    fn abs_appro_eq_with_tol(&self, other: &Rhs, tol: &Diff) -> bool {
        match self.abs_error(other) {
            Ok(ref val) => match val {
                Some(ref val) => val <= tol,
                None => true,
            },
            Err(_) => false,
        }
    }
}

/// Trait for approximately equality comparisons.
#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
pub trait AbsApproEq<Rhs: ?Sized = Self, Diff = Self> {
    /// This method tests for approximately equal.
    #[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
    fn abs_appro_eq(&self, other: &Rhs) -> bool;

    /// This method tests for not approximately equal.
    #[inline]
    #[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
    fn appro_ne_abs(&self, other: &Rhs) -> bool {
        !self.abs_appro_eq(other)
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl<Rhs, Diff: PartialOrd + AbsTolerance<Diff>, T: AbsApproEqWithTol<Rhs, Diff>>
    AbsApproEq<Rhs, Diff> for T
{
    #[inline]
    fn abs_appro_eq(&self, other: &Rhs) -> bool {
        self.abs_appro_eq_with_tol(other, &Diff::abs_tolerance())
    }
}

/// Trait for approximately equality comparisons.
#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
pub trait RelApproEqWithTol<Rhs: ?Sized = Self, Diff = Self> {
    /// This method tests for approximately equal.
    #[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
    fn rel_appro_eq_with_tol(&self, other: &Rhs, tol: &Diff) -> bool;

    /// This method tests for not approximately equal.
    #[inline]
    #[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
    fn appro_ne_rel_with_tol(&self, other: &Rhs, tol: &Diff) -> bool {
        !self.rel_appro_eq_with_tol(other, tol)
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl<Rhs, Diff: PartialOrd, T: RelError<Rhs, Diff>> RelApproEqWithTol<Rhs, Diff> for T {
    #[inline]
    fn rel_appro_eq_with_tol(&self, other: &Rhs, tol: &Diff) -> bool {
        match self.rel_error(other) {
            Ok(ref val) => match val {
                Some(ref val) => val <= tol,
                None => true,
            },
            Err(_) => false,
        }
    }
}

/// Trait for approximately equality comparisons.
#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
pub trait RelApproEq<Rhs: ?Sized = Self, Diff = Self> {
    /// This method tests for approximately equal.
    #[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
    fn rel_appro_eq(&self, other: &Rhs) -> bool;

    /// This method tests for not approximately equal.
    #[inline]
    #[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
    fn rel_appro_ne(&self, other: &Rhs) -> bool {
        !self.rel_appro_eq(other)
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl<Rhs, Diff: PartialOrd + RelTolerance<Diff>, T: RelApproEqWithTol<Rhs, Diff>>
    RelApproEq<Rhs, Diff> for T
{
    #[inline]
    fn rel_appro_eq(&self, other: &Rhs) -> bool {
        self.rel_appro_eq_with_tol(other, &Diff::rel_tolerance())
    }
}

/// tolerance is 1e-6 for f32
#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl Tolerance for f32 {
    fn tolerance() -> f32 {
        1e-6
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl AbsError for f32 {
    fn abs_error(&self, expected: &f32) -> ApproEqResult<f32> {
        Ok(Some((self - expected).abs()))
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl RelError for f32 {
    fn rel_error(&self, expected: &f32) -> ApproEqResult<f32> {
        if *expected == 0.0 {
            Err(ApproEqError::DividedByZero)
        } else {
            Ok(Some((self - expected).abs() / expected))
        }
    }
}

/// tolerance is 1e-6 for f64
#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl Tolerance for f64 {
    fn tolerance() -> f64 {
        1e-11
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl AbsError for f64 {
    fn abs_error(&self, expected: &f64) -> ApproEqResult<f64> {
        Ok(Some((self - expected).abs()))
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl RelError for f64 {
    fn rel_error(&self, expected: &f64) -> ApproEqResult<f64> {
        if *expected == 0.0 {
            Err(ApproEqError::DividedByZero)
        } else {
            Ok(Some((self - expected).abs() / expected))
        }
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl AbsError<f64, f32> for f32 {
    fn abs_error(&self, expected: &f64) -> ApproEqResult<f32> {
        Ok(Some((f64::from(*self) - expected).abs() as f32))
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl RelError<f64, f32> for f32 {
    fn rel_error(&self, expected: &f64) -> ApproEqResult<f32> {
        if *expected == 0.0 {
            Err(ApproEqError::DividedByZero)
        } else {
            Ok(Some(
                ((f64::from(*self) - expected).abs() / expected) as f32,
            ))
        }
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl AbsError<f32, f32> for f64 {
    fn abs_error(&self, expected: &f32) -> ApproEqResult<f32> {
        Ok(Some((self - f64::from(*expected)).abs() as f32))
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl RelError<f32, f32> for f64 {
    fn rel_error(&self, expected: &f32) -> ApproEqResult<f32> {
        if *expected == 0.0 {
            Err(ApproEqError::DividedByZero)
        } else {
            Ok(Some(
                ((self - f64::from(*expected)).abs() / f64::from(*expected)) as f32,
            ))
        }
    }
}

macro_rules! itype_impls {
    ($($T:ty)+) => {
        $(
            /// tolerance is zero for $T
            #[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
            impl Tolerance for $T {
                fn tolerance() -> $T {
                    0
                }
            }

            #[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
            impl AbsError for $T {
                fn abs_error(&self, expected: &$T) -> ApproEqResult<$T> {
                    Ok(Some((self - expected).abs()))
                }
            }

            #[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
            impl RelError for $T {
                fn rel_error(&self, expected: &$T) -> ApproEqResult<$T> {
                    if *expected == 0 as $T {
                        Err(ApproEqError::DividedByZero)
                    } else {
                        Ok(Some((self - expected).abs() / expected))
                    }
                }
            }
        )+
    }
}

itype_impls! { i8 i16 i32 i64 i128 }

macro_rules! utype_impls {
    ($($T:ty)+) => {
        $(
            #[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
            impl Tolerance for $T {
                fn tolerance() -> $T {
                    0
                }
            }

            #[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
            impl AbsError for $T {
                fn abs_error(&self, expected: &$T) -> ApproEqResult<$T> {
                    Ok(Some(if *self > *expected { *self - *expected } else { *expected - *self }))
                }
            }

            #[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
            impl RelError for $T {
                fn rel_error(&self, expected: &$T) -> ApproEqResult<$T> {
                    if *expected == 0 as $T {
                        Err(ApproEqError::DividedByZero)
                    } else {
                        Ok(Some((if *self > *expected { *self - *expected } else { *expected - *self }) / expected))
                    }
                }
            }
        )+
    }
}

utype_impls! { u8 u16 u32 u64 u128 }

fn max<D: PartialOrd, T: Iterator<Item = ApproEqResult<D>>>(iter: T) -> ApproEqResult<D> {
    iter.fold(Ok(None), move |m, i| {
        if match (&m, &i) {
            (&Err(_), _) => false,
            (_, &Err(_)) => true,
            (&Ok(ref m), &Ok(ref i)) => match (m, i) {
                (&None, _) => true,
                (_, &None) => false,
                (&Some(ref m), &Some(ref i)) => i > m,
            },
        } {
            i
        } else {
            m
        }
    })
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl<A, D: PartialOrd, B: AbsError<A, D>> AbsError<[A], D> for [B] {
    fn abs_error(&self, expected: &[A]) -> ApproEqResult<D> {
        if self.len() != expected.len() {
            Err(ApproEqError::LengthMismatch)
        } else {
            max((0..self.len()).map(|i| self[i].abs_error(&expected[i])))
        }
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl<A, D: PartialOrd, B: RelError<A, D>> RelError<[A], D> for [B] {
    fn rel_error(&self, expected: &[A]) -> ApproEqResult<D> {
        if self.len() != expected.len() {
            Err(ApproEqError::LengthMismatch)
        } else {
            max((0..self.len()).map(|i| self[i].rel_error(&expected[i])))
        }
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl<A, D: PartialOrd, B: AbsError<A, D>> AbsError<Vec<A>, D> for Vec<B> {
    fn abs_error(&self, expected: &Vec<A>) -> ApproEqResult<D> {
        if self.len() != expected.len() {
            Err(ApproEqError::LengthMismatch)
        } else {
            max((0..self.len()).map(|i| self[i].abs_error(&expected[i])))
        }
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl<A, D: PartialOrd, B: RelError<A, D>> RelError<Vec<A>, D> for Vec<B> {
    fn rel_error(&self, expected: &Vec<A>) -> ApproEqResult<D> {
        if self.len() != expected.len() {
            Err(ApproEqError::LengthMismatch)
        } else {
            max((0..self.len()).map(|i| self[i].rel_error(&expected[i])))
        }
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl<'a, A: ?Sized, D: PartialOrd, B: AbsError<A, D> + ?Sized> AbsError<&'a A, D> for &'a B {
    fn abs_error(&self, expected: &&A) -> ApproEqResult<D> {
        (*self).abs_error(expected)
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl<'a, A: ?Sized, D: PartialOrd, B: RelError<A, D> + ?Sized> RelError<&'a A, D> for &'a B {
    fn rel_error(&self, expected: &&A) -> ApproEqResult<D> {
        (*self).rel_error(expected)
    }
}

macro_rules! array_impls {
    ($($N:expr)+) => {
        $(
            #[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
            impl<A, D: PartialOrd, B: AbsError<A, D>> AbsError<[A; $N], D> for [B; $N] {
                fn abs_error(&self, expected: &[A; $N]) -> ApproEqResult<D>{
                    max((0..self.len()).map(|i| self[i].abs_error(&expected[i])))
                }
            }

            #[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
            impl<A, D: PartialOrd, B: RelError<A, D>> RelError<[A; $N], D> for [B; $N] {
                fn rel_error(&self, expected: &[A; $N]) -> ApproEqResult<D>{
                    max((0..self.len()).map(|i| self[i].rel_error(&expected[i])))
                }
            }
        )+
    }
}

array_impls! {
     0  1  2  3  4  5  6  7  8  9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl<A, D, B: AbsError<A, D>> AbsError<Option<A>, D> for Option<B> {
    fn abs_error(&self, expected: &Option<A>) -> ApproEqResult<D> {
        match (self, expected) {
            (&None, &None) => Ok(None),
            (&None, _) | (_, &None) => Err(ApproEqError::NonNumDifference),
            (&Some(ref x), &Some(ref y)) => x.abs_error(y),
        }
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl<A, D, B: RelError<A, D>> RelError<Option<A>, D> for Option<B> {
    fn rel_error(&self, expected: &Option<A>) -> ApproEqResult<D> {
        match (self, expected) {
            (&None, &None) => Ok(None),
            (&None, _) | (_, &None) => Err(ApproEqError::NonNumDifference),
            (&Some(ref x), &Some(ref y)) => x.rel_error(y),
        }
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl<A: ?Sized, D: PartialOrd, B: AbsError<A, D> + ?Sized> AbsError<Rc<A>, D> for Rc<B> {
    fn abs_error(&self, expected: &Rc<A>) -> ApproEqResult<D> {
        self.as_ref().abs_error(expected)
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl<A: ?Sized, D: PartialOrd, B: RelError<A, D> + ?Sized> RelError<Rc<A>, D> for Rc<B> {
    fn rel_error(&self, expected: &Rc<A>) -> ApproEqResult<D> {
        self.as_ref().rel_error(expected)
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl<A: ?Sized, D: PartialOrd, B: AbsError<A, D> + ?Sized> AbsError<Arc<A>, D> for Arc<B> {
    fn abs_error(&self, expected: &Arc<A>) -> ApproEqResult<D> {
        self.as_ref().abs_error(expected)
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl<A: ?Sized, D: PartialOrd, B: RelError<A, D> + ?Sized> RelError<Arc<A>, D> for Arc<B> {
    fn rel_error(&self, expected: &Arc<A>) -> ApproEqResult<D> {
        self.as_ref().rel_error(expected)
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl<A: ?Sized, D: PartialOrd, B: AbsError<A, D> + ?Sized> AbsError<Weak<A>, D> for Weak<B> {
    fn abs_error(&self, expected: &Weak<A>) -> ApproEqResult<D> {
        self.upgrade().abs_error(&expected.upgrade())
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl<A: ?Sized, D: PartialOrd, B: RelError<A, D> + ?Sized> RelError<Weak<A>, D> for Weak<B> {
    fn rel_error(&self, expected: &Weak<A>) -> ApproEqResult<D> {
        self.upgrade().rel_error(&expected.upgrade())
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl<A: Copy + ?Sized, D: PartialOrd, B: AbsError<A, D> + Copy + ?Sized> AbsError<Cell<A>, D>
    for Cell<B>
{
    fn abs_error(&self, expected: &Cell<A>) -> ApproEqResult<D> {
        (*self).get().abs_error(&(*expected).get())
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl<A: Copy + ?Sized, D: PartialOrd, B: RelError<A, D> + Copy + ?Sized> RelError<Cell<A>, D>
    for Cell<B>
{
    fn rel_error(&self, expected: &Cell<A>) -> ApproEqResult<D> {
        (*self).get().rel_error(&(*expected).get())
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl<A: ?Sized, D: PartialOrd, B: AbsError<A, D> + ?Sized> AbsError<RefCell<A>, D> for RefCell<B> {
    fn abs_error(&self, expected: &RefCell<A>) -> ApproEqResult<D> {
        (*self).borrow().abs_error(&(*expected).borrow())
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl<A: ?Sized, D: PartialOrd, B: RelError<A, D> + ?Sized> RelError<RefCell<A>, D> for RefCell<B> {
    fn rel_error(&self, expected: &RefCell<A>) -> ApproEqResult<D> {
        (*self).borrow().rel_error(&(*expected).borrow())
    }
}

/// absolute tolerance is 1s for Duration
#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl AbsTolerance for Duration {
    fn abs_tolerance() -> Duration {
        Duration::new(1, 0) // 1s
    }
}

/// relative tolerance is 1ms(1/1000) for Duration
#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl RelTolerance for Duration {
    fn rel_tolerance() -> Duration {
        Duration::new(0, 1_000_000) // 1ms (1/1000)
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl AbsError<Instant, Duration> for Instant {
    fn abs_error(&self, expected: &Instant) -> ApproEqResult<Duration> {
        Ok(Some(if *self > *expected {
            *self - *expected
        } else {
            *expected - *self
        }))
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl AbsError<SystemTime, Duration> for SystemTime {
    fn abs_error(&self, expected: &SystemTime) -> ApproEqResult<Duration> {
        match if *self > *expected {
            self.duration_since(*expected)
        } else {
            expected.duration_since(*self)
        } {
            Ok(v) => Ok(Some(v)),
            Err(e) => Err(ApproEqError::ComponentError(
                Box::new(e) as Box<dyn error::Error>
            )),
        }
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl AbsError for Duration {
    fn abs_error(&self, expected: &Duration) -> ApproEqResult<Duration> {
        Ok(Some(if *self > *expected {
            *self - *expected
        } else {
            *expected - *self
        }))
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl RelError for Duration {
    fn rel_error(&self, expected: &Duration) -> ApproEqResult<Duration> {
        if *expected == Duration::new(0, 0) || expected.as_secs() > u64::from(std::u32::MAX) {
            Err(ApproEqError::DividedByZero)
        } else {
            Ok(Some(
                (if *self > *expected {
                    *self - *expected
                } else {
                    *expected - *self
                }) / expected.as_secs() as u32,
            ))
        }
    }
}
