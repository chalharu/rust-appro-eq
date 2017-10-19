//! # Licensing
//! This Source Code is subject to the terms of the Mozilla Public License
//! version 2.0 (the "License"). You can obtain a copy of the License at
//! http://mozilla.org/MPL/2.0/.

use ndarray::{ArrayBase, Axis, Data, Dimension};
use AbsError;
use RelError;
use ApproEqResult;
use ApproEqError;

fn max<D: PartialOrd, T: Iterator<Item = ApproEqResult<D>>>(
    iter: T,
) -> ApproEqResult<D> {
    iter.fold(Ok(None), move |m, i| if match (&m, &i) {
        (&Err(_), _) => false,
        (_, &Err(_)) => true,
        (&Ok(ref m), &Ok(ref i)) => {
            match (m, i) {
                (&None, _) => true,
                (_, &None) => false,
                (&Some(ref m), &Some(ref i)) => i > m,
            }
        }
    }
    {
        i
    } else {
        m
    })
}

#[cfg_attr(feature = "docs", stable(feature = "ndarray", since = "0.1.0"))]
impl<A: Data, B: PartialOrd, C: Data, D: Dimension> AbsError<ArrayBase<A, D>, B> for ArrayBase<C, D>
where
    C::Elem: AbsError<A::Elem, B> + Sized,
{
    fn abs_error(&self, expected: &ArrayBase<A, D>) -> ApproEqResult<B> {
        if self.ndim() != expected.ndim() {
            return Err(ApproEqError::LengthMismatch);
        }
        for n in 0..self.ndim() {
            if self.len_of(Axis(n)) != expected.len_of(Axis(n)) {
                return Err(ApproEqError::LengthMismatch);
            }
        }

        max(self.iter().zip(expected.iter()).map(move |(i, j)| i.abs_error(j)))
    }
}

#[cfg_attr(feature = "docs", stable(feature = "ndarray", since = "0.1.0"))]
impl<A: Data, B: PartialOrd, C: Data, D: Dimension> RelError<ArrayBase<A, D>, B> for ArrayBase<C, D>
where
    C::Elem: RelError<A::Elem, B> + Sized,
{
    fn rel_error(&self, expected: &ArrayBase<A, D>) -> ApproEqResult<B> {
        if self.ndim() != expected.ndim() {
            return Err(ApproEqError::LengthMismatch);
        }
        for n in 0..self.ndim() {
            if self.len_of(Axis(n)) != expected.len_of(Axis(n)) {
                return Err(ApproEqError::LengthMismatch);
            }
        }

        max(self.iter().zip(expected.iter()).map(move |(i, j)| i.rel_error(j)))
    }
}